"""Immutable causal fp16 observation shards, shared by every v9 experiment.

Only deployment-history features are cached. Results/winner weights are labels,
never policy inputs. One full causal stream per perspective removes quadratic
prefix re-encoding; prefix-specific hidden deck statistics are recomputed.
"""
from __future__ import annotations

import argparse
import hashlib
from concurrent.futures import ProcessPoolExecutor
from dataclasses import replace
from datetime import datetime, timezone
import json
import multiprocessing as mp
from pathlib import Path
import pickle
import resource
import time

import numpy as np
import torch
from torch.utils.data import Dataset, Sampler

from .artifact_cache import sha256_json, sha256_file, source_fingerprint
from .causal_observation import OBSERVATION_VERSION, prefix_observation
from .controlled_dataset import ActionExamples, ClockExamples, tensor_row
from .controlled_experiment import atomic_json
from .feature_schema import CONTINUOUS_INDEX as C, GLOBAL_INDEX as G, THREAT_DIM
from .policy_dataset import _encode_causal_stream, acting_cycle_features, recent_opponent_threat
from .policy_manifest import load_manifest, _id_hash
from .winner_dataset import collect_battles, build_vocab, load_card_costs, _deck_stats, CardVocab

CACHE_VERSION = "v9-causal-shards-1"


def card_event_fingerprint(battle):
    def view(swap):
        events = sorted((float(e["seconds"]), ("opponent" if e["side"] == "team" else "team") if swap else e["side"], e["card"], int(e["x"]), 32000-int(e["y"]) if swap else int(e["y"])) for e in battle.events if e["event_type"] == "card_play")
        return json.dumps([battle.opponent_deck if swap else battle.team_deck, battle.team_deck if swap else battle.opponent_deck, events], sort_keys=True)
    return hashlib.sha256(min(view(False), view(True)).encode()).hexdigest()

FIELDS = {
    "c": (np.float16, (64, 28)), "ids": (np.int16, (64,)),
    "td": (np.int16, (8,)), "od": (np.int16, (8,)),
    "g": (np.float32, (62,)), "sf": (np.float32, (8, 10)),
    "hm": (np.bool_, (8,)), "slot": (np.int64, ()),
    "kind": (np.int64, ()), "zone": (np.int64, ()),
    "xy": (np.float32, (2,)), "dt": (np.float32, ()),
    "length": (np.int64, ()), "weight": (np.float32, ()),
    "group": (np.int32, ()), "event": (np.int32, ()),
    "winner": (np.bool_, ()), "reaction": (np.bool_, ()),
    "cause": (np.int64, ()), "observed": (np.bool_, ()),
}
# Derive the cycle feature width instead of freezing a guessed layout.
from .policy_dataset import SLOT_FEAT_DIM
FIELDS["sf"] = (np.float32, (8, SLOT_FEAT_DIM))


class PrefixBank:
    """Fast path with exact fp32 parity to PolicyObservation at event boundaries."""
    def __init__(self, battle, vocab, costs):
        self.battle, self.vocab, self.costs = battle, vocab, costs
        self.streams = {
            side: _encode_causal_stream(battle, vocab, costs, swap_sides=side == "opponent", min_events=1)
            for side in ("team", "opponent")
        }

    def encode(self, end, side):
        b, vocab, costs = self.battle, self.vocab, self.costs
        if end == 0:
            return prefix_observation(b, 0, side).encode(vocab, costs)
        enemy = "opponent" if side == "team" else "team"
        own_deck = b.team_deck if side == "team" else b.opponent_deck
        enemy_deck = b.opponent_deck if side == "team" else b.team_deck
        prefix = replace(b, events=b.events[:end])
        seen = {e["card"] for e in prefix.events if e["side"] == enemy}
        masked = tuple(c if c in seen else "_unknown" for c in enemy_deck)
        stream = self.streams[side]
        c, ids, td, _, gs = stream
        start = max(0, end - 64)
        c = c[start:end].clone()
        c[:, C["seconds"]] = torch.tensor([float(e["seconds"]) / 300 for e in prefix.events[start:]])
        c[:, C["pad"]] = torch.tensor([not e.get("position_known", True) for e in prefix.events[start:]])
        g = gs[end - 1].clone()
        own_avg, _, own_spells = _deck_stats(own_deck, costs)
        other_avg, _, other_spells = _deck_stats(masked, costs)
        g[G["avg_cost_diff"]] = (own_avg - other_avg) / 10
        g[G["spell_frac_diff"]] = own_spells - other_spells
        g[G["shared"]] = len(set(own_deck) & set(masked)) / 8
        now = float(prefix.events[-1]["seconds"])
        # The canonical observation intentionally does not clamp its query clock.
        g[G["now"]] = now / 300
        threat, _ = recent_opponent_threat(prefix, end, side, costs, observation_seconds=now)
        sf, hm = acting_cycle_features(prefix, end, costs, side == "opponent", observation_seconds=now)
        od = torch.tensor([vocab.encode(c) for c in masked])
        return c, ids[start:end], td, od, torch.cat([g, threat]), sf, hm


