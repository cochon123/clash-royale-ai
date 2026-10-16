"""v9 GRU policy and a separately gated approximate troop-motion placement prior."""
from __future__ import annotations
import json
from pathlib import Path
import torch
from torch import nn
from .controlled_policy import ControlledPolicy
from .feature_schema import CONTINUOUS_INDEX as C, GLOBAL_INDEX as G
from .policy_model import TILE_ROWS, TILE_COLS, NUM_TILES


def load_motion_stats(vocab, root="data/v9_card_stats"):
    root = Path(root)
    cards = json.loads((root / "cards.json").read_text())
    troops = {x["name"]: x for x in json.loads((root / "cards_stats_troop.json").read_text())}
    units = {x["name"]: x for x in json.loads((root / "cards_stats_characters.json").read_text())}
    buildings = {x["name"]: x for x in json.loads((root / "cards_stats_building.json").read_text())}
    stats = torch.zeros(vocab.vocab_size, 7)
    for card in cards:
        if card["key"] not in vocab.name_to_id or card["type"] not in {"Troop", "Building"}:
            continue
        troop = troops.get(card["sc_key"], {})
        unit = units.get(troop.get("summon_character", card["sc_key"]), buildings.get(card["sc_key"], {}))
        if not unit:
            continue
        building = card["type"] == "Building"
        life = float(unit.get("life_time") or 0) / 1000
        stats[vocab.encode(card["key"])] = torch.tensor([
            float(unit.get("speed") or 0) / 60, float(building),
            float(bool(unit.get("flying_height"))), float(bool(unit.get("target_only_buildings"))),
            life if life > 0 else (30 if building else 20),
            float(unit.get("deploy_time") or 1000) / 1000,
            max(1, int(troop.get("summon_number") or 1)),
        ])
    return stats


class GhostRasterizer(nn.Module):
    """Uncertain positions, not observed health or a combat simulator.

    Stats use RoyaleAPI's extracted client data. Speed/60 tiles per second and
    troop survival half-life are explicit hypotheses tested by a shuffled
    control. Ground units travel via a bridge; air units travel directly.
    Only the policy's last 64 confirmed events are available.
    """
    def __init__(self, stats):
        super().__init__()
        self.register_buffer("stats", stats)

    def forward(self, c, ids, g, lengths):
        stat = self.stats[ids]
        own = c[:, :, C["side_flag"]] > .5
        age = (g[:, G["now"], None]*300 - c[:, :, C["seconds"]]*300).clamp_min(0)
        valid = (torch.arange(c.size(1), device=c.device)[None] < lengths[:, None]) & (ids != 0) & (stat[:,:,4] > 0) & (c[:,:,C["is_ability"]] < .5) & (c[:,:,C["pad"]] < .5)
        x, y = c[:,:,C["x"]], c[:,:,C["y"]]
        speed = stat[:,:,0]
        building, air, wincon = stat[:,:,1] > .5, stat[:,:,2] > .5, stat[:,:,3] > .5
        left = x < .5
        target_x = torch.where(left, .22, .78)
        target_y = torch.where(own, .91, .09)
        bridge_y = torch.full_like(y, .5)
        need_bridge = ((own & (y < .5)) | (~own & (y > .5))) & ~air & ~building
        first_x = torch.where(need_bridge, target_x, target_x)
        first_y = torch.where(need_bridge, bridge_y, target_y)
        # Distances in arena tiles, not normalized-coordinate Euclidean units.
        dx, dy = (first_x-x)*18, (first_y-y)*32
        dist = (dx.square()+dy.square()).sqrt().clamp_min(1e-6)
        travel = (age-stat[:,:,5]).clamp_min(0)*speed
        ratio = (travel/dist).clamp(max=1)
        nx, ny = x + ratio*dx/18, y+ratio*dy/32
        second_dist = ((target_y-first_y).abs()*32).clamp_min(1e-6)
        second = ((travel-dist).clamp_min(0)/second_dist).clamp(max=1)
        ny = ny+second*(target_y-first_y)
        nx, ny = torch.where(building,x,nx), torch.where(building,y,ny)
        survival = torch.exp(-.693147*age/stat[:,:,4].clamp_min(1))
        survival = survival*torch.where(building, (age<stat[:,:,4]).float(), 1.)
        mass = survival*valid*stat[:,:,6].sqrt()
        channels = torch.stack([own & ~building, ~own & ~building, own & building, ~own & building, own & wincon, ~own & wincon, own & air, ~own & air], 1).float()*mass[:,None]
        fx=(nx*TILE_COLS-.5).clamp(0,TILE_COLS-1);fy=(ny*TILE_ROWS-.5).clamp(0,TILE_ROWS-1)
        x0,y0=fx.long(),fy.long();wx,wy=fx-x0,fy-y0
        result = c.new_zeros(len(c),8,NUM_TILES)
        for ox,oy,w in ((0,0,(1-wx)*(1-wy)),(1,0,wx*(1-wy)),(0,1,(1-wx)*wy),(1,1,wx*wy)):
            tile=(y0+oy).clamp(max=TILE_ROWS-1)*TILE_COLS+(x0+ox).clamp(max=TILE_COLS-1)
            result.scatter_add_(2,tile[:,None].expand(-1,8,-1),channels*w[:,None])
        return result.reshape(len(c),8,TILE_ROWS,TILE_COLS)


