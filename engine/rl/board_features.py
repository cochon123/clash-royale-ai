"""C1 board JSON to learner-perspective tensors.

The placement head in ``banana_policy`` / ``banana_fast_policy`` treats the
arena as a 18000 by 32000 rectangle split into 18 rows and 32 columns
(576 tiles). ``native_clash`` reports ``arena_cells`` of [36, 64] with
500-unit cells (36 * 500 = 18000, 64 * 500 = 32000), so each placement tile
covers a 2 by 2 block of native cells. This module uses that placement grid.

Perspective matches the existing side swap: the acting side stays at the
bottom. Side 0 keeps native coordinates. Side 1 mirrors Y only
(``ny = 1 - y / 32000``), the same reflection ``BananaPolicy.act`` applies
before writing a placement. X is not mirrored. Motion dy flips with Y.

Row elixir comes from ``engine/data/board_rows.json`` when W1 has written it.
Otherwise the loader uses the built-in fallback: every native card in the six
decks of ``engine/data/fast_league_decks.json``, with elixir and unit counts
taken from ``cards.json`` (or elixir only from ``card_catalog.json`` when the
combat export has no row). An unknown row is a zero-elixir bucket.
"""
from __future__ import annotations

import hashlib
import json
import math
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Mapping, Sequence

import torch
from torch import Tensor

ROOT = Path(__file__).resolve().parents[1]
DEFAULT_ROWS_PATH = ROOT / "data" / "board_rows.json"
FAST_LEAGUE_DECKS = ROOT / "data" / "fast_league_decks.json"
CARDS_PATH = ROOT / "data" / "cards.json"
CARD_CATALOG_PATH = ROOT / "data" / "card_catalog.json"

# Placement frame. Keep these aligned with banana_policy.act.
ARENA_X = 18000.0
ARENA_Y = 32000.0
TILE_ROWS = 18
TILE_COLS = 32
NUM_TILES = TILE_ROWS * TILE_COLS
NATIVE_CELLS = (36, 64)
CELL_SIZE = 500

MAX_ENTITIES = 64
HASH_BUCKETS = 32
MOTION_SCALE = 1000.0
AGE_TICKS = 4000.0
TOWER_ROWS = ("KingTower", "PrincessTower")
ZERO_VALUE_KINDS = frozenset({"tower", "projectile", "area_effect"})
# Spawned rows that board_rows.json cannot cost from the card tables alone
# (several producers or a missing count). Values are elixir per entity, taken
# from the usual deploying card in the training pool. Heuristic, not native.
UNIT_VALUE_OVERRIDES = {
    "Skeleton": ("Skeletons", 1.0 / 3.0),
    "SpearGoblin": ("GoblinGang", 0.5),
    "Goblin_Stab": ("GoblinGang", 0.5),
    "Minion": ("Minions", 1.0),
    "Bat": ("Bats", 0.4),
    "Barbarian": ("Barbarians", 1.0),
    "ElectroWizard": ("ElectroWizard", 4.0),
    "BalloonBomb": ("Balloon", 0.0),
    "RageBarbarianBottle": ("RageBarbarian", 0.0),
}

FEATURE_NAMES = ("own", "x", "y", "hp_fraction", "motion_x", "motion_y", "age")
RASTER_CHANNELS = (
    "own_hp",
    "enemy_hp",
    "own_value",
    "enemy_value",
    "own_tower_hp",
    "enemy_tower_hp",
    "own_motion",
    "enemy_motion",
)
SCALAR_NAMES = (
    "own_left_value",
    "own_right_value",
    "enemy_left_value",
    "enemy_right_value",
    "enemy_value_on_my_side",
)


@dataclass(frozen=True)
class RowCost:
    """Elixir of one source card and how many board entities share it."""

    card: str | None
    elixir: float
    units_per_card: float
    unknown: bool = False
    kind: str = "unit"

    def entity_value(self, hp_fraction: float) -> float:
        if self.unknown or self.kind in ZERO_VALUE_KINDS or self.units_per_card <= 0:
            return 0.0
        return self.elixir / self.units_per_card * float(hp_fraction)


@dataclass
class RowTable:
    """Row name to card cost. Unknown rows share one zero-value bucket."""

    rows: dict[str, RowCost]
    unknown: RowCost
    source: str

    def lookup(self, row: str) -> RowCost:
        if row in TOWER_ROWS:
            return RowCost(card=row, elixir=0.0, units_per_card=1.0, kind="tower")
        return self.rows.get(row, self.unknown)

    def known(self, row: str) -> bool:
        return row in TOWER_ROWS or row in self.rows


