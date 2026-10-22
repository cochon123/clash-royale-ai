# Clash Lab

Read-only research observatory. Uses the shared `webapps.common` paths, catalog
and HTTP toolkit, Python's standard library, and the existing `markdown` package.
No package installation or build step is needed.

```sh
.venv/bin/python -m webapps.lab --host 127.0.0.1 --port 8801
```

For a phone on the same LAN, bind with `--host 0.0.0.0` and open
`http://<this-computer-LAN-IP>:8801`. Service links replace loopback with the
browser's hostname. The other service must also listen on a LAN interface to
be accessible directly from the phone. Lab probes stay on server-side loopback.

## Pages

- **Overview** (`/`, `/overview`, `#overview`): five-second observations of trainer
  processes, user services, run state, STOP files, hardware, and local service
  health. Collector pause reasons are visible. Recent run outcomes link to detail.
- **Models** (`/models`, `#models`): curated playable registry first, then discovered
  replay models, auxiliary predictors and experiments. Search/filter, metric
  definitions and provenance, checkpoint presence/size, model cards and reports.
- **Runs** (`/runs`, `#runs`): classified run catalog and search/filter. Detail links
  use `#runs?id=<encoded ID>`. Native SVG curves separate evaluation banks,
  inference modes, specialists and train/eval outcomes. Hover/tap gives values;
  legend buttons toggle curves. Less prominent outcome charts expand on demand.
- **Engine** (`/engine`, `#engine`): component descriptions, reference/recompiled
  binaries, file modification times and sizes, validation summaries, six decks.
- **Data** (`/data`, `#data`): direct raw capture file count/bytes, collector health,
  data areas, top-level log inventory, searchable links to existing HTML reports.
- **Docs** (`/docs`, `#docs`): both research and engine Markdown catalogs, search,
  local Markdown reader and relative document links.

The direct page URLs serve the same local shell; hash navigation preserves
filters and supports deep links. Asset requests never require a CDN.

## Read-only GET endpoints

| Endpoint | Contents |
| --- | --- |
| `/api/health` | Lab health and read-only flag |
| `/api/overview` | Reconciled training status, hardware, services, recent runs |
| `/api/models` | Playable and offline model catalogs |
| `/api/models/{id}` | One model's metrics, metadata and artifacts |
| `/api/runs` | Classified run summaries and status evidence |
| `/api/runs/{id}` | State, bounded log samples, chart series, files, checkpoints |
| `/api/engine` | Binaries, validation records, preset decks and card catalog |
| `/api/data` | Replay counts, dataset areas, logs, reports, collector response |
| `/api/docs` | Markdown document catalog |
| `/api/document?path=<repo-relative-path>` | Rendered, sanitized local document |
| `/model-card?path=<repo-relative-path>` | Standalone readable Markdown/HTML card |
| `/reports/{path}` | Existing report and report assets, served by shared mount |
| `/cards/{path}` | Existing local card art |
| `/static/{path}` | Lab shell assets |

Encode nested run IDs as a single URL component, e.g.
`/api/runs/banana-play%2FSESSION`. Unknown IDs and document traversal return 404.
There are no mutation routes. No subprocess command starts/stops a service,
engine, match, or trainer. The only subprocesses are read-only `systemctl --user
list-units` and `nvidia-smi`, each with a one-second timeout. Service probes issue
local GET requests with a 0.6-second timeout and bypass proxy settings.

## Observation limits

- Hardware/process/service probes run in a background thread every five seconds.
  `/api/overview` reads that snapshot plus a five-second run cache. No GPU command
  or HTTP probe blocks the overview request. Before the first sample, or with
  inaccessible/stale probes, training absence is **unknown**.
- A matching trainer process/active training unit overrides a STOP file or
  completed state, exposing the conflict. A recent (<120s) nonterminal state can
  indicate activity without confirmed processes. Past-deadline/unconfirmed old
  states are stale. Dashboard and engine-worker processes alone do not prove
  training. Process/service evidence expires after 20 seconds if sampling stalls.
  Trainer ownership uses its explicit `--run-dir` (relative paths are resolved
  from the observed process's working directory), so seed checkpoints do not
  make the parent run appear active. Trainers without an explicit run directory
  may only appear in Overview. Test entry points are not counted as trainers.
- Models/engine/docs cache for 60 seconds. Model discovery is depth ≤2, skips
  archives and symlinks, caps 4,000 entries and 70 metadata files per model.
  Expected checkpoint references in reports are listed even when missing.
  Unrecognized metrics remain in the abbreviated original metadata.
- Run detail caches for five seconds. Each known JSONL reader loads at most 8 MiB
  and 12,000 lines from the end. Truncated windows and malformed lines are marked;
  cumulative outcome counts then apply only to that window. JSON files are capped
  at 12 MiB. Huge arrays are abbreviated in metadata responses. JSONL history is
  never implicitly a full-history guarantee.
- Raw counts/sizes use a single direct `scandir` and cache for 300 seconds. No
  recursion into `data/raw`, `data/v9_cache`, or `engine/analysis`. Other directory
  sizes are unknown. Raw count means regular files, not validated replay records.
- File timestamps are filesystem modification times, not verified build times.
  Report matching uses normalized filename/version prefixes and explicit aliases;
  unmatched reports remain accessible in Data. No performance/parity checks run.
- Historical `clash-rust/` absolute paths map to the current `engine/` location.
- Reader documents omit scripts/styles/embedded images for offline readability.
  Top-level report HTML is served from an in-memory copy with remote font links
  removed and a local-resource content policy; inline charts remain enabled.
  Report files on disk remain unchanged. The Lab has no external scripts, fonts or CDN.
- The server reads current project artifacts on demand and stores caches only in
  memory. Python may create normal bytecode caches; use
  `PYTHONDONTWRITEBYTECODE=1` when you want no generated bytecode outside the Lab.

## Verification

```sh
PYTHONDONTWRITEBYTECODE=1 .venv/bin/python -m unittest webapps.lab.test_lab -v
node --check webapps/lab/static/app.js
```

`test_lab.py` uses temporary fixtures exclusively under `webapps/lab/` and checks
status conflicts, stale/unknown observations, entry-point and run ownership,
partial JSONL, cache behavior,
legacy paths, malformed state schemas, and document traversal/active content.
`verify_render.cjs` exercises all page renderers against the running server
without installing a DOM/browser dependency:

```sh
node webapps/lab/verify_render.cjs http://127.0.0.1:8801
```
