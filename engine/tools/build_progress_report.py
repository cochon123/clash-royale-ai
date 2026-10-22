#!/usr/bin/env python3
"""Build the standalone, evidence-linked project report from retained results."""
from pathlib import Path
import base64
import datetime
import hashlib
import html
import json
import statistics

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "reports/clash-rust-report.html"


def load(path):
    return json.loads((ROOT / path).read_text())


def esc(value):
    return html.escape(str(value), quote=True)


EVIDENCE = [
    ("package", "Package identity and REA inspection", "analysis/rea-android-package.json",
     "Static package metadata for the supplied build; not runtime execution."),
    ("ledger", "Original extraction and investigation ledger", "docs/reverse-engineering.md",
     "Historical ledger includes superseded prototype-era status; current findings are consolidated here."),
    ("data", "Recovered data provenance", "data/provenance.json",
     "Packaged and decoded identities for recovered resources."),
    ("ui", "Original UI recovery methods", "docs/native-ui-recovery.md",
     "Display trees, texture formats and the explicit gaps in runtime UI behavior."),
    ("ui-proof", "Rust/Python asset raster comparison", "analysis/native-ui/rust-verification/comparison.json",
     "56 sampled asset states; reference raster parity, not official-client parity."),
    ("flow", "Recovered flow and configuration logic", "docs/native-flow-logic.md",
     "Separates Merge Tactics graphs from ordinary Royale native behavior."),
    ("cipher", "Native code decryption investigation", "docs/libg-cipher.md",
     "Cipher, metadata layout, translation checks and reproduction commands."),
    ("cipher-proof", "All protected ranges comparison", "analysis/libg-cipher/direct-all-ranges-proof.json",
     "All five ranges match completed native loader execution."),
    ("bindings", "Protection-binding audit", "analysis/native-init-array/protection-investigation/binding-audit.json",
     "416 of 416 bindings verified against retained original decoder outputs; envelope signature not verified."),
    ("focus", "Focused native analysis workflow", "docs/native-focused.md",
     "Source-bound FDE analysis and the Ghidra address convention."),
    ("bootstrap", "Native resource bootstrap", "docs/native-resource-bootstrap.md",
     "Original parsers, typed tables, SCDB, workers, allocation and modeled OS boundaries."),
    ("pipeline", "Completed native battle initialization chain", "analysis/native-pipeline/battle-full-04/manifest.json",
     "Typed postload, original maps, construction and native tick probes."),
    ("battle", "Native battle loop evidence", "docs/native-battle-loop.md",
     "World, command, entity and component routes; complete harness execution."),
    ("entities", "Prepared native entity/component audit", "analysis/native-pipeline/battle-full-04/stage06-attempt001/native-entity-audit.json",
     "Six initial entities and 18 nonnull components; callback provenance."),
    ("replay", "Original native replay comparison", "analysis/native-pipeline/full-match-replay-audit.json",
     "One identical prepared scenario/seed; opaque Unicorn context bytes are not identical."),
    ("host-full", "Python/Rust complete-match comparison", "analysis/native-oracle/full-match-comparison.json",
     "12 saved regions, 70 architectural registers and allocator state match."),
    ("host-one", "Python/Rust one-update comparison", "analysis/native-oracle/world1-comparison.json",
     "Exact comparison for one original world update."),
    ("host-hundred", "Python/Rust 100-update comparison", "analysis/native-oracle/world100-comparison.json",
     "Exact comparison for 100 original world updates."),
    ("deck", "Native scenario and deck contract", "docs/native-deck-contracts.md",
     "Recovered row and factory contracts; arbitrary user deck selection is still separate work."),
    ("leaf", "Small Rust deck-operation comparisons", "analysis/native-engine-index/deck-leaf-rust-differential.json",
     "8,192 native comparisons across eight operations; not a combat rewrite."),
    ("card", "Native card command and runtime probe", "docs/native-human-card-command.md",
     "Live hand selection, descriptor, command creation, validation, queue and deployment."),
    ("card-proof", "Python/Rust card-play state comparison", "analysis/native-oracle/manual-card-comparison.json",
     "Independent adapters match after external input and 100 follow-through updates."),
    ("card-run", "External Mortar deployment evidence", "analysis/native-context/manual-card-evidence.json",
     "Cost, queue delay, entity creation and hand cycling through original engine code."),
    ("outcome", "Native winner and completion fields", "docs/native-rl-outcome.md",
     "Signed native winner after completion; no reward inferred from event strings or entity counts."),
    ("rl", "RL interface and reproduction guide", "rl/README.md",
     "Fixed snapshot/decks, partial observations, action discretization and benchmark scope."),
    ("smoke", "Native RL integration checks", "analysis/native-rl/smoke-test.json",
     "Gymnasium compliance, reset, truncation and a complete controlled match."),
    ("benchmark", "One-minute end-to-end RL benchmark", "analysis/native-rl/benchmark-60s.json",
     "12 complete games; no errors or truncations; unfinished thirteenth episode excluded."),
    ("machine", "Benchmark platform and code identities", "analysis/native-rl/benchmark-environment.json",
     "CPU/platform and exact measured code/binary SHA-256 values."),
    ("android", "Android runtime investigation", "docs/android-runtime.md",
     "Original APK lab setup and startup limits; not an Android release."),
    ("prototype", "Earlier approximate game and AI bridge", "docs/approximation-prototype.md",
     "Retained independent simulator; its functionality and throughput are separate from native recovery."),
    ("host", "Rust oracle instructions", "native-engine/README.md",
     "Prepared-snapshot native execution, CLI input probes and comparison commands."),
]
for _, _, path, _ in EVIDENCE:
    if not (ROOT / path).is_file():
        raise FileNotFoundError(f"Report evidence missing: {path}")


def cite(*keys):
    labels = []
    for key in keys:
        index = next(i for i, e in enumerate(EVIDENCE, 1) if e[0] == key)
        labels.append(f'<a class="cite" href="#ev-{key}" aria-label="Evidence {index}">E{index:02d}</a>')
    return '<span class="citations">' + " ".join(labels) + "</span>"


def link(path, label=None):
    if not (ROOT / path).is_file():
        raise FileNotFoundError(path)
    return f'<a href="../{esc(path)}">{esc(label or path)}</a>'


def code(text, language="shell"):
    return f'<div class="codebox"><span class="code-label">{esc(language)}</span><button class="copy" type="button">Copy</button><pre><code>{esc(text.strip())}</code></pre></div>'


def section(anchor, number, title, intro, body):
    return f'<section id="{anchor}" class="report-section"><header class="section-heading"><span class="section-number">{number:02d}</span><div><h2>{title}</h2><p class="section-intro">{intro}</p></div></header>{body}</section>'


benchmark = load("analysis/native-rl/benchmark-60s.json")
machine = load("analysis/native-rl/benchmark-environment.json")
smoke = load("analysis/native-rl/smoke-test.json")
ui_proof = load("analysis/native-ui/rust-verification/comparison.json")
cipher_proof = load("analysis/libg-cipher/direct-all-ranges-proof.json")
episodes = benchmark["episodes"]
avg_reset = statistics.mean(e["reset_seconds"] for e in episodes)
avg_episode = statistics.mean(e["duration_seconds"] for e in episodes)
median_episode = statistics.median(e["duration_seconds"] for e in episodes)
wins = [sum(e["winner_side"] == side for e in episodes) for side in (0, 1)]
ui_image = "data:image/png;base64," + base64.b64encode(
    (ROOT / "analysis/native-ui/recovered-controls.png").read_bytes()).decode()

sections = []
sections.append(section("read-first", 1, "What exists today", "The short answer to the questions that caused confusion.", f"""
<div class="callout strong"><strong>The original game engine runs. The full Rust combat rewrite is unfinished.</strong>
<p>The current system executes the supplied game's compiled ARM64 engine in Unicorn, with a Rust host controlling it and a Python Gymnasium environment above it. We have decompiled selected functions and recovered contracts. We do not have the complete original C/C++ source tree.</p></div>
<div class="two-col">
<div class="panel"><h3>Working now</h3><ul>
<li>Headless matches using the original native test harness and its built-in bots.</li>
<li>External card commands through original native validation and deployment.</li>
<li>A persistent Python <code>reset()</code>/<code>step(action)</code> interface with a simple external opponent.</li>
<li>Partial structured observations and native win/loss/draw rewards.</li>
<li>A measured end-to-end games-per-minute benchmark.</li></ul></div>
<div class="panel"><h3>Still separate work</h3><ul>
<li>A playable desktop battle with the original graphics, HUD and interactive human controls.</li>
<li>Arbitrary decks, famous presets and integration of the user's trained AI agents into this native path.</li>
<li>Unit positions, unit health and tower health in the RL observations.</li>
<li>A faithful Rust implementation of the complete combat simulation.</li>
<li>A phone release and an original-client-compatible offline server.</li></ul></div></div>
<p>The new RL prototype is usable for integration and throughput experiments. No neural-network policy was trained as part of this work. Its benchmark uses two simple affordable-card/random-position Python policies. {cite("rl","benchmark","host")}</p>
<div class="table-wrap"><table><thead><tr><th>Question</th><th>Current answer</th></tr></thead><tbody>
<tr><td>“Is this the real engine?”</td><td>Yes: the original compiled engine executes its battle and card logic, within the documented emulator and OS models.</td></tr>
<tr><td>“Do we have a C version?”</td><td>No complete recovered C/C++ source version. Decompiled pseudocode is analysis material; the running combat remains machine code.</td></tr>
<tr><td>“Is it all Rust?”</td><td>The host is Rust. The cipher decoder, sampled UI renderer and eight small deck operations also have Rust implementations. Combat is still ARM64 code.</td></tr>
<tr><td>“Can I use it for RL?”</td><td>Yes, through the new minimal Gymnasium wrapper, with fixed native decks/scenario and partial observations.</td></tr>
<tr><td>“Can I play through the original interface?”</td><td>Not yet. Programmatic card input works; the graphical battle client is unfinished.</td></tr>
</tbody></table></div>
"""))

