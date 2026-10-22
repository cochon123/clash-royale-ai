# Six-deck original-engine simulator

`rust_league_rl` runs compiled routines from the recovered game build. It
supports different eight-card decks on the two sides, native seeds, legal
placement queries, and remaining tower HP. It fails explicitly on uncovered
code paths. It does not fall back to Unicorn or use the approximate rules in
the repository's root `src/battle.rs`.

The pinned [deck list](../data/fast_league_decks.json) contains Hog 2.6, classic
Log Bait, PEKKA Bridge Spam, LavaLoon, Golem Night Witch, and the saved Mortar
deck: Mortar, Knight, Archers, Skeletons, Barbarian Barrel, Arrows, Mini PEKKA,
and Rocket. These are the user-approved classic archetypes compatible with
the recovered build, without current heroes or evolutions.

## Measured result

The frozen release passed the 95% target: **504/504 fresh games matched
exactly**, including **154/154 for each deck** and **79,155 semantic protocol
replies**. Both test cohorts passed separately. Final princess and king HP
had zero mean and maximum error across 3,024 tower values. All 39 card types
were deployed; the card-cycling cohort played Golem in 73 of its 77 eligible
games. Games that ended before Golem was played remain in the denominator.

Eight full-state audits matched all 12 guest-memory regions, all 70
architectural registers and allocator state. The final translator's 3,200
instruction checks and the older runner's 422-reply compatibility replay also
passed. The [validation record](../rust-engine/league-validation.json) pins
the executable and evidence hashes.

In a sequential benchmark of 12 identical complete games per engine, the
compiled backend ran **26.08 games/minute**, versus **10.68** for the original
Unicorn runner: **2.44× faster** in one worker process. This includes reset,
placement queries, action submission, tower HP and JSON IPC; it excludes
process startup and neural-policy inference. Raw timings and accuracy evidence
are under [the retained run](../analysis/fast-league-worktree-20261022/).

## Profile-guided build

The generator emits structured Rust control flow (`structured_cfg` in
`rust-engine/codegen.json`) instead of one block-dispatch loop per routine.
LLVM profile-guided optimization is then trained by replaying the 36
development games exactly in an instrumented build. Neither step changes
translated semantics. The profiled build still matched **504/504** held-out
regression games and **79,155** replies. The trainer's `rust_deck_rl` matched
all **2,796** retained replies.

| Single core, complete games | Release | Dense PGO | Structured PGO |
| --- | ---: | ---: | ---: |
| Development games/minute | 25.5 | 71.7 | 84.5 |
| Held-out games/minute | 17.8 | 51.1 | 59.1 |

That is about 3.3× faster per core than the frozen release build. In a
two-minute, ten-worker Banana training run, decisions per second went from 147
to 217 (+47%). The trainer process tree then averaged only 2.1 CPU cores, so
policy inference and the Python collection loop are now the main limit.
The [PGO record](../rust-engine/pgo-validation.json) pins the profile,
source and executable hashes.

```sh
rustup component add llvm-tools-preview
python3 tools/league_pgo.py --out analysis/my-pgo --phase instrument
python3 tools/league_pgo.py --out analysis/my-pgo --phase train
python3 tools/league_pgo.py --out analysis/my-pgo --phase pgo
python3 tools/league_pgo.py --out analysis/my-pgo --phase pgo --bin rust_deck_rl
mkdir -p rust-engine/target/pgo
cp analysis/my-pgo/bin/pgo rust-engine/target/pgo/rust_league_rl
cp analysis/my-pgo/bin/pgo-rust_deck_rl rust-engine/target/pgo/rust_deck_rl
```

`rl.banana_fast_train` uses `rust-engine/target/pgo/rust_deck_rl` when it
exists, otherwise the release build. The PGO tool refuses to reuse a profile
after the hashed sources change. After any generator or source change, use a
fresh output directory and rerun the league regression and deck replays.

## Validation criteria

