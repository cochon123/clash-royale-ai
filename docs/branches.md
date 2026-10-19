# Working versions

- `slow-working` preserves the verified original-engine host and Python RL interface.
- `rust-fast` contains the ongoing native Rust compilation and performance work.

The main checkout is `/home/cochon/Documents/mini-project/clash-rust` on
`rust-fast`. The independent slow checkout is
`/home/cochon/Documents/mini-project/clash-rust-slow` on `slow-working`.
You can use both checkouts without switching the active development directory.

Run the verified slow environment from its checkout:

```sh
cd /home/cochon/Documents/mini-project/clash-rust-slow
cargo build --manifest-path native-engine/Cargo.toml --release --offline --bin native-rl
tools/reverse-env/bin/python -m rl.smoke_test
tools/reverse-env/bin/python -m rl.benchmark --seconds 60 --out /tmp/clash-slow-benchmark.json
```

The original XAPK, extracted libraries, checkpoints and installed analysis tools
are large local inputs, excluded from Git. The slow checkout links its `analysis`
directory and Python environment to the existing workspace. These inputs must
stay in place. Runtime changes belong on `rust-fast`; the verified checkpoint
files are preserved and shared read-only by the engines.

Git branches alone do not make a portable package. The included report embeds
its main narrative and selected evidence, but full reproduction still needs the
local inputs above. No remote repository or publication is configured.