sections.append(section("architecture", 2, "How the pieces fit together", "Original battle code at the bottom; newly written control layers above it.", f"""
<ol class="architecture" aria-label="Current runtime architecture">
<li><span class="layer-tag written">Python policy</span><strong>Agent + simple opponent</strong><p>Choose wait/card/position actions. No built-in bot decisions are invoked by these RL transitions.</p></li>
<li><span class="layer-tag written">Python adapter</span><strong>Gymnasium environment</strong><p>Numeric observations, action conversion, sparse rewards, reset/step and episode caps.</p></li>
<li><span class="layer-tag written">Rust adapter</span><strong>Persistent native session</strong><p>JSON-lines requests, hash-verified snapshot restore and calls into original functions.</p></li>
<li><span class="layer-tag third">Execution dependency</span><strong>Unicorn ARM64 emulator</strong><p>Executes the original instructions on this x86-64 desktop with explicit imported-function models.</p></li>
<li><span class="layer-tag original">Original game</span><strong>Decoded libg + native resources</strong><p>World updates, card validation, commands, entities, deployment and finalized match outcome.</p></li>
</ol>
<p>The original-resource UI viewer is a parallel component. Its asset playback is validated independently, but it is not connected to this battle loop. The earlier approximate Rust simulator is another separate component and is not used by the native RL environment. {cite("ui","prototype")}</p>
<h3>Why keep a reference execution path?</h3>
<p>A faithful Rust port needs something authoritative to compare against. Executing the original instructions supplies a reference for recovered operations. The current Python/Rust comparisons establish that two execution adapters agree for specific native workloads; they do not yet compare a full Rust combat implementation against the original.</p>
<details><summary>Two kinds of AI vs AI, with different workloads</summary>
<p><strong>Original harness:</strong> original function <code>11d6364</code> repeatedly calls the original bot/harness update. This path includes extra testing, serialization and validation work.</p>
<p><strong>RL loop:</strong> external Python policies submit card commands, then the Rust session advances the original world function. The builtin bot loop is not called. The trajectories, decisions and overhead differ, so the old warm harness timing and the new RL benchmark are not directly comparable speedups. {cite("battle","rl")}</p></details>
"""))

sections.append(section("scope", 3, "Goal and change of direction", "The desktop goal stayed the same; the first independent simulation did not meet the fidelity requirement.", f"""
<p>The requested destination is a desktop game with human vs AI and AI vs AI, per-side controller selection, eight-card decks or familiar presets, direct battle launch, the original Clash interface, and a fast headless engine for possible reinforcement learning. Android is a later target.</p>
<p>An earlier implementation provided a playable local Rust simulation, browser arena, deck selection and an AI-policy bridge. It consumed extracted game data and original card pictures, but approximated movement, targeting, spells and many special mechanics. The user explicitly rejected that as the intended solution. It remains under <code>src/</code>, <code>web/</code> and <code>run.sh</code> as a separate prototype. {cite("prototype","ledger")}</p>
<p>The subsequent work concentrated on the supplied native binary: decrypt it, execute its actual constructors and resource loading, recover battle and input contracts, and establish native runtime evidence. The most recent scope was deliberately narrower: make the simplest usable RL interface first and measure it before committing to the full Rust rewrite.</p>
<div class="callout"><strong>No project-completion percentage is asserted.</strong> Earlier “points” estimates were planning judgments, not measured progress. A working native reference and a minimal RL wrapper are major milestones; neither establishes a nearly finished native desktop client or combat rewrite.</div>
<h3>What was retained from the earlier effort</h3>
<p>Package extraction, source hashes, resource catalogs, card art and analysis tools remain useful. The old model's rules, UI and very high throughput are not evidence of original-engine equivalence and are not substituted into the new RL runtime.</p>
"""))

sections.append(section("inputs", 4, "Inputs, tools and provenance", "One pinned supplied game build, with derived outputs recorded separately.", f"""
<p>The supplied XAPK identifies <code>com.supercell.clashroyale</code>, version name/code <code>160402020</code>, with launcher <code>com.supercell.titan.GameApp</code>. REA inspected the package and Android declarations; native execution and recovery use the extracted ARM64 libraries and asset pack. This is analysis of that build, not a claim about today's live balance or client behavior. {cite("package","ledger")}</p>
<div class="table-wrap"><table><thead><tr><th>Tool / component</th><th>What it contributed</th></tr></thead><tbody>
<tr><td>morluto/rea, local REA 6.5.0</td><td>Requested shipped-artifact inspection workflow, Android/native entry inspection and retained evidence.</td></tr>
<tr><td>JADX and Ghidra 12.1.4 / Java 21</td><td>Android declarations and focused native disassembly/decompilation.</td></tr>
<tr><td>Capstone and ELF/unwind analysis</td><td>Exact instruction checks, references, original function bounds and vtable/callback resolution.</td></tr>
<tr><td>Unicorn 2.1.4</td><td>Bounded original ARM64 execution, memory/register checkpoints and runtime probes.</td></tr>
<tr><td>Rust / Cargo and Python</td><td>Cipher and asset implementations, native host, analysis tools, tests and the RL adapter.</td></tr>
<tr><td>Android emulator / ARM64 QEMU lab</td><td>Unmodified APK startup attempts and a relocated protection-companion memory capture.</td></tr>
<tr><td>Pinned SC5 parser and texture decoders</td><td>Original display trees, authored timelines and ASTC atlas recovery.</td></tr>
<tr><td>Gymnasium 1.4.0 / NumPy 2.5.4</td><td>Minimal standard RL API and typed observation/action spaces.</td></tr>
</tbody></table></div>
<p>Luna subagents helped trace native call contracts and outcome fields, implement persistent sessions, and review the adapter. Integration depended on exact assembly and runtime checks. Agent suggestions were treated as hypotheses until verified; several address, descriptor and reporting mistakes were corrected during review.</p>
<details><summary>Key input and output identities</summary>
<div class="hashes">
<p><strong>Supplied XAPK</strong><code>3dc0085605d495ee40b443084f7767ecc547d329be30b5f11884b0d345643f13</code></p>
<p><strong>Original ARM64 libg.so</strong><code>347afc57c8d168bb67d0defa488a89f76297025cac1e8544b5182bb7e70ff433</code></p>
<p><strong>Decoded analysis ELF</strong><code>79fe6612e20922d4c4086b1e6c939464202cefa7cb3b20344236529f941423d6</code></p>
<p><strong>Original assets.scdb</strong><code>792abc4536d56a4e7dc11fb825b578b4d6b3da43afa19747411f48d8d504c882</code></p>
</div><p>The original supplied APKs and libraries were preserved. Derived decoded files, explicit analysis interventions, virtual inputs and execution reports have separate identities. A matching hash establishes artifact identity, not automatically gameplay correctness. {cite("ledger","cipher","bootstrap")}</p>
</details>
"""))

sections.append(section("assets", 5, "Recovering the original assets and data", "This supplied usable source material without pretending that configuration files contain the entire engine.", f"""
<h3>Resource and card-data extraction</h3>
<p>All 645 packaged CSV/TOML resources were decoded, with packaged and decoded hashes and size/encoding checks. The catalog retains 375 raw card variants and 153 base-card entries, together with character, building, projectile, area-effect, buff and extension records. Original PNG card art was copied unchanged. Blank-name continuation rows and unused/event variants were retained in the raw catalog rather than silently collapsed. Package presence does not prove live availability. {cite("data","ledger")}</p>
<h3>Original UI structure</h3>
<p>Six selected SC5 display documents were decoded: the main UI, ability button, deck builder, spectator HUD, battle-end UI and arena UI. Recovery preserves child names, named exports, polygons, UVs, transforms, colors, masks, text metadata and frame labels. Atlas textures and six bundled TrueType fonts were retained. The main <code>ui.sc</code> contains 3,087 shapes, 4,871 movie clips, 2,695 text fields and 2,079 named exports. {cite("ui")}</p>
<p>Two upstream-parser mismatches had to be corrected: string indices needed direct zero-based lookup, and SCTX pixel type 208 required ASTC RGBA 6×6 decoding. Those corrections were checked against pinned primary upstream readers. Other selected atlases use ASTC 8×8.</p>
<figure class="asset-figure"><img src="{ui_image}" alt="Recovered original UI components: unbound player hand and elixir display, spectator HUD, replay panel, deck tabs, card slot and ability button." width="1100" height="1000" loading="lazy"><figcaption>Actual recovered asset renderings from the supplied package. These show static display objects and authoring placeholders; runtime text, card portraits and live battle-state bindings are absent. This is not a screenshot of a playable client.</figcaption></figure>
<p>A separate Rust display-list renderer reads the recovered trees and atlas PNGs. It traverses original clip frames, applies matrices/colors, rasterizes triangle strips and composites masks/blend modes. All 56 sampled states match an independent corrected Python parser/rasterizer pixel for pixel, including image origin: player and spectator HUDs, two mask-bearing components, and all 52 ability-button root frames. The authored ability timeline has 52 frames at 30 FPS. {cite("ui-proof")}</p>
<h3>SCDB schemas and original logic graphs</h3>
<p>The asset database is SQLite. Its 162 UI-schema and 155 UI-timeline blobs decode as JSON. Separately, 2,458 original logic/animation JSON assets were recovered. Important scope separation: all 509 game-object flows, 176 player flows and six behavior FSM assets in the identified flow families belong to <code>logic/autochess/</code>, associated with Merge Tactics. They are not recovered ordinary Royale troop AI. Another 477 chaos-data assets provide Royale extension configurations, while the native handlers remain essential. {cite("flow")}</p>
<div class="callout"><strong>Asset recovery is one layer of the client.</strong> Original text layout, resizing, state selection, game bindings, the battlefield renderer and interactive controls are not established by successful asset decoding or reference raster matches.</div>
"""))

