# Banana round robin

This is a frozen offline tournament between Banana-1, the selected Banana-2
actor, and its matched BC-only continuation. No real game or optimizer runs.

The schedule contains 144 games: three model pairings, six classic decks, four
fresh native seeds and both arena seats. Each game uses an identical deck on
both sides, isolating policy strength from deck matchup advantage. The pool is
`engine/data/fast_league_decks.json`: Hog 2.6, Classic Log Bait, PEKKA Bridge
Spam, LavaLoon, Golem and Custom Mortar. Seeds are 3824100000–3824100003.

All actors use the same sampled phone-style controller, learned delay, actual
native hand checks, native placement mask and 20 Hz poll schedule. Model RNG
uses deterministic per-seed/seat/tick uniforms rather than worker completion
order. Accepted action history records the native resolved target, including
territory and pocket placement corrections.

The fast engine adds an opt-in `resolve_actions: true` step argument and
`resolve_target` query using the recovered `f9e16c` routine. Default raw steps
remain unchanged. Its separate build at
`engine/rust-engine/target-banana-eval/release/rust_league_rl` preserves the
existing enrichment engine binary. The preflight compares target resolution
and accepted actions against `native-watch`, the original emulator.

Only native terminal outcomes count. Wins score one point, draws half a point,
and losses zero. Native errors, wall-time limits and nonterminal truncations
are VOID; they never become guessed wins from tower HP. Standings show both
raw wins and score per valid match. A small margin in this bank is not a proof
of broad playing superiority or current-game fidelity.

The run is `engine/runs/banana-battle-royal-eval-20261024`, owned by
`banana-battle-royal-eval-20261024.service`. Four CPU workers have a 400% CPU
quota and a 5 GiB cgroup limit. Checkpoints remain frozen and their hashes are
checked again after completion. `manifest.json` pins the schedule, controller
sources and binary. `games.jsonl` retains one record per native game;
`replays/*.json.gz` stores requested and resolved actions and the terminal state.

```sh
tail -F ~/Documents/mini-project/clash-royale-ai/engine/runs/banana-battle-royal-eval-20261024/console.log
```

Progress prints completed work and ETA, followed by wins/losses/draws/voids for
each model. Small state/summary metadata mirrors to the home Lab; replay traces
stay local.

[Clash Lab tournament](http://localhost:8801/#runs?id=banana-battle-royal-eval-20261024)

## Epoch-7 rematch

`banana-battle-royal-e7-20261025` is a second frozen round robin. It keeps
Banana-1 and the epoch-3 Banana-2 export that won the first tournament, and
adds the epoch-7 Banana-2 export. That export is now the canonical
`engine/runs/banana-2/base.pt`. The epoch-3 actor is
`engine/runs/banana-2/archive-epoch3/base.pt`. The matched
BC-only twin is omitted: after seven epochs its development loss matches the
auxiliary arm to 0.00001. Seeds are 3824101000–3824101003, a fresh bank.
Same six mirror decks, both seats, four workers, and the same scoring rules.

```sh
tail -F ~/Documents/mini-project/clash-royale-ai/engine/runs/banana-battle-royal-e7-20261025/console.log
```

[Clash Lab rematch](http://localhost:8801/#runs?id=banana-battle-royal-e7-20261025)
