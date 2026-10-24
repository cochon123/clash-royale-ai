# W5 results: native-watch preload and in-memory reset

Branch `arena-speed/engine`. Local machine only. The VPS was not touched, so the plan's VPS reset target is not measured here.

## What was done

`native-watch` now verifies a checkpoint once per process and keeps a pristine template in memory: the `checkpoint.json` bytes, the region images, the allocator (`GuestHeap`), and the architectural registers. `--preload` does that verification before reading stdin.

- `{"op":"ping"}` returns `ready`, `load_seconds`, and `resets`.
- A later `reset` stats each verified file (device, inode, size, mtime) and builds a new `NativeSession` from the template. It does not re-read or re-hash checkpoint contents. The reset reply fields are the same as before, including `engine_reset`.
- The first hash failure still aborts that attempt. `--preload` exits with code 2. A `reset` that fails verification returns `ok: false` and does not install a session, which is what the process did before, and the process stays up so the client can read the error.
- If a later stat shows a different file identity, the reset fails and does not re-hash.
- `NativeSession::load` still verifies on every call. `native-rl`, `native-oracle`, and the other native-engine bins were not switched to the template cache.

Tests added:

- file identity changes when the file size changes (`oracle.rs`)
- ping before load, missing-checkpoint preload exit, and a corrupted region hash
- C4 equivalence: 5 seeds (`1`, `7`, `42`, `1000`, `44219`) × 3 deck pairs (`hog26`/`log-bait`, `hog26`/`pixel9_mortar`, `log-bait`/`pixel9_mortar`) × 600 ticks. The fixed script steps 20 ticks at a time and plays at ticks 100, 220, 340, and 460. Reset #1 of a fresh process matched reset #5 of a preloaded process.

`cargo test --release --offline --manifest-path engine/native-engine/Cargo.toml` : lib tests 5 passed, 1 ignored (the pre-existing full harness match). The watch integration tests passed in 107.55 s, including the 15-scenario equivalence run.

## Measurements

Reset time is `reset_seconds` from the reply. RSS is `VmRSS` from `/proc/<pid>/status`.

Before binary: main checkout `engine/native-engine/target/release/native-watch`, sha256 `9abd8ec0fa0c00049939b15f8f99c3c7648e098ed9b9f6102da1fe6f642d2acf`. Five resets in one process, `hog26` vs `log-bait`, seed 1, no template (that binary has no `--preload`).

| reset | reset_seconds | RSS |
| --- | ---: | ---: |
| 1 | 2.361 s | 839404 kB |
| 2 | 1.959 s | 1020420 kB |
| 3 | 1.929 s | 1020980 kB |
| 4 | 2.095 s | 1020980 kB |
| 5 | 2.193 s | 1020996 kB |

Median reset **2.095 s**. Median RSS **1020980 kB** (997 MiB).

After binary: this worktree `engine/native-engine/target/release/native-watch`, sha256 `3479f75f508e8d88f3813b26288246e5797e2d16ee44f6ea38626c0451cec64b`. Started with `--preload`, then the same five resets.

| reset | reset_seconds | RSS |
| --- | ---: | ---: |
| 1 | 0.216 s | 885472 kB |
| 2 | 0.285 s | 989492 kB |
| 3 | 0.269 s | 989020 kB |
| 4 | 0.219 s | 989564 kB |
| 5 | 0.210 s | 989572 kB |

Median reset **0.219 s**. Median RSS **989492 kB** (966 MiB). Preload finished in `load_seconds` **1.604 s**, with RSS **588248 kB** (574 MiB) before any reset. Ping then reported `ready: true`, `resets: 0`, and after the five resets `resets: 5`.

| Target | Measured |
| --- | --- |
| Local reset ≤ 0.3 s (plan: down from 1.9 s) | **0.219 s median**, from 2.095 s on the previous binary. One of the five samples was 0.285 s. |
| VPS reset ≤ 0.6 s (plan: down from 3.8–4.1 s) | Not measured. |
| Equivalence, 5×3×600 ticks | Passed. Replies matched after removing every key whose name ends in `_seconds`. |
| First verification failure aborts | Passed for a missing `checkpoint.json` and for a region hash mismatch. |

## Contract deviations

- Ping before a template exists returns `{"ready":false,"load_seconds":0.0,"resets":0}`. The contract example shows `ready: true`, which is the shape after `--preload` or a successful verification. The three fields are always present.
- A later reset does not read checkpoint bytes. It does `stat` the verified paths, and each new session still calls `uc_open`. Libunicorn is `dlopen`ed once during verification, not again on reset.
- `--preload` failure exits 2 because stdin is not being read yet. A failed `reset` verification stays in-process and returns `ok: false`, matching the previous binary.
- The equivalence check compares canonical JSON (`serde_json`) after dropping `*_seconds` keys. It does not compare the raw line bytes with those numbers still in place. All 15 scenarios matched under that comparison.
- An identity mismatch is covered by a temp-file unit test. It was not run by rewriting the shared checkpoint.

## Open problems

- The VPS reset time is unknown. Local median is under 0.3 s; the slowest of the five warm samples was 0.285 s.
- A preloaded process that has completed a reset sits at about 966 MiB RSS. That is close to the old process after five reloads (997 MiB) and above the old first-reset RSS (820 MiB). The template alone, before a session exists, is about 574 MiB. The parsed checkpoint heap is about 2.9 million allocation entries, and that cost is inside the warm reset rather than broken out separately.
- This worktree does not contain the gitignored `engine/tools/reverse-env` tree. Load looks for `libunicorn.so.2` there after the checkpoint's old `clash-rust` path misses. Tests used a local symlink to the main checkout's copy. That symlink is gitignored and is not part of the branch.