sections.append(section("unpacking", 6, "Making the native engine readable", "The protected engine first had to become analyzable machine code.", f"""
<p>The original ARM64 <code>libg.so</code> contains protected executable ranges and strings. Early decompilation only exposed an initializer thunk, which was insufficient to recover combat. The initializer relies on the companion <code>libsupercell_clashroyale.so</code>. An isolated Android startup capture supplied relocated companion memory for controlled execution. {cite("cipher","ledger")}</p>
<ol class="steps">
<li><strong>Recover the companion's protected text.</strong> Executing its original routine at RVA <code>0x65bb90</code> changed 4,841,564 bytes inside a 4,860,456-byte region. This exposed the loader callbacks and binding-codec path.</li>
<li><strong>Recover the binding metadata.</strong> The signed-envelope asset contains 416 protected bindings. Original codec execution recovered the exact 672-byte metadata requested by libg; a later audit ties all 416 records to retained original decoder outputs. The envelope signature itself was not verified.</li>
<li><strong>Execute and translate the real loader.</strong> Bounded control-flow recovery and instruction translation made its arithmetic practical to verify. Native checkpoints were compared across scalar/vector registers, flags and mapped memory.</li>
<li><strong>Implement the recovered cipher in Rust.</strong> Speck64/128-CBC with 27 expanded round keys reproduced the original loader's inverse operations and CBC chaining. Source and metadata hashes are checked before output.</li>
<li><strong>Verify all five ranges.</strong> The Rust decoder's 19,190,488 output bytes match completed original native loader execution, with zero range differences.</li>
</ol>
<p>The resulting analysis ELF retains the original layout and relocation slots and makes native entry points, parsers, battle functions and UI binding code readable. This is recovered machine code; it is not the original C++ source tree. {cite("bindings","cipher-proof")}</p>
<details><summary>Analysis intervention and the meaning of the decryption timing</summary>
<p>The captured companion had not completed initialization: its metadata tree was empty. The adapter supplied independently recovered original decoded bytes at the verified metadata lookup boundary. That intervention is recorded; it does not claim a successful unmodified Android startup.</p>
<p>The Rust decoder processed the protected ranges in approximately 0.0813 seconds in a retained run. That measures decryption and output hashing. It does not measure battle speed, Rust combat speed or RL throughput.</p>
<p>The loader translation is a separate verification/acceleration artifact. Its generated C++ instruction-level implementation is not a recovered C version of the whole game. {cite("cipher")}</p></details>
"""))

sections.append(section("bootstrap", 7, "Bootstrapping a real native battle", "Native code needs correctly initialized data, services and objects before a tick is meaningful.", f"""
<p>After decoding, focused analysis used original unwind/FDE bounds, exact instruction bytes and source hashes. Ghidra display addresses include a <code>0x100000</code> image base; report RVAs below exclude it. ARM64 hidden structure returns through X8 were checked against assembly rather than guessed from decompiler argument order. {cite("focus")}</p>
<h3>Native loading chain</h3>
<ol class="steps">
<li><strong>Global constructors.</strong> Execute all 763 original game constructors, together with the required packaged C++ and Sentry constructor paths. Sentry code is a dependency here; telemetry delivery was not initialized.</li>
<li><strong>Outer context and asset service.</strong> The bare context factory leaves a required asset holder absent. The original outer manager route initializes the context and holder used by later native jobs.</li>
<li><strong>Original parsers and tables.</strong> Original CSV/TOML/JSON functions parse the bytes and construct original native rows. Preparation covers 164 nonnull tables; localization and flag-1 typed postload establish cross-table references and cached records.</li>
<li><strong>Original SCDB types and prefabs.</strong> Explicit original type registrars and filesystem-provider setup allow the native SQLite and prefab machinery to read the 23,543,808-byte database. A real projectile-prefab lookup succeeds.</li>
<li><strong>Original object factory and services.</strong> A bounded source-hashed startup instruction region installs original factory providers. The CSV-loader service selects its supported synchronous <code>(0,0)</code> configuration for this offline execution.</li>
<li><strong>Maps and battle construction.</strong> All 209 Location map loads from 202 original CSVs complete. The native Normal harness constructs configuration, decks, world, battle, command manager, arena, tilemap and initial objects.</li>
</ol>
<p>The completed chain is retained in <code>analysis/native-pipeline/battle-full-04/</code>. Its prepared battle contains six native entities and 18 nonnull components. The audited entity/component tick callbacks point into executable regions of the original engine. Empty objects or fabricated vtables were not used to claim battle success. {cite("bootstrap","pipeline","entities")}</p>
<h3>OS and runtime models needed to get there</h3>
<div class="two-col"><div class="panel"><h4>Files, imports and time</h4><p>Read-only, hash-bound virtual files and mappings supply real bytes. Native SQLite and parsers still execute. Observed allocation, memory, string, formatting, clock and selected Android/Bionic calls are modeled explicitly. Unknown imports/conversions stop execution.</p></div>
<div class="panel"><h4>Workers, TLS and allocation</h4><p>Cooperative ARM64 workers execute original job callbacks with separate stacks, TLS, mutex ownership, condition waits and saved contexts. A reusable aligned guest heap replaces the insufficient bump-only analysis allocator.</p></div></div>
<details><summary>Important modeling choices and limitations</summary>
<ul>
<li>The model reports 4 KiB pages and one configured/online virtual processor using Android's actual sysconf constants. A prior page-size response to a CPU-count query was corrected, and affected loading was replayed.</li>
<li>The clock is deterministic: supported query results advance by a fixed virtual amount. Native performance and original phone OS timing are not reproduced.</li>
<li>The read-only asset boundary returns missing only after verifying absence in all supplied APK splits. Present assets needing an unsupported AAsset API cause explicit failure.</li>
<li>The original 256 MiB analysis heap was expanded with allocation reuse and support for additional arenas. Retained bootstrap snapshots include legacy allocation bookkeeping; that count is not a measurement of the original game's live RAM requirement.</li>
<li>Busy-wait preemption, timed waits and full TLS-destructor teardown are not a complete modeled Android threading runtime.</li>
<li>Bounded startup-region execution, recovered metadata lookup and the chosen offline service settings are explicit analysis interventions. No complete Android runtime-equivalence claim follows.</li>
</ul>{cite("bootstrap")}
</details>
"""))

sections.append(section("native-battle", 8, "Battle execution and external card control", "The first full native match and the path needed for an agent to act.", f"""
<h3>The original update path</h3>
{code('11d6398  original bot/harness tick\n  → 10ca81c  original world update\n    → d75ed0  command scheduling and execution\n    → d33f74  battle update\n      → f8cd90  original entity/component update\n  → paired 32-bit world counters advance', 'Original function RVAs')}
<p>One world update advanced the paired counters from 0 to 1. One original bot/harness update advanced them to 2 and returned no terminal event. From that prepared tick-2 state, the original complete-match loop ran another 1,352 ticks to counters <strong>1,354 / 1,354</strong>, with 11 final entities, 39 nonnull components and no invalid memory accesses in the recorded run. Native terminal event code 0 came from the completion path; the separate 36,000-step limit would produce code 3. {cite("battle")}</p>
<h3>Recovering a native card-play command</h3>
<p>An agent needs the original live hand selection, not an invented integer-to-object layout. The recovered route obtains the native side object, actor and current hand wrapper, asks original code to create the selection descriptor, validates the target, and constructs a native <code>0x58</code>-byte play command. Native setters provide its card and coordinates; the original submit helper attaches the actor/current tick, validates it and queues it.</p>
<div class="event-strip"><div><b>7 → 3</b><span>Elixir after Mortar cost</span></div><div><b>0 → 1 → 0</b><span>Native command queue</span></div><div><b>6 → 7</b><span>Entities after deployment</span></div></div>
<p>The concrete probe selected side 0, hand slot 0: Mortar, global ID <code>27000002</code>, cost 4, at world tick 102. Native coordinates <code>(4500,4500)</code> passed validation. Queue count increased to 1; the first world update spent elixir from 7 to 3. Deployment was delayed: after 10 further updates the command remained queued; by 20 it had completed and a seventh entity existed. The hand cycled, and callbacks for the seven entities and 21 nonnull components passed the original-code audit. No bot decision helper was invoked during this input probe. {cite("card","card-run")}</p>
<details><summary>Exact card-input ABI and ownership details</summary>
{code('side    = d35d5c(battle, side_index)\nactor   = d3c980(battle, side_index)\nwrapper = f9dd98(side, hand_slot)\ne80bbc(wrapper, side, 1, X8=&descriptor)\nproxy   = e80680(&descriptor)\nd42214(battle, x, y, proxy, 0)       // placement check\n\nd774ac(command, context)            // original constructor\nd77c8c(command, &descriptor)        // descriptor, not proxy\nd77a14(command, x, y)\n11d680c(harness, &command_pointer, actor)', 'Verified native call route')}
<p>The descriptor reserves 24 bytes, but the command setter copies exactly 20: a 16-byte vector and one 32-bit field. The proxy returned for target validation is not the descriptor pointer. Accepted commands transfer to native queue ownership and must not be immediately freed by the adapter. Temporary pointer storage is separate from the command allocation.</p>
<p>Acceptance reporting was tightened to require the actual queue to grow by one, the submitted pointer to remain the command pointer, and the command allocation to be live. Passing preliminary placement/elixir checks alone is not reported as acceptance. The helper's unspecified X0 residue is not treated as a return error code.</p>
</details>
<p>Insufficient-elixir and invalid-target cases were exercised and left the queue unchanged. A side-1 submission was also accepted. The per-side explicit-deck factories and scenario row fields are recovered, but arbitrary deck selection is not exposed in the minimal RL wrapper. {cite("deck","card")}</p>
"""))