The target is at least 95% winner agreement overall and for each deck, with
mean absolute remaining-HP error below five percentage points for princess
and king towers. HP is normalized by each original tower's maximum HP.
Removed towers retain their initial identity and receive zero HP. Unsupported
games count as winner failures; the report also penalizes unavailable HP
outcomes by the maximum possible error instead of dropping those games.

Development uses 36 games covering every ordered matchup. Coverage comes
from original-engine execution of those games, combined with the existing
source-bound profile. This adds exact original routines, rather than changing
combat parameters to fit final outcomes. The generator preserves two original
constructor hook locations used by the reference to select per-side decks.

An initial 252-game test exposed two missing routines: 250 games matched
exactly and two stopped explicitly. Those routines added 29 original
instructions. That cohort is retained as discovery/regression evidence;
the revised candidate's final test uses new seeds starting at 1,040,000.

The final candidate is frozen before generating two separate test cohorts:
seven new seeds for every ordered matchup in each cohort, 504 games total.
Each deck appears in 154 distinct games, counting a mirror once. The first
cohort varies deployment locations and affordable-card choices. The second
saves elixir and repeatedly cycles through all eight cards, ensuring expensive
win conditions such as Golem are exercised. Both vary decision intervals
(10, 20 or 40 ticks). The same frozen binary is used for both cohorts.
Every semantic JSON
reply is compared, including actions, observations, placement masks, winners
and tower HP; only elapsed-time fields are ignored. No hand, resource, or
combat-state synchronization is injected during replay.

Results apply to the pinned recovered build and the tested scripted action
distribution. They do not establish parity with today's live game, every
possible placement sequence, or an arbitrary trained policy. The existing
training entry points are unchanged; neural-policy inference is outside the
engine throughput measurement.

## Build and protocol

The existing recovered source, checkpoint and indexed original functions must
be present under `analysis/`, as described in the [backend README](../rust-engine/README.md).
Run one session per process on Linux x86_64.

```sh
cargo build --release --offline --manifest-path rust-engine/Cargo.toml \
  --bin rust_league_rl
rust-engine/target/release/rust_league_rl \
  --checkpoint analysis/native-pipeline/oracle-export-01/stage01-attempt001/state
```

Send one JSON object per line. `reset` requires `deck0` and `deck1`, each an
array of eight exact `native_cards` names from the pinned deck file, plus an
optional `seed`, `max_ticks` and `reward_state: true`. Then use `step` with
`ticks` and `actions: [{"side":0,"slot":0,"x":4500,"y":12000}]`.
The other operations are `observe`, `reward_state`, `placement_mask`, `finish`,
`audit` and `close`. The reply envelope is `{"ok":true,"result":...}`;
failures return `{"ok":false,"error":...}`. A terminal or finished game
requires a reset before further steps.

## Reproduce the checks

Build the independent original ARM/Unicorn reference:

```sh
cargo build --release --offline --manifest-path native-engine/Cargo.toml \
  --bin league_validation_reference
```

Record fresh held-out games only after freezing the candidate. Use a new
output directory; the recorder refuses to overwrite an existing manifest.

```sh
python3 tools/league_validation.py --record \
  --reference native-engine/target/release/league_validation_reference \
  --candidate rust-engine/target/release/rust_league_rl \
  --out analysis/my-new-league-test --split holdout \
  --seed-base 1040000 --seeds-per-matchup 7 --workers 6
python3 tools/league_report.py analysis/my-new-league-test
```

For the complementary card-cycling cohort, use another fresh output directory,
`--policy cycle` and a different seed base (the retained run uses 1,300,000).
Report both cohorts together with:

```sh
python3 tools/league_report.py analysis/my-new-league-test \
  analysis/my-new-card-cycle-test --out analysis/my-combined-accuracy.json
```

The combined report checks the target for each individual cohort as well as
the aggregate, so the random-action results cannot mask a weaker cycling test.

`tools/league_audit.py` compares complete guest memory, all 70 architectural
registers and allocator state for six development games. `tools/league_benchmark.py`
runs identical complete replays in one engine process at a time, alternates
engine order and records the binary hashes. Its games/minute includes reset,
legal-mask queries, action submission, tower reads and JSON IPC, excluding
process startup and neural inference.
