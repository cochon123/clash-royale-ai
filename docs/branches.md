# Working version

The active development branch is `main` in
`/home/cochon/Documents/mini-project/clash-rust`. It contains the compiled Rust
engine, the original reference runtime, spectator, and RL training tools.

The separate `/home/cochon/Documents/mini-project/clash-rust-slow` checkout was
removed at the user's request after verifying it had no uncommitted files.
Its `slow-working` Git branch remains as historical source; there is no second
working directory. Large local inputs and Python environments in the main
workspace were preserved.

See [fast training](fast-banana-training.md) for the current CPU/GPU runner and
[engine validation](../rust-engine/README.md) for recovered-code coverage.
No remote repository is configured.