sections.append(section("verification", 9, "What was verified, and what that proves", "The report distinguishes original instruction execution from a Rust implementation of combat.", f"""
<div class="table-wrap"><table><thead><tr><th>Check</th><th>Observed result</th><th>Practical limit</th></tr></thead><tbody>
<tr><td>Rust protected-range decoder</td><td>All five ranges equal original loader output, 19,190,488 bytes.</td><td>Verifies decoding; not battle semantics.</td></tr>
<tr><td>Rust UI asset renderer</td><td>{ui_proof["exact_matches"]}/{ui_proof["cases"]} sampled states match every pixel and origin.</td><td>Independent asset-renderer reference; not the official executing client.</td></tr>
<tr><td>Eight Rust deck leaf operations</td><td>8,192 original-instruction comparisons, zero mismatches.</td><td>Small field/array operations; no full hand, movement or damage implementation.</td></tr>
<tr><td>Repeated original AI harness</td><td>Saved memory, architectural registers, allocator state and import counts equal.</td><td>One identical prepared scenario and seed.</td></tr>
<tr><td>Python vs Rust native adapters</td><td>One update, 100 updates, a complete AI match and external card input all match.</td><td>Both execute original code; this is adapter parity, not whole-game Rust-port parity.</td></tr>
<tr><td>Gymnasium integration</td><td>Reset, deterministic transitions, truncation and a complete controlled match pass.</td><td>Partial observations and fixed native initialization.</td></tr>
</tbody></table></div>
<p>The full-match and manual-card comparisons rehash all <strong>12 saved memory regions</strong> and compare <strong>70 architectural registers</strong> plus complete allocator metadata. Selected equality results are embedded in the evidence appendix. {cite("cipher-proof","ui-proof","leaf","host-one","host-hundred","host-full","card-proof","smoke")}</p>
<h3>Checkpoint and replay boundaries</h3>
<p>Checkpoints bind original source, saved memory, imports, allocation metadata, register state and supporting runtime information. The Rust adapter verifies the exact Unicorn shared-library identity and restores architectural registers rather than treating an opaque serialized CPU-context blob as a portable public ABI.</p>
<p>In two original native replays the opaque Unicorn CPU-context byte strings differ, while every architectural register and saved memory-region hash agrees. The report therefore states architectural/state equality and separately records the opaque-context difference. It does not claim the opaque blobs are identical. {cite("replay")}</p>
<details><summary>Corrections that matter to interpretation</summary>
<ul>
<li><strong>Decompiler addresses:</strong> Ghidra's image base must be subtracted arithmetically; an early hand-slot getter candidate was corrected to raw RVA <code>0xf9dd98</code>.</li>
<li><strong>Hidden return buffers:</strong> ARM64 X8 structure-return destinations are not ordinary first parameters. Putting scratch storage below SP can overlap a callee frame; proper allocated storage is used.</li>
<li><strong>Descriptor vs proxy:</strong> <code>d77c8c</code> receives the selection descriptor; <code>e80680</code>'s proxy is for placement checks.</li>
<li><strong>Native row identity:</strong> the global-ID getter uses offset <code>0x40</code>, not decimal 40. A real Mortar ID/name/cost assertion covers this.</li>
<li><strong>Formatted text vs actual error:</strong> the harness formats a card-event “too many” message before testing whether count exceeds 4,096. Formatting-call counts are not failure counts. The same applies to the pointer-identity message constructed before a pointer comparison.</li>
<li><strong>Winner vs terminal event:</strong> event code 0 means the completion route; winner is a separate signed field. Equal cached side counts do not establish a draw.</li>
</ul>
</details>
"""))

sections.append(section("rl-interface", 10, "The minimal reinforcement-learning environment", "The new deliverable: a real reset/step loop around the recovered engine.", f"""
<h3>Session and protocol</h3>
<p><code>NativeSession</code> owns the Unicorn instance and stable boxed API/hook state. It restores the original snapshot once per reset, then keeps the process and engine alive between steps. A single reusable descriptor scratch allocation avoids allocating one for every observed hand card. Session methods expose native world stepping, hand observations, card submission, completion and an optional original-harness reference call.</p>
<p>The <code>native-rl</code> executable accepts sequential JSON lines: <code>reset</code>, <code>step</code>, <code>observe</code> and <code>close</code>. Actions for both sides are parsed before execution; at most one action per side is accepted per decision step. Native execution failures discard the session and require reset. The adapter does not silently substitute an approximate engine or fabricate success. {cite("rl","host")}</p>
<h3>Action space</h3>
{code('MultiDiscrete([5, 36, 64])\naction = [card, x_cell, y_cell]\n\ncard = 0      → wait\ncard = 1..4   → current hand slot 0..3\nnative x      = 500*x_cell + 250\nnative y      = 500*y_cell + 250', 'Gymnasium action encoding')}
<p>The position encoding samples the range obtained from original width/height getters and the original bot's width/height × 500 coordinate use. It is a coarse action discretization, not an observed occupancy grid. Native rules decide whether a requested play is legal. Current availability means a live native hand card exists; affordability and placement are separate checks.</p>
<h3>Observation space</h3>
<div class="table-wrap"><table><thead><tr><th>Field</th><th>Shape</th><th>Native meaning</th></tr></thead><tbody>
<tr><td><code>tick</code></td><td>(1,)</td><td>Original world counter.</td></tr>
<tr><td><code>elixir</code></td><td>(2,)</td><td>Current clamped elixir for each side.</td></tr>
<tr><td><code>card_ids</code></td><td>(2,4)</td><td>Original current hand identities; zero when absent.</td></tr>
<tr><td><code>card_costs</code></td><td>(2,4)</td><td>Costs decoded by the native selection getter.</td></tr>
<tr><td><code>card_available</code></td><td>(2,4)</td><td>Whether a hand wrapper resolves to a native card row.</td></tr>
<tr><td><code>entity_count</code></td><td>(1,)</td><td>Current total native entities, without per-unit features.</td></tr>
</tbody></table></div>
<div class="callout"><strong>This is a partial-observation starting point.</strong> Unit locations, troop and tower health, images, ability controls and complete combat state are absent. The API can exercise an RL pipeline, but its information is limited for learning tactical combat. No trained-agent quality or learning convergence is claimed.</div>
<h3>Opponent, reset and episode semantics</h3>
<p>The controlled agent is side 0. The default opponent is a simple Python policy selecting affordable cards and random positions for side 1; an optional wait opponent submits no cards. Default action spacing is 20 original world ticks. There is no real-time rendering pace.</p>
<p><code>reset(seed=...)</code> seeds Python sampling. The native engine resets to the same prepared tick-2 snapshot with the same native RNG state and decks. Different Python seeds produce different action trajectories; they do not randomize the engine's underlying initial scenario.</p>
<p>Default episode cap is 7,200 native ticks. A genuine completion sets <code>terminated=True</code>. Reaching the cap first sets <code>truncated=True</code> and returns zero reward; it is not counted as a completed game. Further steps require reset. {cite("rl","smoke")}</p>
<h3>Reward recovered from the original finalizer</h3>
<p>The original finalization routine <code>d3e360</code> sets completion at <code>battle+0x1e0</code> and stores a signed result at <code>battle+0x1b8</code>: 0 means side 0 wins, 1 means side 1 wins, and −1 means draw. Native later player lookup reads this same field. The wrapper reads it only after completion and rejects unsupported values. Side 0 receives +1, −1 or 0 accordingly; before completion reward is zero. {cite("outcome")}</p>
<p>A full controlled smoke-test episode ended at native tick <strong>{smoke["complete_game"]["tick"]}</strong>, winner side <strong>{smoke["complete_game"]["winner_side"]}</strong>, producing side-0 reward <strong>{smoke["complete_game"]["side0_reward"]:.0f}</strong> after {smoke["complete_game"]["decision_steps"]} decisions. Gymnasium's deterministic-transition checks passed. {cite("smoke")}</p>
"""))

chart_parts = ['<svg viewBox="0 0 790 465" role="img" aria-labelledby="chart-title chart-desc"><title id="chart-title">Measured wall time for each of the 12 completed games</title><desc id="chart-desc">Stacked bars show reset and remaining episode wall time. Values come from the retained one-minute benchmark.</desc>']
for second in range(9):
    x = 88 + second * 72
    chart_parts.append(f'<line class="gridline" x1="{x}" y1="18" x2="{x}" y2="414"/><text class="axis-label" x="{x}" y="441" text-anchor="middle">{second}s</text>')
for index, ep in enumerate(episodes):
    y = 26 + index * 32
    total, reset = ep["duration_seconds"] * 72, ep["reset_seconds"] * 72
    chart_parts.append(f'''<g class="episode" tabindex="0" role="button" aria-label="Episode {index+1}: {ep["duration_seconds"]:.3f} seconds; select for details" data-seed="{ep["seed"]}">
    <title>Episode {index+1}: {ep["duration_seconds"]:.3f}s total, {ep["reset_seconds"]:.3f}s reset; winner side {ep["winner_side"]}</title>
    <rect class="row-hit" x="0" y="{y-5}" width="790" height="30" rx="5"/>
    <text class="episode-label" x="62" y="{y+14}" text-anchor="end">{index+1:02d}</text>
    <rect class="bar-total" x="88" y="{y}" width="{total:.2f}" height="19" rx="3"/>
    <rect class="bar-reset" x="88" y="{y}" width="{reset:.2f}" height="19" rx="3"/>
    <rect class="bar-play" x="{88+reset:.2f}" y="{y}" width="{total-reset:.2f}" height="19"/>
    <text class="bar-value" x="{98+total:.2f}" y="{y+14}">{ep["duration_seconds"]:.2f}s</text></g>''')
chart_parts.append("</svg>")
chart_svg = "".join(chart_parts)
episode_rows = "".join(
    f'<tr data-seed="{e["seed"]}" data-duration="{e["duration_seconds"]}" data-reset="{e["reset_seconds"]}" data-tick="{e["final_tick"]}">'
    f'<td>{i:02d}</td><td>{e["seed"]}</td><td>{e["duration_seconds"]:.3f}s</td><td>{e["reset_seconds"]:.3f}s</td><td>{e["steps"]}</td><td>{e["final_tick"]:,}</td>'
    f'<td><span class="winner side-{e["winner_side"]}">Side {e["winner_side"]}</span></td><td>{e["reward"]:+.0f}</td></tr>'
    for i, e in enumerate(episodes, 1))