@dataclass(frozen=True)
class EntityView:
    row: str
    entity_id: int
    own: bool
    tower: bool
    unknown: bool
    x: float
    y: float
    motion_x: float
    motion_y: float
    hp_fraction: float
    value: float
    age: float


class RowVocab:
    """Stable type ids. Known rows are sorted; everything else hashes."""

    def __init__(self, names: Sequence[str], hash_buckets: int = HASH_BUCKETS):
        self.names = tuple(sorted({str(name) for name in names if str(name)}))
        self.hash_buckets = int(hash_buckets)
        if self.hash_buckets < 1:
            raise ValueError("hash_buckets must be positive")
        self.name_to_id = {name: index + 1 for index, name in enumerate(self.names)}
        self.bucket_offset = len(self.names) + 1
        self.size = self.bucket_offset + self.hash_buckets

    def encode(self, row: str) -> int:
        known = self.name_to_id.get(str(row))
        if known is not None:
            return known
        return self.bucket_offset + stable_hash(str(row)) % self.hash_buckets

    def to_dict(self) -> dict[str, Any]:
        return {"names": list(self.names), "hash_buckets": self.hash_buckets}

    @classmethod
    def from_dict(cls, payload: Mapping[str, Any]) -> "RowVocab":
        return cls(payload["names"], int(payload["hash_buckets"]))


@dataclass
class BoardTensors:
    """One match, already normalized so the learner is the bottom side."""

    type_ids: Tensor
    features: Tensor
    mask: Tensor
    raster: Tensor
    scalars: Tensor
    perspective: int
    tick: int
    dropped: int
    unknown_count: int
    own_value: float
    enemy_value: float

    def clone(self) -> "BoardTensors":
        return BoardTensors(
            type_ids=self.type_ids.detach().cpu().clone(),
            features=self.features.detach().cpu().clone(),
            mask=self.mask.detach().cpu().clone(),
            raster=self.raster.detach().cpu().clone(),
            scalars=self.scalars.detach().cpu().clone(),
            perspective=self.perspective,
            tick=self.tick,
            dropped=self.dropped,
            unknown_count=self.unknown_count,
            own_value=self.own_value,
            enemy_value=self.enemy_value,
        )


@dataclass
class BoardBatch:
    type_ids: Tensor
    features: Tensor
    mask: Tensor
    raster: Tensor
    scalars: Tensor

    def to(self, device) -> "BoardBatch":
        return BoardBatch(
            type_ids=self.type_ids.to(device, non_blocking=True),
            features=self.features.to(device, non_blocking=True),
            mask=self.mask.to(device, non_blocking=True),
            raster=self.raster.to(device, non_blocking=True),
            scalars=self.scalars.to(device, non_blocking=True),
        )


def stable_hash(text: str) -> int:
    digest = hashlib.blake2s(text.encode("utf-8"), digest_size=8).digest()
    return int.from_bytes(digest, "little")


def _finite_number(value: Any, label: str) -> float:
    try:
        number = float(value)
    except (TypeError, ValueError) as error:
        raise ValueError(f"board field {label} is not a number") from error
    if not math.isfinite(number):
        raise ValueError(f"board field {label} is not finite")
    return number


def learner_motion(x: float, y: float, dx: float, dy: float, perspective: int) -> tuple[float, float, float, float]:
    """Map physical arena units into the learner frame used by placement."""
    if perspective not in (0, 1):
        raise ValueError("perspective must be 0 or 1")
    if not 0 <= x <= ARENA_X or not 0 <= y <= ARENA_Y:
        raise ValueError("board coordinates are outside the 18000 by 32000 arena")
    nx = x / ARENA_X
    ny = y / ARENA_Y
    mx = dx / MOTION_SCALE
    my = dy / MOTION_SCALE
    if perspective == 1:
        ny = 1.0 - ny
        my = -my
    return nx, ny, mx, my


