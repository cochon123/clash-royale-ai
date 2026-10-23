# Final eight-worker RL pipeline benchmark

This directory is the isolated rerun after the earlier uncapped 12/16-worker matrix exhausted host memory. Nothing in the original matrix or repository was modified by this benchmark worker.

Watch progress:

    watch -n1 python3 /tmp/cr-prof/bench-safe/monitor.py
    python3 /tmp/cr-prof/bench-safe/monitor.py --watch 1

Supervisor: `cr-bench-safe-supervisor.service` (transient user service). Individual trainer units and exact launch commands are recorded in `commands.txt`; all trainers/native children have MemoryMax=8G, MemorySwapMax=0, OOMPolicy=kill and KillMode=control-group. Each launch requires at least12GiB MemAvailable. The supervisor polls every0.5s and stops only its current trainer unit on a cap event or MemAvailable below6GiB. Trainer RuntimeMaxSec=1200. Unit properties are recorded with every telemetry sample.

Results:

- `report.html`: standalone report, embedded data, native SVG graphs and training curves; no network dependencies.
- `analysis.json`: complete structured analysis, raw results, paired outcome comparison, training metrics and input hashes.
- `results.csv`, `waves.csv`: per-run and per-wave timing/throughput.
- `summary.json`: authoritative scalar summary of completed final-code runs.
- `*-matches.jsonl`: all native terminal outcomes.
- `*-updates.jsonl`: actual PPO metrics for all three updates.
- `*-telemetry.json`: cgroup properties, host memory, memory.events, kernel/systemd memory peaks, before/after snapshots.
- `*-before.json`, `*-after.json`, `preflight.json`, `postflight.json`: resource snapshots and immutable input hashes/repository HEADs.
- `*.stdout.log`, `*-train.log`, `*-native-worker-*.log`, `*-reward-audit.jsonl`, `*-state.json`, `*-manifest.json`: preserved evidence. Scratch model checkpoint directories are deleted after each successful run.
- `earlier_incomplete_matrix_summary.json`: clearly labeled historical observations before final correctness fix; these are not final measurements or safe-worker recommendations.

`total_s` includes all three training collection+PPO waves and cleanup; excludes constructor_s. Wave1 includes native startup/reset. Games/min and decisions/s come directly from those total_s fields. Policies evolve between waves, so first-wave measurements are analyzed separately. Native request duration sums and engine_s overlap across threads and inference; never add them to wall time. Encoding is nested in act_batch, which is nested in launch. Act is inclusive policy work, not pure GPU inference.

The two seeds are777 and2026. Seed777 order baseline,g1,g2; seed2026 order g2,g1,baseline. All configurations use eight workers and three PPO waves, eight sampler jobs per wave. Outcome equality compares deck/controller/seat/result/seconds by(policy_version,seed); it does not establish action equality.