sections.append(section("benchmark", 11, "Measured games per minute", "An actual wall-clock run through the new RL interface, not a theoretical speed estimate.", f"""
<div class="metrics benchmark-metrics">
<div><strong>{benchmark["completed_games"]}</strong><span>Completed games</span></div>
<div><strong>{benchmark["completed_games_per_minute"]:.2f}</strong><span>Games per minute</span></div>
<div><strong>{benchmark["elapsed_seconds"]:.3f}s</strong><span>Measured elapsed</span></div>
<div><strong>0 / 0</strong><span>Errors / truncations</span></div></div>
<p>The requested window was 60 seconds. Twelve games completed in 59.627 seconds of summed episode time; a thirteenth reset started near the deadline and completed without taking any decision steps. Actual measured elapsed time, including that unfinished reset and shutdown, was <strong>{benchmark["elapsed_seconds"]:.3f} seconds</strong>. Only the twelve genuine completions enter the throughput numerator. {cite("benchmark")}</p>
<div class="chart-card"><div class="chart-toolbar"><div><h3>Where each game's wall time went</h3><p>Episode total includes its reset.</p></div><div class="segmented" aria-label="Chart display"><button type="button" data-view="stacked" aria-pressed="true">Breakdown</button><button type="button" data-view="total" aria-pressed="false">Total</button></div></div>
<div class="legend"><span><i class="swatch reset"></i>Snapshot reset</span><span><i class="swatch play"></i>Simulation + actions + interface</span></div>
<div id="episode-chart" data-view="stacked">{chart_svg}</div>
<p id="chart-detail" class="chart-detail" aria-live="polite">Select a bar to inspect its episode. Table headers below can sort the recorded rows.</p></div>
<div class="table-wrap"><table id="episode-table"><caption>All 12 native completions; reward is from side 0's perspective.</caption><thead><tr>
<th scope="col">Game</th><th scope="col"><button class="sort" data-sort="seed">Policy seed ↕</button></th><th scope="col"><button class="sort" data-sort="duration">Wall time ↕</button></th><th scope="col"><button class="sort" data-sort="reset">Reset ↕</button></th><th scope="col">Decisions</th><th scope="col"><button class="sort" data-sort="tick">Final tick ↕</button></th><th scope="col">Winner</th><th scope="col">Reward</th></tr></thead><tbody>{episode_rows}</tbody></table></div>
<div class="two-col"><div class="panel"><h3>Recorded workload</h3><dl class="facts">
<div><dt>Decision steps</dt><dd>{benchmark["decision_steps"]:,}</dd></div><div><dt>Native ticks</dt><dd>{benchmark["native_ticks"]:,}</dd></div><div><dt>End-to-end ticks / second</dt><dd>{benchmark["native_ticks_per_second"]:.2f}</dd></div><div><dt>Accepted card requests</dt><dd>{benchmark["accepted_cards"]} / {benchmark["card_attempts"]}</dd></div><div><dt>Side 0 / side 1 wins</dt><dd>{wins[0]} / {wins[1]}</dd></div></dl></div>
<div class="panel"><h3>Timing distribution</h3><dl class="facts"><div><dt>Mean completed episode</dt><dd>{avg_episode:.3f}s</dd></div><div><dt>Median episode</dt><dd>{median_episode:.3f}s</dd></div><div><dt>Shortest / longest</dt><dd>{min(e["duration_seconds"] for e in episodes):.3f}s / {max(e["duration_seconds"] for e in episodes):.3f}s</dd></div><div><dt>Mean reset, completed games</dt><dd>{avg_reset:.3f}s</dd></div><div><dt>Completed-game reset time</dt><dd>{sum(e["reset_seconds"] for e in episodes):.3f}s total</dd></div></dl></div></div>
<h3>What this number includes</h3>
<p>One native process on the local Intel Core i5-13420H under current desktop load, with full snapshot reloads, observations, Python policy decisions, JSON IPC, native validation and original world simulation. The benchmark uses two affordable-random policies, 20 native ticks per decision, fixed native scenario/decks, and no rendering or neural training. Initial Rust compilation and Python import time are excluded. {cite("machine","benchmark")}</p>
<p>Roughly 1.7 seconds per episode are spent reloading the prepared snapshot. There is a clear cost to investigate before deciding that a full combat rewrite is necessary. No speedup from removing it has been measured, and the remaining simulation cost would still exist. Games with other decks, policies, lengths or native branches may have different throughput.</p>
<details><summary>Timing numbers that measure different things</summary>
<div class="table-wrap"><table><thead><tr><th>Recorded number</th><th>What was measured</th><th>How to interpret it</th></tr></thead><tbody>
<tr><td>0.0813s</td><td>Rust decryption of protected ranges, including output hashing.</td><td>Binary recovery speed, not game speed.</td></tr>
<tr><td>13.93s</td><td>Earlier warm original bot/harness full-match call, 1,352 additional ticks.</td><td>ARM64 emulation with native harness overhead; snapshot load excluded.</td></tr>
<tr><td>11.71 games/min</td><td>Current end-to-end external-policy RL loop.</td><td>Resets and interface included; different native workload and trajectories.</td></tr>
<tr><td>190 matches/s</td><td>Earlier independent approximate Rust simulator's documented default workload.</td><td>Separate rules and benchmark scope; not original-game fidelity or a valid equivalent-engine speed comparison.</td></tr>
</tbody></table></div>{cite("cipher","host","prototype")}</details>
"""))

sections.append(section("boundaries", 12, "Limits, unresolved work and next steps", "Working execution is a foundation for further recovery, not proof of universal client or game parity.", f"""
<div class="table-wrap"><table><thead><tr><th>Area</th><th>Status</th><th>Remaining work / boundary</th></tr></thead><tbody>
<tr><td>Original binary execution</td><td><span class="status verified">Working</span></td><td>Source-bound tested offline scenario; imported functions and startup dependencies are explicitly modeled.</td></tr>
<tr><td>Basic RL loop</td><td><span class="status verified">Working, limited</span></td><td>One fixed native initialization and partial observations; no learning run or generalized evaluation.</td></tr>
<tr><td>Deck/controller choice</td><td><span class="status partial">Partial recovery</span></td><td>Native contracts exist; arbitrary decks, famous presets and trained-policy integration are not exposed by the current native RL interface.</td></tr>
<tr><td>Original UI assets</td><td><span class="status verified">Recovered / sampled parity</span></td><td>Asset viewer works; live battle graphics, layout, text, bindings and interactive controls remain.</td></tr>
<tr><td>Rust combat rewrite</td><td><span class="status pending">Unfinished</span></td><td>Recover and port movement, targeting, collision, damage, projectiles, special mechanics and rounding with native comparisons.</td></tr>
<tr><td>Android phone version</td><td><span class="status pending">Unfinished</span></td><td>Original startup remains blocked in the labs; no phone package for this project.</td></tr>
<tr><td>Original-client offline server</td><td><span class="status pending">Unimplemented</span></td><td>The local JSON-lines RL process is not an original-game-compatible server protocol.</td></tr>
</tbody></table></div>
<h3>Android lab results</h3>
<p>The x86-64 Android lab accepted the ARM64 APK split through Android's native bridge but exited during protection startup before libg was mapped; the exact cause is unresolved. A separate ARM64 QEMU system booted and launched unchanged APK splits, but the protection companion's JNI_OnLoad returned JNI_ERR before loading libg. The large asset pack was not yet installed in that separate baseline lab. No successful phone/client startup is claimed. {cite("android")}</p>
<h3>Practical continuation order</h3>
<ol class="steps">
<li><strong>Improve observations.</strong> Recover native unit identity/team, position, health and tower state to give an agent meaningful combat feedback.</li>
<li><strong>Make initial conditions configurable.</strong> Safely expose native deck and scenario initialization, plus engine RNG variation, without fabricated row layouts.</li>
<li><strong>Profile the existing runtime.</strong> Investigate snapshot reload and interface costs, then measure before and after. Parallelism and reset changes require their own correctness checks.</li>
<li><strong>Connect controllers and presentation.</strong> Integrate desired policies and original UI/battle rendering for a genuinely playable desktop client.</li>
<li><strong>Port combat components to Rust progressively.</strong> Use original execution as the reference, with meaningful deterministic and edge-case comparisons, before making a performance/fidelity claim.</li>
</ol>
<p>No official account or live online match was used for these tests. The current evidence is local offline execution of supplied artifacts. Broad parity with the full official client, all cards, all game modes and all Android/runtime behavior remains unestablished.</p>
"""))

run_commands = """uv pip install --python tools/reverse-env/bin/python -r rl/requirements.txt
cargo build --manifest-path native-engine/Cargo.toml --release --offline --bin native-rl
tools/reverse-env/bin/python -m rl.smoke_test
tools/reverse-env/bin/python -m rl.benchmark --seconds 60 --out analysis/native-rl/benchmark-repeat.json"""
gym_example = """from rl import NativeClashEnv

with NativeClashEnv(ticks_per_step=20) as env:
    observation, info = env.reset(seed=1)
    done = False
    while not done:
        action = env.action_space.sample()  # replace with your policy
        observation, reward, terminated, truncated, info = env.step(action)
        done = terminated or truncated"""
sections.append(section("reproduce", 13, "Run it and find the implementation", "Commands are for the existing local workspace and its retained native artifacts.", f"""
<p>Run from <code>engine/</code> in the clash-royale-ai repository. The prepared snapshot, original extracted libraries and resources are required; this HTML file alone is not a distributable game or a complete runtime bundle.</p>
{code(run_commands)}
<p>The repeat benchmark uses a different output filename to preserve the original measured report.</p>
{code(gym_example, "Python")}
<div class="table-wrap"><table><thead><tr><th>Path</th><th>Role</th></tr></thead><tbody>
<tr><td>{link("native-engine/src/oracle.rs")}</td><td>Unicorn C-API adapter, restored guest allocator/imports, native session, commands, observations and outcome.</td></tr>
<tr><td>{link("native-engine/src/bin/native_rl.rs")}</td><td>Persistent JSON-lines reset/step/observe/close process.</td></tr>
<tr><td>{link("rl/native_clash.py")}</td><td>Gymnasium spaces, opponent sampling, action conversion, rewards and episode semantics.</td></tr>
<tr><td>{link("rl/benchmark.py")}<br>{link("rl/smoke_test.py")}</td><td>Actual timed benchmark and native integration checks.</td></tr>
<tr><td>{link("native-engine/src/bin/native_oracle.rs")}</td><td>Earlier controlled CLI probes and audit exports.</td></tr>
<tr><td>{link("tools/native_unpack.py")}<br>{link("tools/native_pipeline.py")}</td><td>Original loader/bootstrap execution, checkpoints and bounded stage continuation.</td></tr>
<tr><td>{link("tools/native_threads.py")}<br>{link("tools/native_heap.py")}<br>{link("tools/native_vfs.py")}</td><td>Explicit worker, allocator and read-only file models used during recovery.</td></tr>
<tr><td>{link("tools/native_oracle_compare.py")}<br>{link("tools/native_entity_audit.py")}</td><td>Memory/register/allocator comparison and native callback provenance checks.</td></tr>
<tr><td>{link("tools/libg_cipher_decode.rs")}<br>{link("native-ui-viewer/README.md")}</td><td>Rust protected-range decoder and independent original-asset UI viewer.</td></tr>
<tr><td>{link("tools/build_progress_report.py")}</td><td>Rebuild this standalone report from retained data and results.</td></tr>
</tbody></table></div>
<p>Full implementation and protocol details are in {link("rl/README.md", "the RL guide")} and {link("native-engine/README.md", "the native host guide")}. This report embeds its benchmark data, charts, UI evidence image and styles; its source-document links require the surrounding project files.</p>
"""))