def tile_of(x_norm: float, y_norm: float) -> tuple[int, int]:
    """Placement tile containing a learner-frame point. Tile id is ``row * 32 + col``."""
    column = min(TILE_COLS - 1, max(0, int(math.floor(x_norm * TILE_COLS))))
    row = min(TILE_ROWS - 1, max(0, int(math.floor(y_norm * TILE_ROWS))))
    if x_norm >= 1.0:
        column = TILE_COLS - 1
    if y_norm >= 1.0:
        row = TILE_ROWS - 1
    return row, column


def _parse_cost(row: str, payload: Mapping[str, Any], *, unknown: bool = False) -> RowCost:
    kind = payload.get("kind")
    if row in UNIT_VALUE_OVERRIDES:
        card, value = UNIT_VALUE_OVERRIDES[row]
        return RowCost(card=card, elixir=value, units_per_card=1.0, kind=str(kind or "troop"))
    if not unknown and (payload.get("elixir") is None or payload.get("units_per_card") is None):
        # W1 leaves projectiles, area effects and ambiguous spawns without a
        # cost. They keep their own type id but carry no board value.
        if kind in ZERO_VALUE_KINDS or payload.get("uncertain"):
            return RowCost(card=payload.get("source_card"), elixir=0.0, units_per_card=1.0,
                           kind=str(kind) if kind in ZERO_VALUE_KINDS else "unvalued")
    elixir = _finite_number(payload.get("elixir", 0 if unknown else None), f"{row}.elixir")
    units = _finite_number(payload.get("units_per_card", 1), f"{row}.units_per_card")
    if elixir < 0 or units <= 0:
        raise ValueError(f"row {row} needs non-negative elixir and positive units_per_card")
    card = payload.get("card", payload.get("source_card"))
    return RowCost(
        card=None if card is None else str(card),
        elixir=elixir,
        units_per_card=units,
        unknown=unknown,
        kind=str(payload.get("kind", "unknown" if unknown else "unit")),
    )


def _table_from_mapping(rows: Mapping[str, Any], unknown_payload: Mapping[str, Any] | None, source: str) -> RowTable:
    parsed: dict[str, RowCost] = {}
    for name, payload in rows.items():
        if name in TOWER_ROWS:
            continue
        if not isinstance(payload, Mapping):
            raise ValueError(f"board row {name} must be an object")
        parsed[str(name)] = _parse_cost(str(name), payload)
    unknown = _parse_cost("unknown", unknown_payload or {"elixir": 0, "units_per_card": 1}, unknown=True)
    return RowTable(parsed, unknown, source)


def parse_board_rows(payload: Mapping[str, Any], source: str = "board_rows.json") -> RowTable:
    """Read W1's row table.

    Accepted shape::

        {"format": "board-rows-v1",
         "unknown": {"elixir": 0, "units_per_card": 1},
         "rows": {"HogRider": {"card": "hog-rider", "elixir": 4,
                               "units_per_card": 1, "kind": "troop"}}}

    ``rows`` may also be a list of objects that each include ``row``.
    """
    if "format" in payload and payload["format"] != "board-rows-v1":
        raise ValueError(f"unsupported board row table format: {payload['format']}")
    raw_rows = payload.get("rows", payload)
    if isinstance(raw_rows, list):
        mapping = {}
        for item in raw_rows:
            if not isinstance(item, Mapping) or "row" not in item:
                raise ValueError("board row list entries need a row name")
            mapping[str(item["row"])] = item
        raw_rows = mapping
    if not isinstance(raw_rows, Mapping):
        raise ValueError("board row table needs a rows object")
    unknown = payload.get("unknown") if isinstance(payload.get("unknown"), Mapping) else None
    return _table_from_mapping(raw_rows, unknown, source)


