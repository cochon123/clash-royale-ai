# Correctness validation

Three GPT-6.1-Sol implementation tasks produced the incremental encoder, grouped engine scheduling and shared opponent inference. Independent review identified two placement-sampling reproducibility problems, resolved in `fc4d9eb`:

1. Fresh placement support now sorts by score and ascending tile ID, so tied scores cannot reorder when batch padding changes. Fixed rollout candidate order remains unchanged for PPO recomputation.
2. Mixed-mode global-RNG sampling trims placement distributions to the sampled subset support width. Greedy rows therefore do not change global RNG consumption by widening the categorical.

A second read-only GPT-6.1-Sol review found no additional correctness findings. The per-game-uniform vectorized path was retained to avoid unnecessary categorical reconstruction.

Validation ran 77 tests on CPU and CUDA, all passing. Frozen-encoder checks compared 3,790 sample pairs and 49,218 tensor pairs exactly. Tests cover heterogeneous legal masks, equal-score top-k boundaries, mixed greedy/sampled rows, per-game uniform partitioning, global RNG state, fixed PPO support likelihood/entropy/gradients, mask request concurrency, and worker scheduling.

The tied-placement regression fixes wait, placement and critic outputs to isolate sampling behavior from CUDA batch-shape roundoff. Existing tests retain real neural-output parity checks. Canonical ties intentionally differ from historical arbitrary topk tie order. Global-RNG sampling does not promise arbitrary partition invariance. Multi-wave training can diverge because PPO transition/minibatch ordering changes with scheduling; throughput does not establish playing strength.

The first benchmark process was lost under memory pressure. Eight completed earlier runs remain explicitly separate from final-code measurements. Final validation uses eight workers, an 8 GiB cgroup limit with no swap, and a host MemAvailable watchdog.
