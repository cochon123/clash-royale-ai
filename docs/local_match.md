# Local match

This is a private loopback harness for the policies in this repo. It is not a
client for the live game. It never opens a socket to Supercell.

REA ([morluto/rea](https://github.com/morluto/rea)) is the inspection toolkit.
Its Android path is `rea inspect-android-package` (JADX) and
`rea pull-adb-package` (adb). It does not download Clash Royale and it does
not contain a battle server. The battle binary in the official client talks to
Supercell's servers; those hosts are refused here.

Until a process you control speaks `cr-local-match/1` on `127.0.0.1`, matches
run on the control harness:

- the full card roster from `data/v9_card_stats/cards.json` (Supercell card id,
  elixir, type), not an arena-gated subset
- an 8-card deck, at most one champion
- 4-card hand and cycle
- elixir (5 start, 10 cap, 2.8s / 1.4s / 0.9s)
- tile on the same 32×18 grid as the behavior-cloning policy
- actions are card id, tile, and match time

The harness does not simulate troops, targeting, or crowns.

## 1. Obtain the APK

REA's channel is a file you already have, or a pull from a device you control
that installed Clash Royale from the Play Store.

```bash
# phone plugged in, USB debugging on
adb devices
rea pull-adb-package <serial> com.supercell.clashroyale \
  --output-directory var/clash-royale
```

If `rea` is not installed yet, the same pull is:

```bash
cr-replays local-match-prepare --from-device <serial>
```

That requires `adb` on `PATH` (or `REA_ADB_PATH`). This environment has neither
`adb` nor a connected device. A HEAD request to APKMirror's public Clash Royale
page on 2026-10-16 returned HTTP 403 (Cloudflare challenge), so no APK was
downloaded. Do not commit the APK. `var/clash-royale/` and `*.apk` are gitignored.

## 2. Inspect with REA

Node.js 22+, a full JDK 17+ (`jdk.compiler`), and the pinned JADX jar from
REA's Android guide:

```bash
mkdir -p "$HOME/.local/share/rea/android"
curl -fL -o "$HOME/.local/share/rea/android/jadx-headless-mcp-0.7.1-all.jar" \
  https://github.com/1013503897/jadx-headless-mcp/releases/download/v0.7.1/jadx-headless-mcp-0.7.1-all.jar
echo '6e5eacf500b64292bfb73c49797c1958f6ee44646e43e868039ae7feb573ff75  jadx-headless-mcp-0.7.1-all.jar' \
  | sha256sum -c -
export REA_JADX_MCP_JAR="$HOME/.local/share/rea/android/jadx-headless-mcp-0.7.1-all.jar"
npm install --global rea-agents
rea inspect-android-package var/clash-royale/base.apk --json
cr-replays local-match-prepare --apk var/clash-royale/base.apk
```

`local-match-prepare` writes `var/clash-royale/prepare-status.json` (hash,
size, native library names). It does not extract DEX or assets into git.

On the 3060 / 13th-gen i5 / 32 GB machine, `/dev/kvm` is the right place to run
an emulator later. This cloud VM also has `/dev/kvm`, but an official client
still cannot enter a match without Supercell's servers. Do not point it at
them. A local engine replaces the resolver by listening on loopback.

## 3. Start the control server

```bash
python -m venv --system-site-packages .venv
.venv/bin/pip install -e .
.venv/bin/cr-replays local-match
```

Open `http://127.0.0.1:8770`.

- Red AI and Blue AI: `random` always. A `policy_bc_*` entry becomes selectable
  when `best_model.pt` is in that model directory (Hugging Face snapshots; the
  weights are gitignored).
- Deck: a famous preset, or change any of the eight cards.
- Launch battle. There is no tutorial and no menu. Both AIs start submitting
  actions immediately.

Stream progress (work done, time left, ETA):

```bash
watch -n 1 'curl -s http://127.0.0.1:8770/api/match/table'
```

Columns: `work`, `sim_s`, `left_s`, `eta_s`, `phase`.

One match on stdout, no browser:

```bash
cr-replays local-match --headless --max-seconds 30 \
  --red random --blue random \
  --red-deck hog_cycle_2.6 --blue-deck logbait
```

`--speed 1` sleeps in real time. The default `--speed 0` runs the clock as
fast as the CPU, which is what you want on the harness. Use `--speed 1` only
when a real loopback engine is attached.

## 4. Attach a local engine later

The wire protocol is HTTP `POST /v1` on `127.0.0.1` only.

```json
{"op":"hello"}
{"op":"start","red":["knight", "..."],"blue":["archers","..."],"seed":1,"max_seconds":180}
{"op":"advance","seconds":3.2}
{"op":"act","side":"red","card":"knight","tile_col":10,"tile_row":4,"event_type":"card_play"}
{"op":"state"}
```

`hello.real_engine` stays false for the built-in harness. A replacement sets
it true and resolves combat. Point the control plane at it with
`--endpoint http://127.0.0.1:8790`. Any other host is refused before connect.

Card identity in each accepted action is the integer `card_id` from the roster
(for example Knight is `26000000`). Tiles are the policy grid: 32 columns,
18 rows. Red plays the low rows. Blue plays the high rows. Spells, Miner, and
Goblin Drill may use either half.