def fallback_row_table() -> RowTable:
    """Costs for the six fast-league decks, from the recovered card catalogs.

    Each deck ``native_cards`` entry is looked up by ``sc_name``. Elixir and
    ``count`` come from ``cards.json``. Tombstone is only in ``card_catalog.json``,
    so its fallback count is 1. When ``raw_record`` differs from ``sc_name``
    (Skeleton for Skeletons, BarrelGoblin for GoblinBarrel, Goblin_Stab for
    GoblinGang, Minion for Minions), that character row receives the same card
    cost and the same units-per-card. It does not invent a separate spawn
    table: a lone Goblin_Stab is valued as one sixth of Goblin Gang, and a
    tombstone skeleton shares the Skeletons card's 1/3 because its raw row is
    Skeleton. Rows outside this set use the unknown bucket (elixir 0).
    """
    decks = json.loads(FAST_LEAGUE_DECKS.read_text())["decks"]
    cards = {row["sc_name"]: row for row in json.loads(CARDS_PATH.read_text())}
    catalog = {row["sc_name"]: row for row in json.loads(CARD_CATALOG_PATH.read_text())}
    parsed: dict[str, RowCost] = {}
    aliases: list[tuple[str, RowCost]] = []
    for deck in decks:
        for name in deck["native_cards"]:
            if name in parsed or name in TOWER_ROWS:
                continue
            card = cards.get(name)
            if card is not None:
                units = int(card["count"])
                cost = RowCost(
                    card=str(card["id"]),
                    elixir=float(card["elixir"]),
                    units_per_card=float(units if units > 0 else 1),
                    kind=str(card["kind"]),
                )
                raw = card.get("raw_record")
                if raw and raw != name:
                    aliases.append((str(raw), cost))
            else:
                entry = catalog.get(name)
                if entry is None:
                    raise ValueError(f"fast-league card {name} is absent from the card catalogs")
                cost = RowCost(card=str(entry["id"]), elixir=float(entry["elixir"]), units_per_card=1.0, kind="catalog")
            parsed[name] = cost
    for raw, cost in aliases:
        parsed.setdefault(raw, cost)
    unknown = RowCost(card=None, elixir=0.0, units_per_card=1.0, unknown=True, kind="unknown")
    return RowTable(parsed, unknown, "fallback:fast_league_decks")


def load_row_table(path: str | Path | None = None) -> RowTable:
    """Load ``board_rows.json``. The default path falls back when the file is absent.

    An explicit path that does not exist is an error. A present file replaces
    the fallback entirely; rows it omits are unknown and worth zero elixir.
    """
    if path is None:
        if not DEFAULT_ROWS_PATH.is_file():
            return fallback_row_table()
        path = DEFAULT_ROWS_PATH
    else:
        path = Path(path)
        if not path.is_file():
            raise FileNotFoundError(path)
    return parse_board_rows(json.loads(path.read_text()), source=str(path))


def vocab_for_table(table: RowTable, hash_buckets: int = HASH_BUCKETS) -> RowVocab:
    return RowVocab([*table.rows.keys(), *TOWER_ROWS], hash_buckets)


def _entity_views(board: Mapping[str, Any], perspective: int, table: RowTable) -> tuple[int, list[EntityView]]:
    if perspective not in (0, 1):
        raise ValueError("perspective must be 0 or 1")
    if not isinstance(board, Mapping) or not isinstance(board.get("entities"), list):
        raise ValueError("board JSON needs an entities list")
    tick = int(_finite_number(board.get("tick", 0), "tick"))
    views: list[EntityView] = []
    for index, entity in enumerate(board["entities"]):
        if not isinstance(entity, Mapping):
            raise ValueError(f"board entity {index} is not an object")
        row = str(entity.get("row", ""))
        if not row:
            raise ValueError(f"board entity {index} has no row")
        side = entity.get("side")
        if side not in (0, 1):
            raise ValueError(f"board entity {row} side must be 0 or 1")
        x = _finite_number(entity.get("x"), f"{row}.x")
        y = _finite_number(entity.get("y"), f"{row}.y")
        hp = _finite_number(entity.get("hp"), f"{row}.hp")
        maximum = _finite_number(entity.get("max_hp"), f"{row}.max_hp")
        if maximum <= 0 or hp < 0 or hp > maximum:
            raise ValueError(f"board entity {row} has invalid hp")
        dx = _finite_number(entity.get("dx", 0), f"{row}.dx")
        dy = _finite_number(entity.get("dy", 0), f"{row}.dy")
        first_seen = entity.get("first_seen_tick", tick)
        age_ticks = tick - _finite_number(first_seen, f"{row}.first_seen_tick")
        age = min(1.0, max(0.0, age_ticks / AGE_TICKS))
        nx, ny, mx, my = learner_motion(x, y, dx, dy, perspective)
        fraction = hp / maximum
        cost = table.lookup(row)
        views.append(EntityView(
            row=row,
            entity_id=int(entity.get("id", 0) or 0),
            own=side == perspective,
            tower=row in TOWER_ROWS,
            unknown=not table.known(row),
            x=nx,
            y=ny,
            motion_x=mx,
            motion_y=my,
            hp_fraction=fraction,
            value=0.0 if row in TOWER_ROWS else cost.entity_value(fraction),
            age=age,
        ))
    return tick, views