evidence_rows = []
for index, (key, title, path, limit) in enumerate(EVIDENCE, 1):
    evidence_rows.append(f'<tr id="ev-{key}" data-evidence><td class="evidence-id">E{index:02d}</td><td><strong>{esc(title)}</strong><a class="evidence-path" href="../{esc(path)}">{esc(path)}</a></td><td>{esc(limit)}</td></tr>')
embedded_keys = ["benchmark", "smoke", "machine", "host-full", "card-proof", "replay", "cipher-proof", "bindings"]
embedded_views = []
for key in embedded_keys:
    record = next(e for e in EVIDENCE if e[0] == key)
    data = load(record[2])
    embedded_views.append(f'<details><summary>{esc(record[1])} — embedded JSON</summary><pre class="json-view">{esc(json.dumps(data, indent=2))}</pre></details>')
sections.append(section("evidence", 14, "Evidence index and embedded records", "Each claim links to retained local sources. Selected primary reports are also embedded for offline reading.", f"""
<label class="search-label" for="evidence-search">Filter evidence by topic, filename or limitation</label>
<input id="evidence-search" type="search" placeholder="Try: winner, decoder, reset, UI, comparison…" autocomplete="off">
<p class="muted" id="evidence-count" aria-live="polite">{len(EVIDENCE)} evidence records</p>
<div class="table-wrap"><table class="evidence-table"><thead><tr><th>ID</th><th>Record</th><th>What it supports / limits</th></tr></thead><tbody>{"".join(evidence_rows)}</tbody></table></div>
<div class="embedded-records">{"".join(embedded_views)}</div>
<h3>Reading the evidence correctly</h3>
<p>Older ledgers retain historical status and failed or superseded attempts. This consolidated report uses later successful runs where explicitly cited. A source hash establishes the artifact used; a successful probe establishes the exercised behavior. Neither automatically establishes behavior of untested cards, modes, native branches, original client graphics or a future Rust port.</p>
<dl class="glossary"><div><dt>XAPK / APK</dt><dd>The supplied Android package bundle and its split archives.</dd></div><div><dt>ELF / ARM64</dt><dd>The native library format and instruction architecture of the shipped engine.</dd></div><div><dt>RVA / FDE</dt><dd>Relative virtual address and unwind-defined original function extent, used to anchor analysis.</dd></div><div><dt>Oracle / adapter parity</dt><dd>Original execution used as a reference; matching control adapters are different from matching a rewritten combat engine.</dd></div><div><dt>Native tick</dt><dd>One original world-counter update; benchmark counts it directly rather than pacing a rendered client.</dd></div><div><dt>Termination / truncation</dt><dd>Engine-completed outcome versus an external episode cap; only native completions count as completed games here.</dd></div></dl>
"""))

