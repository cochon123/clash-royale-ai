#!/usr/bin/env sh
set -eu
cd "$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
cargo build --release --bin clash-rust
exec ./target/release/clash-rust "$@"