class V9Policy(ControlledPolicy):
    def __init__(self,vocab_size,d_model=160,clock=True,ghost="none",motion_stats=None):
        super().__init__(vocab_size,"clock" if clock else "baseline",d_model)
        if ghost not in {"none","aligned","shuffled"}:
            raise ValueError(ghost)
        self.base.padded_encoder=True
        self.ghost=ghost
        if ghost != "none":
            if motion_stats is None:
                motion_stats=torch.zeros(vocab_size,7)
            self.ghost_rasterizer=GhostRasterizer(motion_stats)
            self.ghost_cnn=nn.Sequential(nn.Conv2d(8,24,3,padding=1),nn.GELU(),nn.Conv2d(24,24,3,padding=1),nn.GELU())
            self.ghost_context=nn.Linear(d_model+48,24)
            self.ghost_gate=nn.Parameter(torch.tensor(-2.))

    def forward(self,batch,teacher=False):
        out=super().forward(batch,teacher)
        if self.ghost != "none":
            c,ids,td,_,g=batch[:5]
            raster=self.ghost_rasterizer(c,ids,g,batch[12])
            if self.ghost == "shuffled":
                raster=raster.roll(1,0)
            slots=batch[7] if teacher else out["slot_logits"].argmax(-1)
            card=self.base.card_embedding(td)[torch.arange(len(td),device=td.device),slots]
            context=self.ghost_context(torch.cat([out["fused"],card],-1))
            features=self.ghost_cnn(raster)
            residual=(features*context[:,:,None,None]).sum(1).flatten(1)/24**.5
            out["tile_logits"]=out["tile_logits"]+torch.sigmoid(self.ghost_gate)*residual
            out["xy"]=out["tile_logits"].softmax(-1)@self.base.tile_centers
        return out


def tile_loss(logits,xy,sigma=0.):
    x=(xy[:,0]*TILE_COLS).long().clamp(0,TILE_COLS-1)
    y=(xy[:,1]*TILE_ROWS).long().clamp(0,TILE_ROWS-1)
    tile=y*TILE_COLS+x
    if sigma <= 0:
        return nn.functional.cross_entropy(logits,tile,reduction="none")
    rows=torch.arange(TILE_ROWS,device=xy.device).repeat_interleave(TILE_COLS)
    cols=torch.arange(TILE_COLS,device=xy.device).repeat(TILE_ROWS)
    target=torch.exp(-((rows[None]-y[:,None]).square()+(cols[None]-x[:,None]).square())/(2*sigma*sigma))
    target=target/target.sum(-1,keepdim=True)
    return -(target*logits.log_softmax(-1)).sum(-1)


def load_v9_policy(path,device="cpu"):
    from .winner_dataset import CardVocab
    artifact=torch.load(path,map_location=device,weights_only=False)
    config=artifact["config"]
    model=V9Policy(config["vocab_size"],config["d_model"],config["clock"],config["ghost"]).to(device)
    model.load_state_dict(artifact["model_state"])
    model.eval()
    return model,CardVocab.from_dict(artifact["vocab"]),config