CSS = """
:root{--paper:#f4f3ee;--white:#fff;--ink:#182b39;--muted:#5c6b73;--line:#dce1df;--teal:#147568;--teal-soft:#e6f3ef;--amber:#c3692b;--amber-soft:#faf0e5;--blue:#3d5f93;--shadow:0 12px 35px #172a3910;--mono:ui-monospace,SFMono-Regular,Consolas,monospace}
*{box-sizing:border-box}html{scroll-behavior:smooth;scroll-padding-top:88px}body{margin:0;color:var(--ink);background:var(--paper);font:16px/1.72 system-ui,-apple-system,BlinkMacSystemFont,"Segoe UI",sans-serif}a{color:var(--teal);text-underline-offset:3px;overflow-wrap:anywhere}a:hover{color:#064b42}button,input{font:inherit}button{cursor:pointer}button:focus-visible,a:focus-visible,input:focus-visible,summary:focus-visible,.episode:focus-visible{outline:3px solid #d5803d;outline-offset:4px}p{margin:0 0 1.15em}ul,ol{padding-left:1.4em;margin:.5em 0 1.4em}li{margin:.35em 0}strong{font-weight:650}code{font-family:var(--mono);font-size:.86em;overflow-wrap:anywhere;background:#edf1f0;padding:.12em .3em;border-radius:4px}pre code{background:none;padding:0;font-size:12.5px}h1,h2,h3,h4{line-height:1.18;letter-spacing:-.025em;margin:0 0 .6em}h1,h2{font-family:Georgia,"Times New Roman",serif;font-weight:500}h1{font-size:clamp(42px,4.4vw,67px);max-width:920px;line-height:1.055;letter-spacing:-.045em}h2{font-size:clamp(30px,3vw,40px)}h3{font-size:22px;margin-top:1.7em}h4{font-size:17px}.muted{color:var(--muted)}.small{font-size:13px}.masthead{height:66px;display:flex;align-items:center;justify-content:space-between;padding:0 38px;background:#fffffff5;border-bottom:1px solid var(--line);position:sticky;top:0;z-index:30;backdrop-filter:blur(12px)}.brand{display:flex;gap:12px;align-items:center;text-decoration:none;color:var(--ink);font-weight:700;font-size:14px;letter-spacing:.02em}.brand-icon{height:29px;width:29px;background:var(--ink);color:white;display:grid;place-items:center;border-radius:7px;font-family:Georgia,serif;font-size:20px}.masthead-right{display:flex;align-items:center;gap:20px;font-size:12px;color:var(--muted)}.button{border:1px solid var(--line);background:white;border-radius:6px;color:var(--ink);padding:7px 13px;font-size:12px;font-weight:600}.reading-progress{position:fixed;top:0;left:0;height:3px;width:0;background:var(--teal);z-index:50}.layout{max-width:1510px;margin:auto;display:grid;grid-template-columns:230px minmax(0,1fr);gap:55px;padding:44px 50px 0}.sidebar{position:sticky;top:100px;align-self:start;max-height:calc(100vh - 130px);overflow:auto;font-size:12px;padding-right:12px}.nav-label{text-transform:uppercase;letter-spacing:.14em;color:var(--muted);font-size:10px;font-weight:700;margin-bottom:16px}.sidebar nav{display:grid;gap:2px}.sidebar nav a{display:flex;gap:10px;padding:7px 9px;text-decoration:none;color:var(--muted);line-height:1.45;border-left:2px solid transparent}.sidebar nav a span{font-family:var(--mono);font-size:10px;opacity:.6;padding-top:2px}.sidebar nav a.active{border-left-color:var(--teal);color:var(--teal);background:#e9eeea;font-weight:600}.sidebar-foot{border-top:1px solid var(--line);margin-top:24px;padding-top:18px;color:var(--muted);font-size:11px;line-height:1.6}.sidebar-foot b{color:var(--teal)}main{min-width:0}.hero{padding:15px 0 45px}.eyebrow{text-transform:uppercase;letter-spacing:.16em;font-size:10px;font-weight:750;color:var(--teal);margin-bottom:22px}.hero .lead{font-size:19px;color:var(--muted);line-height:1.6;max-width:760px;margin:26px 0}.hero-meta{display:flex;flex-wrap:wrap;gap:8px}.pill{font-size:11px;border-radius:100px;padding:5px 11px;border:1px solid var(--line);background:#fff}.pill.positive{background:var(--teal-soft);border-color:#b6d8cf;color:#105f52}.pill.pending{background:var(--amber-soft);border-color:#e5c9ad;color:#7a471d}.metrics{display:grid;grid-template-columns:repeat(4,minmax(0,1fr));background:var(--ink);color:white;border-radius:12px;margin-top:30px;padding:26px 12px;box-shadow:var(--shadow)}.metrics>div{padding:0 19px;border-right:1px solid #ffffff25}.metrics>div:last-child{border:0}.metrics strong{display:block;font-family:Georgia,serif;letter-spacing:-.03em;font-size:38px;line-height:1.15}.metrics span{font-size:11px;color:#c8d5da;display:block;margin-top:6px}.hero-note{font-size:11px;color:var(--muted);margin-top:12px}.report-section{padding:45px 0 32px;border-top:1px solid #cad2d0;scroll-margin-top:85px}.section-heading{display:flex;gap:16px;margin-bottom:25px}.section-number{font-family:var(--mono);font-size:11px;color:var(--teal);padding-top:12px}.section-intro{font-size:14px;line-height:1.55;color:var(--muted);margin:0;max-width:720px}.report-section>p{max-width:880px}.two-col{display:grid;grid-template-columns:1fr 1fr;gap:18px;margin:24px 0}.panel{border:1px solid var(--line);border-radius:9px;padding:23px;background:#ffffffb5;min-width:0}.panel h3,.panel h4{margin-top:0}.panel p,.panel li{font-size:14px}.panel ul{margin-bottom:0}.callout{padding:20px 23px;border-left:3px solid var(--amber);background:var(--amber-soft);margin:25px 0;font-size:14px;border-radius:0 7px 7px 0}.callout p{margin:7px 0 0}.callout.strong{border-color:var(--teal);background:var(--teal-soft);font-size:15px}.callout.strong>strong{font-size:18px}.citations{white-space:nowrap}.cite{font:10px var(--mono);text-decoration:none;border-bottom:1px solid #9ebdaf;margin-left:4px;padding:2px 1px}.table-wrap{width:100%;overflow:auto;margin:25px 0;border:1px solid var(--line);border-radius:8px;background:#fff}table{width:100%;border-collapse:collapse;text-align:left;font-size:12px;line-height:1.6}th{background:#edf0ec;font-size:10px;letter-spacing:.055em;text-transform:uppercase;color:#4d606a;font-weight:700}th,td{padding:13px 15px;vertical-align:top;border-bottom:1px solid var(--line)}tbody tr:last-child td{border-bottom:none}td:first-child{font-weight:550}caption{caption-side:bottom;padding:12px 15px;text-align:left;color:var(--muted);font-size:11px}tr.selected td{background:#fff3df}.architecture{list-style:none;padding:0;display:grid;grid-template-columns:repeat(5,minmax(0,1fr));gap:10px;margin:30px 0}.architecture li{position:relative;background:white;border:1px solid var(--line);border-radius:7px;padding:16px 12px;margin:0}.architecture li:not(:last-child):after{content:"→";position:absolute;right:-11px;top:67px;z-index:1;color:var(--muted);background:var(--paper)}.architecture strong{display:block;font-size:13px;line-height:1.45;margin:11px 0}.architecture p{font-size:11px;line-height:1.6;color:var(--muted);margin:0}.layer-tag{font-size:8px;line-height:1.3;text-transform:uppercase;letter-spacing:.04em;display:inline-block;border-radius:3px;padding:4px 5px}.layer-tag.written{background:#e8edf6;color:#35557f}.layer-tag.original{background:var(--teal-soft);color:var(--teal)}.layer-tag.third{background:#f0e9df;color:#82613a}details{border:1px solid var(--line);border-radius:7px;background:#fff;margin:18px 0;padding:0 20px}summary{cursor:pointer;padding:17px 0;font-size:13px;font-weight:650;line-height:1.5}details[open] summary{border-bottom:1px solid var(--line);margin-bottom:18px}details p,details li{font-size:13px}details>p:last-child{padding-bottom:8px}.hashes code{display:block;margin:6px 0 13px;word-break:break-all;font-size:11px}.steps{counter-reset:steps;list-style:none;padding:0;margin:25px 0}.steps>li{position:relative;counter-increment:steps;padding:0 0 0 42px;margin-bottom:22px;font-size:14px}.steps>li:before{content:counter(steps,decimal-leading-zero);position:absolute;left:0;top:1px;color:var(--teal);font:11px var(--mono);border:1px solid #c6d8d1;width:26px;height:26px;display:grid;place-items:center;border-radius:50%}.asset-figure{margin:28px 0;background:#15202e;padding:20px;border-radius:9px}.asset-figure img{display:block;width:100%;height:auto;max-width:900px;margin:auto}.asset-figure figcaption{color:#c4d0d8;font-size:11px;line-height:1.6;margin:16px 0 0}.codebox{background:#172a37;border-radius:8px;padding:16px 20px 18px;margin:24px 0;position:relative;color:#e2e9e8}.code-label{font-size:9px;letter-spacing:.1em;text-transform:uppercase;color:#a9bebf}.copy{position:absolute;right:13px;top:12px;background:#ffffff0d;border:1px solid #ffffff30;color:#d4e3df;border-radius:4px;padding:3px 9px;font-size:10px}.codebox pre{overflow:auto;line-height:1.85;white-space:pre;margin:13px 0 0}.event-strip{display:grid;grid-template-columns:repeat(3,1fr);margin:27px 0;border:1px solid #bed6cb;border-radius:7px;background:#edf6f0}.event-strip>div{padding:19px 20px}.event-strip b{font:26px Georgia,serif;display:block}.event-strip span{font-size:10px;color:var(--muted)}.benchmark-metrics{margin:10px 0 26px;background:#0f5a52}.chart-card{border:1px solid var(--line);border-radius:10px;background:white;margin:28px 0;padding:24px}.chart-toolbar{display:flex;justify-content:space-between;gap:15px;align-items:center}.chart-toolbar h3{margin:0;font-size:18px}.chart-toolbar p{color:var(--muted);font-size:11px;margin:6px 0}.segmented{display:flex;border:1px solid var(--line);border-radius:5px;flex-shrink:0}.segmented button{border:0;background:transparent;padding:6px 9px;font-size:10px;color:var(--muted)}.segmented button[aria-pressed=true]{background:var(--ink);color:white;border-radius:4px}.legend{display:flex;flex-wrap:wrap;gap:18px;font-size:10px;color:var(--muted);margin:18px 0 8px}.legend span{display:flex;gap:6px;align-items:center}.swatch{height:9px;width:9px;display:inline-block;border-radius:2px}.swatch.reset{background:#499c90}.swatch.play{background:#df9e57}#episode-chart svg{display:block;width:100%;height:auto;max-height:600px}.gridline{stroke:#e6e9e7;stroke-width:1}.axis-label,.episode-label,.bar-value{font:11px var(--mono);fill:#586a74}.bar-total{fill:#3d5f78}.bar-reset{fill:#499c90}.bar-play{fill:#df9e57}.row-hit{fill:transparent}.episode{cursor:pointer}.episode:hover .row-hit,.episode.selected .row-hit,.episode:focus .row-hit{fill:#f5f2e8}.episode:focus{outline:none}.episode:focus .bar-total{stroke:var(--ink);stroke-width:2}#episode-chart[data-view=total] .bar-reset,#episode-chart[data-view=total] .bar-play{display:none}.chart-detail{font-size:12px;padding:14px;background:#f2f5f1;border-radius:5px;min-height:65px;margin:0}.sort{font:inherit;text-transform:inherit;letter-spacing:inherit;color:inherit;border:0;background:transparent;padding:0;text-align:left}.winner{font-size:10px;padding:3px 6px;border-radius:4px;white-space:nowrap}.side-0{background:#e6edf9;color:#345e8d}.side-1{background:#f9eae5;color:#9d4d3a}.facts{margin:0;font-size:12px}.facts>div{display:flex;justify-content:space-between;gap:14px;padding:9px 0;border-bottom:1px solid var(--line)}.facts>div:last-child{border:0}.facts dt{color:var(--muted)}.facts dd{margin:0;font-weight:650;text-align:right}.status{font-size:10px;display:inline-block;padding:4px 7px;border-radius:4px;line-height:1.4;white-space:nowrap}.verified{background:var(--teal-soft);color:#146450}.partial{background:var(--amber-soft);color:#8c551d}.pending{background:#eef0f1;color:#66737b}.search-label{display:block;font-size:12px;font-weight:600;margin:24px 0 7px}#evidence-search{width:100%;background:white;padding:12px 14px;border:1px solid #bfcac5;border-radius:6px;color:var(--ink);font-size:13px}#evidence-count{font-size:11px;margin-top:8px}.evidence-id{font:11px var(--mono);color:var(--teal);white-space:nowrap}.evidence-path{font:10px var(--mono);display:block;margin-top:7px;word-break:break-all}.evidence-table td:nth-child(2){width:47%}.json-view{font:10px/1.65 var(--mono);background:#f2f5f3;padding:15px;max-height:450px;overflow:auto;white-space:pre-wrap;overflow-wrap:anywhere}.glossary{display:grid;grid-template-columns:1fr 1fr;gap:15px}.glossary>div{padding:16px;background:#eaeeea;border-radius:5px}.glossary dt{font-size:12px;font-weight:650}.glossary dd{font-size:11px;color:var(--muted);margin:5px 0 0}.footer{border-top:1px solid var(--line);padding:28px 0 55px;font-size:11px;color:var(--muted);margin-top:30px}.footer strong{color:var(--ink)}[hidden]{display:none!important}
@media(min-width:1450px){.layout{gap:70px}}@media(max-width:1100px){.layout{grid-template-columns:180px minmax(0,1fr);gap:25px;padding:30px 25px 0}.sidebar{font-size:11px}.architecture{grid-template-columns:1fr 1fr}.architecture li:not(:last-child):after{display:none}.architecture li:last-child{grid-column:1/-1}.metrics strong{font-size:32px}.metrics>div{padding:0 12px}}@media(max-width:760px){body{font-size:15px}.masthead{padding:0 17px;height:58px}.masthead-right .date-label{display:none}.layout{display:block;padding:0 20px}.sidebar{top:58px;max-height:none;position:sticky;margin:0 -20px 23px;padding:7px 12px;background:#f4f3eef5;z-index:20;border-bottom:1px solid var(--line)}.sidebar nav{display:flex;gap:4px;overflow:auto;white-space:nowrap}.sidebar nav a{padding:7px 9px;border-left:0;border-bottom:2px solid transparent}.sidebar nav a.active{border-bottom-color:var(--teal)}.sidebar nav a span,.nav-label,.sidebar-foot{display:none}.hero{padding-top:13px}h1{font-size:43px}.hero .lead{font-size:17px}.metrics{grid-template-columns:1fr 1fr;gap:22px 0;padding:23px 10px}.metrics>div:nth-child(2){border:0}.metrics strong{font-size:35px}.two-col{grid-template-columns:1fr}.section-heading{gap:11px}.section-number{padding-top:8px}.report-section{padding:32px 0 20px;scroll-margin-top:120px}h2{font-size:31px}.architecture{grid-template-columns:1fr}.architecture li:last-child{grid-column:auto}.chart-card{padding:16px 10px}.chart-toolbar{align-items:flex-start}.chart-toolbar h3{font-size:15px}.segmented button{font-size:9px;padding:6px}.chart-toolbar p{font-size:10px}.chart-detail{font-size:11px}.event-strip>div{padding:15px 10px}.event-strip b{font-size:22px}.glossary{grid-template-columns:1fr}.asset-figure{padding:10px}.panel{padding:19px}.codebox{margin-left:-5px;margin-right:-5px}.table-wrap{border-radius:5px}th,td{padding:11px}html{scroll-padding-top:126px}}
@media print{@page{size:A4;margin:16mm}html{scroll-behavior:auto}body{background:white;font-size:10pt;line-height:1.55;color:#111}.masthead,.sidebar,.reading-progress,.copy,.segmented,.sort,#evidence-search,.search-label,#evidence-count{display:none}.layout{display:block;padding:0;max-width:none}.hero{padding:0 0 18px}h1{font-size:31pt}h2{font-size:23pt}.hero .lead{font-size:12pt}.report-section{padding:24px 0 10px}.section-heading{break-after:avoid}.metrics{background:#edf2ef!important;color:#111;box-shadow:none;border:1px solid #aaa}.metrics span{color:#444}.metrics strong{font-size:25pt}.two-col{gap:10px}.panel,.callout{background:white;border:1px solid #bbb;break-inside:avoid}.architecture{grid-template-columns:repeat(5,1fr)}.architecture li{padding:9px}.architecture p{font-size:7pt}.table-wrap{overflow:visible;border-color:#aaa}table{font-size:8pt}th,td{padding:7px}tr{break-inside:avoid}thead{display:table-header-group}.codebox{background:#f3f3f3;color:#111;border:1px solid #bbb;break-inside:avoid}.code-label{color:#444}pre code{font-size:8pt}.codebox pre{white-space:pre-wrap;overflow-wrap:anywhere}.asset-figure{break-inside:avoid;background:#15202e!important}.asset-figure img{max-height:190mm;object-fit:contain}.asset-figure figcaption{color:#444;background:white;padding:7px}.chart-card{break-inside:avoid;box-shadow:none}.json-view{max-height:none;font-size:7pt;overflow:visible}.citations a{color:#222}.footer{padding-bottom:0}.glossary>div{background:#f3f3f3}.button{display:none}a{color:#1c4c43}.event-strip{background:#f5f5f5}}
"""