def check_bank(battle, vocab, costs):
    bank = PrefixBank(battle, vocab, costs)
    for end in sorted({0, 1, min(8, len(battle.events)), len(battle.events) // 2, len(battle.events)}):
        for side in ("team", "opponent"):
            expected = prefix_observation(battle, end, side).encode(vocab, costs)
            actual = bank.encode(end, side)
            for j, (a, b) in enumerate(zip(actual, expected)):
                if not torch.equal(a, b):
                    raise AssertionError(f"Fast encoder differs: {battle.battle_id} end={end} side={side} field={j} error={(a.float()-b.float()).abs().max()}")


def _write_shard(task):
    root, shard_id, battles, first_group, vocab_dict, costs = task
    torch.set_num_threads(1)
    vocab = CardVocab.from_dict(vocab_dict)
    path = Path(root) / f"shard_{shard_id:04d}"
    path.mkdir(exist_ok=False)
    actions = ActionExamples(battles, vocab, costs, max_samples=0)
    clocks = ClockExamples(battles, vocab, costs)
    # Both populations are cached once with separate sampling contracts.
    counts = {"actions": len(actions), "clocks": len(clocks)}
    maps = {}
    for kind, n in counts.items():
        folder = path / kind
        folder.mkdir()
        maps[kind] = {
            key: np.lib.format.open_memmap(folder / f"{key}.npy", mode="w+", dtype=dtype, shape=(n, *shape))
            for key, (dtype, shape) in FIELDS.items()
        }
    banks = [PrefixBank(b, vocab, costs) for b in battles]
    # Check every shard against the independent canonical encoder, before trust.
    for b in battles[:2]:
        check_bank(b, vocab, costs)
    for kind, ds in (("actions", actions), ("clocks", clocks)):
        for j, index in enumerate(ds.index):
            bi, end = index[:2]
            b = battles[bi]
            if kind == "actions":
                e = b.events[end]
                boundary = end
                while boundary and b.events[boundary - 1]["seconds"] == e["seconds"]:
                    boundary -= 1
                side = e["side"]
                deck = b.team_deck if side == "team" else b.opponent_deck
                xy = (e["x"] / 18000, e["y"] / 32000 if side == "team" else 1 - e["y"] / 32000)
                now = float(b.events[boundary - 1]["seconds"]) if boundary else 0.
                row = tensor_row(banks[bi].encode(boundary, side), deck.index(e["card"]), 0, xy, float(e["seconds"]) - now)
                reaction = recent_opponent_threat(b, end, side, costs, max_age=5)[1]
                winner = (side == "team") == bool(b.team_wins)
                cause, observed = -1, False
            else:
                _, _, now, delay, cause, observed = index
                side = "team"
                row = tensor_row(banks[bi].encode(end, side), delay=delay)
                reaction, winner = False, False
            arrays = maps[kind]
            for key, value in zip(list(FIELDS)[:14], row):
                a = value.numpy() if isinstance(value, torch.Tensor) else value
                if key in {"c", "ids"}:
                    arrays[key][j] = 0
                    arrays[key][j, :len(a)] = a
                else:
                    arrays[key][j] = a
            for key, value in dict(group=first_group + bi, event=end, winner=winner, reaction=reaction, cause=cause, observed=observed).items():
                arrays[key][j] = value
    for fields in maps.values():
        for array in fields.values():
            array.flush()
    result = {"path": path.name, "battles": len(battles), **counts}
    atomic_json(path / "complete.json", result)
    return result


class ShardDataset(Dataset):
    """Batch reads from memory maps; no per-sample Python tensor encoding."""
    def __init__(self, root, kind="actions", limit_battles=None, winner_weight=1., reaction_weight=3., mirror=False, advantage=False):
        self.root, self.kind = Path(root), kind
        complete = self.root / "complete.json"
        if complete.exists():
            meta = json.loads(complete.read_text())
        elif limit_battles is not None:
            shards = [json.loads(p.read_text()) for p in sorted(self.root.glob("shard_*/complete.json"))]
            if sum(s["battles"] for s in shards) < limit_battles:
                raise FileNotFoundError("Requested nested training subset is not cached yet")
            # Never skip an incomplete shard in the middle of the seeded order.
            for i, shard in enumerate(shards):
                if shard["path"] != f"shard_{i:04d}":
                    raise FileNotFoundError("Cache prefix has an incomplete shard")
            meta = {"shards": shards}
        else:
            raise FileNotFoundError("Full cache has not completed")
        # Linux counts each mmap against RLIMIT_NOFILE. A full corpus has more
        # shards than a pilot; raise only the soft limit within the hard cap.
        soft,hard=resource.getrlimit(resource.RLIMIT_NOFILE)
        current_fds=len(list(Path("/proc/self/fd").iterdir()))
        needed=current_fds+len(meta["shards"])*(len(FIELDS)+2)+512
        if soft < needed:
            if hard != resource.RLIM_INFINITY and hard < needed:
                raise RuntimeError(f"Cache needs {needed} file descriptors; hard limit is {hard}")
            resource.setrlimit(resource.RLIMIT_NOFILE,(needed,hard))
        self.shards, self.offsets = [], [0]
        self.winner_weight, self.reaction_weight, self.mirror = winner_weight, reaction_weight, mirror
        self.advantage = advantage
        for shard in meta["shards"]:
            folder = self.root / shard["path"] / kind
            arrays = {k: np.load(folder / f"{k}.npy", mmap_mode="r") for k in FIELDS}
            n = len(arrays["slot"])
            if limit_battles is not None:
                n = int(np.searchsorted(arrays["group"], limit_battles))
            if n:
                if advantage:
                    arrays["advantage"] = np.load(folder / "advantage.npy", mmap_mode="r")
                self.shards.append(arrays)
                self.offsets.append(self.offsets[-1] + n)
            if limit_battles is not None and n < len(arrays["slot"]):
                break
        self.offsets = np.asarray(self.offsets)

    def __len__(self):
        return int(self.offsets[-1])

    def __getstate__(self):
        # Python 3.14 defaults to forkserver. NumPy pickles a memmap as array
        # contents unless we explicitly persist filenames; that would copy the
        # entire corpus into each loader worker and exhaust system RAM.
        state=self.__dict__.copy()
        state["shard_files"]=[{key:str(value.filename) for key,value in shard.items()} for shard in self.shards]
        state.pop("shards")
        return state

    def __setstate__(self,state):
        files=state.pop("shard_files")
        self.__dict__.update(state)
        self.shards=[{key:np.load(path,mmap_mode="r") for key,path in shard.items()} for shard in files]

    def __getitem__(self, index):
        raise RuntimeError("Use batched __getitems__ with identity collate")

    def __getitems__(self, indices):
        indices = np.asarray(indices, dtype=np.int64)
        which = np.searchsorted(self.offsets[1:], indices, side="right")
        data = {key: np.empty((len(indices), *shape), dtype=dtype) for key, (dtype, shape) in FIELDS.items()}
        advantage_weights = np.ones(len(indices), dtype=np.float32)
        for sid in np.unique(which):
            selected = np.flatnonzero(which == sid)
            local = indices[selected] - self.offsets[sid]
            for key in FIELDS:
                data[key][selected] = self.shards[sid][key][local]
            if self.advantage:
                advantage_weights[selected] = self.shards[sid]["advantage"][local]
        lengths = data["length"]
        max_len = int(lengths.max())
        tensors = []
        for key in list(FIELDS)[:14]:
            a = data[key]
            if key in {"c", "ids"}:
                a = a[:, :max_len].copy()
            t = torch.from_numpy(a)
            if key in {"ids", "td", "od"}:
                t = t.long()
            elif key == "c":
                t = t.float()
            tensors.append(t)
        if self.kind == "actions":
            weights = np.where(data["reaction"], self.reaction_weight, 1.) * np.where(data["winner"], self.winner_weight, 1.) * advantage_weights
            tensors[13] = torch.tensor(weights, dtype=torch.float32)
            if self.mirror:
                flags = torch.rand(len(indices)) < .5
                # Reflect all horizontal semantics together. Unknown positions stay .5.
                tensors[0][flags, :, C["x"]] = 1 - tensors[0][flags, :, C["x"]]
                for prefix in ("team", "opp"):
                    # Ability observations have unknown positions and contribute
                    # to the play denominator, never to either known lane.
                    plays=tensors[4][:,G[prefix+"_plays"]]*40
                    unknown=tensors[4][:,G[prefix+"_abilities"]]*8
                    known_fraction=torch.where(plays>0,(plays-unknown)/plays.clamp_min(1),0.)
                    k=G[prefix+"_left"]
                    tensors[4][flags,k]=known_fraction[flags]-tensors[4][flags,k]
                from .feature_schema import GLOBAL_DIM, THREAT_INDEX
                active = (tensors[4][:, GLOBAL_DIM + THREAT_INDEX["any_focus"]] > 0) | (tensors[4][:, GLOBAL_DIM + THREAT_INDEX["any_wincon"]] > 0)
                f = flags & active
                tx = GLOBAL_DIM + THREAT_INDEX["threat_x"]
                tensors[4][f, tx] = 1 - tensors[4][f, tx]
                tensors[10][flags, 0] = 1 - tensors[10][flags, 0]
                from .policy_model import xy_to_zone
                tensors[9] = torch.tensor([xy_to_zone(*p) for p in tensors[10]])
            return tuple(tensors)
        return tuple(tensors), torch.from_numpy(data["cause"]), torch.from_numpy(data["observed"])

    def metadata(self, key):
        return np.concatenate([s[key][:self.offsets[i+1]-self.offsets[i]] for i,s in enumerate(self.shards)])


class ShardSampler(Sampler):
    """Shuffle within random shard order to use SSD locality without sorted batches."""
    def __init__(self, dataset, seed):
        self.ds, self.seed = dataset, seed
    def __len__(self):
        return len(self.ds)
    def __iter__(self):
        rng = np.random.default_rng(self.seed)
        for i in rng.permutation(len(self.ds.shards)):
            yield from (rng.permutation(int(self.ds.offsets[i+1]-self.ds.offsets[i])) + self.ds.offsets[i]).tolist()


def identity(value):
    return value


def prepare(args):
    output = Path(args.output)
    output.mkdir(parents=True, exist_ok=True)
    if (output / "complete.json").exists():
        raise ValueError("Cache already complete; refusing to overwrite")
    costs = load_card_costs(args.costs)
    fingerprint = source_fingerprint(args.input)
    battles = collect_battles(args.input, metadata_path=args.metadata, require_decisive_result=True, unresolved_abilities="preserve_event")
    hold = load_manifest(args.holdout)
    hold_ids = set(hold["test_ids"])
    held_battles = collect_battles(args.input, battle_ids=hold_ids, metadata_path=args.metadata, require_decisive_result=False, unresolved_abilities="preserve_event")
    if {b.battle_id for b in held_battles} != hold_ids:
        raise ValueError("Final holdout sources missing")
    held_fingerprints = {card_event_fingerprint(b) for b in held_battles}
    a = load_manifest("data/splits/policy_v4_4_1_seed42.json")
    b = load_manifest("data/splits/policy_v4_4_2_winner_seed42.json")
    common = (set(a["validation_ids"]) | set(a["test_ids"])) & (set(b["validation_ids"]) | set(b["test_ids"]))
    available = {b.battle_id: b for b in battles}
    dev_ids = sorted(common & available.keys() - hold_ids)
    if len(dev_ids) < 100:
        raise ValueError("Too few common reference-unseen dev battles")
    dev_fingerprints = {card_event_fingerprint(available[x]) for x in dev_ids}
    train = [b for b in battles if b.battle_id not in hold_ids and b.battle_id not in dev_ids and card_event_fingerprint(b) not in held_fingerprints | dev_fingerprints]
    # Nested subsets use one seeded battle order, independent of the model seed.
    rng = np.random.default_rng(42)
    train = [train[i] for i in rng.permutation(len(train))]
    dev = [available[x] for x in dev_ids]
    vocab = build_vocab(train)  # Never fit vocabulary to held-out data.
    ids = {"train_ids": [b.battle_id for b in train], "validation_ids": dev_ids, "test_ids": []}
    manifest = dict(schema_version=2, seed=42, min_card_plays=12, battle_count=len(train)+len(dev), ordered_id_sha256=_id_hash(ids["train_ids"]+dev_ids), **ids)
    atomic_json(output / "split_manifest.json", manifest)
    contract = dict(cache_version=CACHE_VERSION, observation=OBSERVATION_VERSION, source=fingerprint, metadata_sha256=sha256_file(args.metadata), costs_sha256=sha256_file(args.costs), manifest_sha256=sha256_json(manifest), vocab_sha256=sha256_json(vocab.to_dict()), holdout_sha256=sha256_file(args.holdout), encoder_sha256=sha256_file(__file__), storage="fp16 event features, fp32 globals/cycle/labels", all_card_actions=True, max_context=64, reaction_repeats=1, reference_ancestry_caveat="Common holdout of final legacy manifests; older warm-start ancestors have incomplete lineage. Final untouched holdout is authoritative.")
    atomic_json(output / "contract.json", contract)
    atomic_json(output / "vocab.json", vocab.to_dict())
    with (output / "battles.pkl").open("wb") as f:
        pickle.dump(dict(train=train, validation=dev), f, protocol=5)
    print(f"Frozen corpus: {len(train)} train, {len(dev)} validation, {len(hold_ids)} reserved holdout", flush=True)
    started = time.monotonic()
    summary = {}
    for name, population in (("validation", dev), ("train", train)):
        folder = output / name
        folder.mkdir(exist_ok=True)
        tasks = [(str(folder), j, population[i:i+args.shard_battles], i, vocab.to_dict(), costs) for j,i in enumerate(range(0,len(population),args.shard_battles))]
        results = []
        with ProcessPoolExecutor(args.workers, mp_context=mp.get_context("spawn")) as pool:
            for result in pool.map(_write_shard, tasks):
                results.append(result)
                done = sum(r["battles"] for r in results)
                elapsed = time.monotonic() - started
                eta = elapsed / max(done,1) * (len(population)-done)
                row = dict(timestamp=datetime.now(timezone.utc).isoformat(), phase="cache", model_name=f"v9 cache {name}", work_done=done, work_total=len(population), samples_done=done, samples_total=len(population), eta_seconds=eta, elapsed_seconds=elapsed, progress_percent=100*done/len(population))
                with (output / "progress.jsonl").open("a") as f:
                    f.write(json.dumps(row)+"\n")
                print(f"cache {name:10} work={done}/{len(population)} ETA={eta:.0f}s actions={sum(r['actions'] for r in results)}", flush=True)
        complete = dict(contract=contract, battles=len(population), shards=results, actions=sum(r["actions"] for r in results), clocks=sum(r["clocks"] for r in results))
        atomic_json(folder / "complete.json", complete)
        summary[name] = complete
    atomic_json(output / "complete.json", dict(contract=contract, elapsed_seconds=time.monotonic()-started, splits=summary))


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--input", default="data/raw")
    p.add_argument("--metadata", default="data/metadata/battles.jsonl")
    p.add_argument("--costs", default="data/card_costs.json")
    p.add_argument("--holdout", default="data/splits/controlled_final_holdout.json")
    p.add_argument("--output", default="data/v9_cache")
    p.add_argument("--workers", type=int, default=6)
    p.add_argument("--shard-battles", type=int, default=256)
    prepare(p.parse_args())

if __name__ == "__main__":
    main()
