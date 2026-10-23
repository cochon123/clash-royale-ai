# opus5.5crazyrl engine host

`rust_league_rl` keeps the previous JSONL protocol. New fields and operations are
opt-in, so a request that does not ask for them gets the same reply shape as
before. Timing fields (`reset_seconds`, `step_seconds`, `query_seconds`) still
vary between runs.

## Board

`reset` accepts `"board": true`. While that episode is active, `reset`,
`observe`, and `step` replies include:

```json
"board": {"tick": 1234, "entities": [
  {"id": 17, "row": "HogRider", "side": 0, "x": 8500, "y": 12000,
   "hp": 1200, "max_hp": 1408, "dx": 0, "dy": 30, "first_seen_tick": 1180}
]}
```

`id` is the native entity id. `row` is the CSV row name from the same
presentation walk that already supplied tower HP. Coordinates are the native
arena integers. `dx` and `dy` are the displacement since the previous board
read in this episode, or 0 the first time that entity is seen. Entities with
no recognized health component use `hp: -1` and `max_hp: 0` (projectiles and
other non-health rows). When `reward_state` is also enabled, tower HP in
`reward_state` is taken from these entities.

`engine/data/board_rows.json` lists row names observed in real league games
and, where the recovered catalog identifies them, the source card, elixir cost,
and units placed per card.

## Snapshots

| Request | Result |
| --- | --- |
| `{"op":"snapshot"}` | `{"snapshot_id","tick","bytes"}` |
| `{"op":"restore","snapshot_id":n}` | the same object `observe` would return |
| `{"op":"drop_snapshot","snapshot_id":n}` | `{"snapshot_id":n}` |
| `{"op":"save_snapshot","snapshot_id":n,"path":"..."}` | adds `file_bytes` |
| `{"op":"load_snapshot","path":"..."}` | installs the image and returns a new id |

`bytes` is the dirty guest-page payload. Pages are kept only when they differ
from the reset checkpoint: file-backed pages are compared with the checkpoint
file, and anonymous pages with zeros. Restore discards copy-on-write changes
with the existing `MADV_DONTNEED` reset, then writes those pages back. The
image also keeps registers, the allocator (including its reset journal), import
counters, deck-swap hook state, world/battle/harness pointers, the descriptor
scratch pointer, and the protocol bookkeeping (decks, tower table, board
motion memory, finish flag).

`--max-snapshots N` (default 8, range 1..1024) refuses another snapshot when
the process is already holding that many. Saved files start with `CRSNAP01`.
Loading checks the pinned checkpoint SHA-256 and the running executable's
SHA-256, so a snapshot cannot be restored by a different build.

`restore` builds its observe reply and then rolls that read back, so the
native calls inside `observe` do not change the branch.

## Sessions

Guest memory is mapped at fixed original addresses, so two sessions cannot
share one address space. `--sessions N` (default 1) keeps one protocol process
and spawns N child processes of this same binary before any request. The
parent routes a request's `"session"` field (default 0). `step_many` runs
different sessions at the same time on std threads, one thread per session,
and returns `{"items":[...]}` in request order. Items for the same session run
in order on that session's thread. With `--sessions 1`, `step_many` runs in
process and only session 0 is valid.

```json
{"op":"step_many","items":[
  {"session":0,"ticks":20,"actions":[{"side":0,"slot":0,"x":3500,"y":7343}]},
  {"session":1,"ticks":20,"actions":[]}
]}
```

Python wrappers live in `rl/engine_ops.py` and use `NativeClient.request`.

## Measured on this machine

A release build, one Hog 2.6 development game, after the first step:

| | |
| --- | --- |
| dirty guest pages | 352,256 bytes |
| snapshot capture | 0.135 seconds |
| restore, including the rolled-back observe | 0.190 seconds |
| save file | 47,179,717 bytes |

The file is larger than the page payload because it also stores the allocator image, registers, and protocol bookkeeping.

Random legal play for about 75 seconds, `max_ticks` 3600, both sides calling `placement_mask` before a play. One 6G scope held every engine process.

| host | games/min | steps | mask calls |
| --- | --- | --- | --- |
| 1 process, `--sessions 8` | 66.23 | 16,528 | 16,549 |
| 8 single-session processes | 173.06 | 35,672 | 35,599 |

The 8-session host is slower on this workload because mask queries are one request at a time. `step_many` is the part that runs concurrently. Eight processes query masks at the same time. Peak memory for the 8-session tree was about 4.2GB.

The PGO binary is `rust-engine/target/pgo/rust_league_rl`, SHA-256 `32b6deb2f5116ebc84b74b6d25fb7e7ef90d38564ba74faafa811eff15103eaa`. Replaying the saved traces with default requests (no `board`) matched 36/36 development games (4,702 replies), 252/252 holdout-final games (46,044 replies), and 252/252 holdout-cycle games (33,111 replies). Hashes and commands are in `docs/opus55-engine-validation.json`.

## What blocks a one-sided built-in AI

The compiled RL loop advances the world with `10ca81c` and submits human
commands through the recovered play path. The original `Normal` bot is not a
separate per-side switch on that path. It lives inside harness step `11d6398`,
which plans and submits commands for the battle and then steps the world.
Calling it would let the original AI play both sides and would mix those
commands with external `play_card` submissions. There is no recovered,
source-proven flag that disables the bot on only one side without changing
the constructor sequence or patching game code. `native_ai_side` is therefore
not implemented.
