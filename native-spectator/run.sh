#!/usr/bin/env bash
set -euo pipefail
spectator_dir="$(cd "$(dirname "$0")" && pwd)"
cargo build --quiet --manifest-path "$spectator_dir/Cargo.toml"
exec "$spectator_dir/target/debug/clash-native-spectator" "$@"