CSS += """
@media(max-width:760px){.brand{gap:9px;white-space:nowrap;font-size:12px}.brand>.muted{display:none}.brand-icon{flex-shrink:0}.masthead-right .button{white-space:nowrap;font-size:11px}}
@media print{.sort{display:inline!important}}
@media(prefers-reduced-motion:reduce){html{scroll-behavior:auto}}
"""

JS = """
const data=JSON.parse(document.getElementById('report-data').textContent);
document.getElementById('print-report').addEventListener('click',()=>window.print());
document.querySelectorAll('.copy').forEach(button=>button.addEventListener('click',async()=>{
  const value=button.parentElement.querySelector('code').textContent;
  try{if(navigator.clipboard&&window.isSecureContext){await navigator.clipboard.writeText(value);}
  else{const box=document.createElement('textarea');box.value=value;box.style.position='fixed';box.style.opacity='0';document.body.append(box);box.select();if(!document.execCommand('copy'))throw Error();box.remove();}
  button.textContent='Copied';setTimeout(()=>button.textContent='Copy',1600);}catch{button.textContent='Select text';}
}));
const chart=document.getElementById('episode-chart');
document.querySelectorAll('[data-view]').forEach(button=>{
  if(button.tagName!=='BUTTON')return;
  button.addEventListener('click',()=>{chart.dataset.view=button.dataset.view;document.querySelectorAll('.segmented button').forEach(b=>b.setAttribute('aria-pressed',String(b===button)));});
});
function selectEpisode(seed){
  const episode=data.benchmark.episodes.find(e=>e.seed===seed);if(!episode)return;
  document.querySelectorAll('.episode, #episode-table tbody tr').forEach(e=>e.classList.toggle('selected',Number(e.dataset.seed)===seed));
  document.getElementById('chart-detail').textContent='Policy seed '+seed+': '+episode.duration_seconds.toFixed(3)+'s total, including '+episode.reset_seconds.toFixed(3)+'s reset; '+episode.steps+' decisions; native completion at tick '+episode.final_tick.toLocaleString()+'. Winner: side '+episode.winner_side+'; side-0 reward '+(episode.reward>0?'+':'')+episode.reward+'.';
}
document.querySelectorAll('.episode').forEach(g=>{g.addEventListener('click',()=>selectEpisode(Number(g.dataset.seed)));g.addEventListener('keydown',event=>{if(event.key==='Enter'||event.key===' '){event.preventDefault();selectEpisode(Number(g.dataset.seed));}});});
document.querySelectorAll('#episode-table tbody tr').forEach(row=>row.addEventListener('click',()=>selectEpisode(Number(row.dataset.seed))));
document.querySelectorAll('.sort').forEach(button=>button.addEventListener('click',()=>{
  const key=button.dataset.sort,ascending=button.dataset.direction!=='ascending';button.dataset.direction=ascending?'ascending':'descending';
  document.querySelectorAll('#episode-table th').forEach(th=>th.removeAttribute('aria-sort'));button.closest('th').setAttribute('aria-sort',button.dataset.direction);
  const body=document.querySelector('#episode-table tbody');[...body.rows].sort((a,b)=>(Number(a.dataset[key])-Number(b.dataset[key]))*(ascending?1:-1)).forEach(row=>body.append(row));
}));
const evidenceSearch=document.getElementById('evidence-search');
function filterEvidence(){
  const query=evidenceSearch.value.trim().toLowerCase();let shown=0;
  document.querySelectorAll('[data-evidence]').forEach(row=>{row.hidden=!row.textContent.toLowerCase().includes(query);if(!row.hidden)shown++;});
  document.getElementById('evidence-count').textContent=shown+' of '+data.evidenceCount+' evidence records';
}
evidenceSearch.addEventListener('input',filterEvidence);
document.querySelectorAll('.cite').forEach(a=>a.addEventListener('click',()=>{
  const target=document.getElementById(a.hash.slice(1));
  if(target&&target.hidden){evidenceSearch.value='';filterEvidence();}
}));
const sections=[...document.querySelectorAll('.report-section')],nav=[...document.querySelectorAll('.sidebar nav a')];
let scheduled=false;
function updateReading(){
  const range=document.documentElement.scrollHeight-innerHeight;document.querySelector('.reading-progress').style.width=(range>0?scrollY/range*100:0)+'%';
  let active=sections[0];for(const section of sections){if(section.getBoundingClientRect().top<=150)active=section;}
  nav.forEach(a=>{const yes=a.hash==='#'+active.id;a.classList.toggle('active',yes);if(yes)a.setAttribute('aria-current','location');else a.removeAttribute('aria-current');});scheduled=false;
}
addEventListener('scroll',()=>{if(!scheduled){scheduled=true;requestAnimationFrame(updateReading);}},{passive:true});updateReading();
let priorDetails=[],priorQuery='';
addEventListener('beforeprint',()=>{priorDetails=[...document.querySelectorAll('details')].map(d=>[d,d.open]);priorDetails.forEach(([d])=>d.open=true);priorQuery=evidenceSearch.value;evidenceSearch.value='';filterEvidence();});
addEventListener('afterprint',()=>{priorDetails.forEach(([d,open])=>d.open=open);evidenceSearch.value=priorQuery;filterEvidence();});
"""

NAV = [
    ("read-first", "Current capabilities"), ("architecture", "Runtime architecture"),
    ("scope", "Goal & earlier prototype"), ("inputs", "Inputs & tools"),
    ("assets", "Assets & original UI"), ("unpacking", "Protected code recovery"),
    ("bootstrap", "Native initialization"), ("native-battle", "Battles & card input"),
    ("verification", "Verification & corrections"), ("rl-interface", "RL interface"),
    ("benchmark", "Measured throughput"), ("boundaries", "Gaps & continuation"),
    ("reproduce", "Run & implementation"), ("evidence", "Evidence & glossary"),
]
navigation = "".join(f'<a href="#{anchor}"><span>{i:02d}</span>{esc(title)}</a>'
                     for i, (anchor, title) in enumerate(NAV, 1))
embedded = json.dumps({"benchmark": benchmark, "evidenceCount": len(EVIDENCE)},
                      separators=(",", ":")).replace("<", "\\u003c")
generated = datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%d %H:%M UTC")
document = f"""<!doctype html>
<html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<meta name="description" content="Detailed evidence-linked report on Clash Royale native engine recovery, Rust hosting, original UI assets and the measured minimal RL environment.">
<title>Clash Rust — Native engine recovery & RL report</title><style>{CSS}</style></head>
<body><div class="reading-progress" aria-hidden="true"></div>
<header class="masthead"><a class="brand" href="#top"><span class="brand-icon">C</span> CLASH RUST <span class="muted">/ RESEARCH REPORT</span></a>
<div class="masthead-right"><span class="date-label">STATUS AS OF 19 OCTOBER 2026</span><button class="button" id="print-report" type="button">Print / Save PDF</button></div></header>
<div class="layout"><aside class="sidebar"><div class="nav-label">Inside this report</div><nav aria-label="Report contents">{navigation}</nav>
<div class="sidebar-foot"><b>Headless native RL works.</b><br>Original graphical client and full Rust combat rewrite remain unfinished.<br><br>{len(EVIDENCE)} local evidence records<br>Charts and data embedded<br>No external page dependencies</div></aside>
<main id="top"><header class="hero"><div class="eyebrow">Engineering report 01 · original engine recovery</div>
<h1>From a protected Android package to a working native RL loop.</h1>
<p class="lead">A detailed account of the extraction, decryption, native initialization, verified Rust execution host, card controls and first reinforcement-learning interface — with evidence and explicit limits.</p>
<div class="hero-meta"><span class="pill positive">Original engine executing</span><span class="pill positive">Basic RL interface tested</span><span class="pill pending">Original desktop UI unfinished</span><span class="pill pending">Combat rewrite unfinished</span></div>
<div class="metrics"><div><strong>{benchmark["completed_games_per_minute"]:.2f}</strong><span>Measured complete games / minute</span></div><div><strong>{benchmark["completed_games"]}</strong><span>Native completions in {benchmark["elapsed_seconds"]:.1f}s</span></div><div><strong>12 / 70</strong><span>Regions / registers in adapter proofs</span></div><div><strong>0</strong><span>Benchmark errors or truncations</span></div></div>
<p class="hero-note">Single-process benchmark · fixed recovered scenario and decks · reset, actions and observations included · no renderer or neural training</p></header>
{"".join(sections)}
<footer class="footer"><strong>Clash Rust · project work to date</strong><p>Generated {generated} from retained local artifacts. Core narrative, images, benchmark data and styles are self-contained. Linked source files require the surrounding workspace. The page makes no completeness claim for the original client or a faithful whole-game Rust rewrite.</p></footer>
</main></div><script type="application/json" id="report-data">{embedded}</script><script>{JS}</script></body></html>"""
OUT.parent.mkdir(parents=True, exist_ok=True)
OUT.write_text(document)
print(json.dumps({"report": str(OUT), "bytes": OUT.stat().st_size, "sections": len(sections),
                  "evidence_records": len(EVIDENCE),
                  "sha256": hashlib.sha256(OUT.read_bytes()).hexdigest()}, indent=2))