def _sort_key(view: EntityView) -> tuple:
    # Towers, then own units, then entities closest to the learner's baseline.
    # The key does not include input position, so a reversed entity list matches.
    return (0 if view.tower else 1, 0 if view.own else 1, view.y, view.x, view.row, view.entity_id)


def board_values(board: Mapping[str, Any], perspective: int, table: RowTable | None = None) -> tuple[float, float, int]:
    """Return ``(own_board_value, enemy_board_value, unknown_count)``."""
    table = load_row_table() if table is None else table
    _, views = _entity_views(board, perspective, table)
    own = sum(view.value for view in views if view.own and not view.tower)
    enemy = sum(view.value for view in views if not view.own and not view.tower)
    unknown = sum(1 for view in views if view.unknown)
    return own, enemy, unknown


def empty_board(perspective: int = 0) -> BoardTensors:
    return encode_board({"tick": 0, "entities": []}, perspective, load_row_table(), vocab_for_table(load_row_table()))


def encode_board(
    board: Mapping[str, Any],
    perspective: int,
    table: RowTable | None = None,
    vocab: RowVocab | None = None,
) -> BoardTensors:
    """Encode one C1 board.

    The entity set keeps at most 64 rows after a deterministic sort and drops
    the rest. The raster and the lane scalars still see every entity, so a
    crowded field does not erase elixir value from the scalar summary.
    """
    table = load_row_table() if table is None else table
    vocab = vocab_for_table(table) if vocab is None else vocab
    tick, views = _entity_views(board, perspective, table)
    own_value = sum(view.value for view in views if view.own and not view.tower)
    enemy_value = sum(view.value for view in views if not view.own and not view.tower)
    unknown_count = sum(1 for view in views if view.unknown)

    scalars = torch.zeros(len(SCALAR_NAMES), dtype=torch.float32)
    for view in views:
        if view.tower:
            continue
        lane = 0 if view.own else 2
        lane += 0 if view.x < 0.5 else 1
        scalars[lane] += view.value
        if not view.own and view.y < 0.5:
            scalars[4] += view.value

    raster = torch.zeros(len(RASTER_CHANNELS), TILE_ROWS, TILE_COLS, dtype=torch.float32)
    for view in views:
        row, column = tile_of(view.x, view.y)
        own = view.own
        if view.tower:
            raster[4 if own else 5, row, column] += view.hp_fraction
        else:
            raster[0 if own else 1, row, column] += view.hp_fraction
            raster[2 if own else 3, row, column] += view.value
            raster[6 if own else 7, row, column] += math.hypot(view.motion_x, view.motion_y)

    ordered = sorted(views, key=_sort_key)
    dropped = max(0, len(ordered) - MAX_ENTITIES)
    kept = ordered[:MAX_ENTITIES]
    type_ids = torch.zeros(MAX_ENTITIES, dtype=torch.long)
    features = torch.zeros(MAX_ENTITIES, len(FEATURE_NAMES), dtype=torch.float32)
    mask = torch.zeros(MAX_ENTITIES, dtype=torch.bool)
    for index, view in enumerate(kept):
        type_ids[index] = vocab.encode(view.row)
        features[index, 0] = 1.0 if view.own else 0.0
        features[index, 1] = view.x
        features[index, 2] = view.y
        features[index, 3] = view.hp_fraction
        features[index, 4] = view.motion_x
        features[index, 5] = view.motion_y
        features[index, 6] = view.age
        mask[index] = True
    return BoardTensors(
        type_ids=type_ids,
        features=features,
        mask=mask,
        raster=raster,
        scalars=scalars,
        perspective=perspective,
        tick=tick,
        dropped=dropped,
        unknown_count=unknown_count,
        own_value=own_value,
        enemy_value=enemy_value,
    )


def collate_boards(boards: Sequence[BoardTensors]) -> BoardBatch:
    if not boards:
        raise ValueError("cannot collate an empty board batch")
    return BoardBatch(
        type_ids=torch.stack([board.type_ids for board in boards]),
        features=torch.stack([board.features for board in boards]),
        mask=torch.stack([board.mask for board in boards]),
        raster=torch.stack([board.raster for board in boards]),
        scalars=torch.stack([board.scalars for board in boards]),
    )
