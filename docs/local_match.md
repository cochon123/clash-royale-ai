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

The page to open is
[apkpure.com/clash-royale/com.supercell.clashroyale](https://apkpure.com/clash-royale/com.supercell.clashroyale).
On 2026-10-16, `curl` to that HTML page returned HTTP 403 (Cloudflare
"Just a moment..."). The transfer that returned the package is the APKPure CDN,
not a Supercell host:

```bash
mkdir -p var/clash-royale
curl -fL --retry 3 --retry-delay 2 \
  -A 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36' \
  -o var/clash-royale/clash-royale.xapk \
  'https://d.apkpure.net/b/APK/com.supercell.clashroyale?version=latest'
cr-replays local-match-prepare --apk var/clash-royale/clash-royale.xapk
```

The same CDN fetch with a host allowlist (it refuses `supercell.com`,
`clashroyale.com`, and every other host, including the HTML page) is:

```bash
cr-replays local-match-prepare --from-apkpure
```

Redirects are followed only to `d.apkpure.net`, `d.cdnpure.com`, and
`data.winudf.com`. This process never dials Supercell. It does not execute
the APK.

That download is an XAPK (a zip), not a single APK. `manifest.json` names
package `com.supercell.clashroyale`, version name `160402020`, version code
`160402020`. The base split is `com.supercell.clashroyale.apk`. Locale, density,
`config.arm64_v8a`, and `install_time_asset_pack` stay inside the XAPK.
`local-match-prepare` copies only the base split to `var/clash-royale/base.apk`
and checks the binary `AndroidManifest` package name. HTML challenge pages,
tiny files, and any other package are rejected.

Recorded on 2026-10-16 from that CDN object:

| file | bytes | sha256 |
| --- | ---: | --- |
| `clash-royale.xapk` | 1028220365 | `3dc0085605d495ee40b443084f7767ecc547d329be30b5f11884b0d345643f13` |
| `base.apk` | 49120331 | `0d0dfed74fdb755b71f5424e70af9234ed25471c029d09863b657fed9057b13e` |

`prepare-status.json` repeats version name, version code, size, and sha256.
Do not commit the XAPK, the base APK, or any extracted Supercell assets.
`var/clash-royale/`, `*.apk`, and `*.xapk` are gitignored.

A phone you control is still a valid source. It talks to local adb only:

```bash
cr-replays local-match-prepare --from-device <serial>
```

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
export REA_JADX_HEAP_MIB=2048
rea inspect-android-package var/clash-royale/base.apk --format json
cr-replays local-match-prepare --apk var/clash-royale/base.apk
```

`local-match-prepare` writes `var/clash-royale/prepare-status.json` (package,
version name, version code, sha256, size, XAPK layout, native library names).
It does not extract DEX, the asset pack, or `.so` files into git. REA's stdout
stays under `var/clash-royale/` as well. Put Node.js 22.19 or newer on `PATH`
before `npm` and `rea` (`rea-agents` 6.5.0 refuses 22.14). `ps` must be procps.
`REA_JADX_HEAP_MIB=2048` fit a 16 GB cloud VM; the 32 GB box can go higher.

On 2026-10-16 that inspect command exited 0 against `base.apk`. REA 6.5.0 /
JADX 0.7.1 (jar sha256 `6e5eacf5…ff75`) reported package
`com.supercell.clashroyale`, versionName `160402020`, versionCode `160402020`,
minSdk 29, targetSdk 35, and 15874 classes. Signature verification was
`not_performed`. The decoded manifest has `android:host="link.clashroyale.com"`
and `android:requiredSplitTypes="base__abi,base__density"`, and it has no
`127.0.0.1`. JADX does not verify split APKs or native libraries. The prepare
scan of `config.arm64_v8a.apk` covered `libsupercell_clashroyale.so` and
`libg.so`; neither contains `127.0.0.1`. `libg.so` contains
`https://android.googlesource.com` and nothing that listens for a battle.

That is the blocker. There is no concrete local combat interface to wire into
`cr-local-match/1`. `real_engine` stays false. The built-in harness is not the
combat engine. Do not patch the client to dial Supercell. The smallest next
step is a process you control on `http://127.0.0.1:8790` that speaks
`cr-local-match/1`, sets `real_engine` true, and actually resolves combat.

On the 3060 / 13th-gen i5 / 32 GB machine, `/dev/kvm` is the right place to run
an emulator later. This cloud VM also has `/dev/kvm`. An emulator would still
not create the missing battle resolver.

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
