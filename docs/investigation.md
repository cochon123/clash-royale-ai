# Investigation: Hero ability noise and retraining feasibility

Date: 2026-07-26

## Conclusion

The proposal is technically valid and materially improves the labels, but
missing Hero abilities were not the only reason the previous model failed.

RoyaleAPI replay HTML contains the exact activation tick and side. New Hero
events can omit the Hero identity. When official battle metadata says that
side has exactly one active Hero, the identity is deterministic. Older
Champion events often name the ability directly in the icon URL.

The correct action space is therefore hierarchical:

1. `wait`, `play_card`, or `activate_ability`;
2. if `play_card`, choose a legal card in hand and a legal position;
3. if `activate_ability`, choose a currently deployed, living, off-cooldown
   Hero/Champion and pay the ability Elixir cost.

Ability events must not be forced through a card-placement head because they
have no arena deployment coordinates.

## Evidence from the old repository

Audited source: `/home/cochon/Documents/ClashRoyaleAI`

### Collection and parsing

- The extension captured `/data/replay` responses verbatim.
- Ability events are represented by `data-ability="1"`, a side, and a raw
  `data-t` tick.
- Their map marker is usually `_invalid` with `x=None` and `y=None`.
- The old parser skipped every marker with missing coordinates, silently
  deleting ability activations.
- The old collector used Firefox-only `browser.webRequest.filterResponseData`
  despite being documented as a Chrome extension.

### Dataset defects

- The V7 loader reconstructed each deck from the first eight unique cards
  played rather than using the actual deck.
- It treated those first four inferred cards as the starting hand and the
  other four as the queue. Neither is observable from the replay.
- The mirror transform flipped X while swapping players but did not rotate the
  player-relative Y axis. This is unlikely to represent a valid side swap.
- Elixir state was simulated from those inferred hands/decks, so a single
  missing event could desynchronize later legality features.
- Crown progression was partly fabricated through interpolation.
- Duplicated downloads and mirrored samples increased apparent sample count.
- No explicit action type represented abilities.

### Model/data mismatch

The logged V7 run used:

- 3,028 training matches and 758 validation matches;
- 260,730 augmented training windows;
- 13,878,100 parameters.

At epoch 50, train loss was 3.47 while validation loss was 5.72. Placement
accuracy within one tile was only 25.9%. The large gap is consistent with
overfitting and/or invalid state reconstruction. The surviving V7 Python
implementation in the working tree is a mock, while the available V8 artifact
is a 167 MB checkpoint without its complete reproducible training source.

## Legacy corpus audit

Input: 7,880 December 2025 files.

| Result | Count |
|---|---:|
| Cloudflare challenge captures | 4,040 |
| Duplicate valid replay downloads | 1,348 |
| Unique parsed battles | 2,492 |
| Accepted high-confidence battles | 1,592 |
| Rejected unique battles | 900 |
| Accepted card-play events | 120,416 |
| Accepted ability activations | 4,677 |
| Directly named ability activations | 2,588 |
| Single-Hero historical inferences | 2,089 |

The December 2025 roster fallback is intentionally limited to the four Heroes
available then: Mini P.E.K.K.A, Musketeer, Giant, and Knight. A replay is
rejected if an unnamed event has zero or multiple candidates on its side.

Generated outputs:

- `data/cleaned-legacy/matches.jsonl`
- `data/cleaned-legacy/events.jsonl`
- `data/cleaned-legacy/quarantine.jsonl`
- `reports/legacy_audit.json`

## Can a good model be retrained?

Yes for a behavior-cloning action prior; not yet for a strong autonomous
real-time player.

The replay stream contains actions, timing, sides, decks, and placements. It
does not contain the complete live arena state: unit positions over time,
health, targets, projectiles, status effects, or tower health trajectories.
A replay-only model can learn common cycles, timings, responses, placements,
and ability usage conditioned on action history. It cannot reliably react to
an unseen board state.

A practical next target is at least 50,000–100,000 unique, metadata-matched
ranked battles across multiple seasons, with battle-level deduplication. At
roughly the observed event density this produces millions of card actions and
well over 100,000 ability labels. Start with a small constrained baseline
before scaling model size.

For a real player, synchronize those labels with screenshots or another board
state extractor. Train and evaluate by battle/season, never by randomly
splitting action rows from the same battle.

## Official game-mechanic references

- [December 2025 update: initial Heroes and Hero slots](https://supercell.com/en/games/clashroyale/blog/release-notes/december-update-2025/)
- [December 2025 season: the initial four Heroes](https://supercell.com/en/games/clashroyale/blog/news/new-season-heroic-holidays/)
- [March 2026 slot rework](https://supercell.com/en/games/clashroyale/blog/news/mid-march-update/)
- [Supercell support: how Hero forms and abilities work](https://support.supercell.com/clash-royale/en/articles/heroes-4.html)

