#!/usr/bin/env bash
set -euo pipefail
viewer_dir="$(cd "$(dirname "$0")" && pwd)"
cargo build --quiet --release --manifest-path "$viewer_dir/Cargo.toml"
exec "$viewer_dir/target/release/clash-native-ui-viewer" "$@"
