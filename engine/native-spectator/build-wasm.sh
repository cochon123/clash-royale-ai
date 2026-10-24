#!/bin/sh
# Build the browser painter. Offline, no wasm-bindgen.
# Output: engine/native-spectator/target/wasm32-unknown-unknown/release/clash_painter.wasm
set -eu
cd "$(dirname "$0")"
root="$(pwd)"
export CARGO_TARGET_DIR="$root/target"
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-6}"
# panic=abort matches the wasm32-unknown-unknown default and keeps the
# module free of the unwind runtime. Scoped to this invocation.
export RUSTFLAGS="${RUSTFLAGS:-} -C panic=abort -C link-arg=--export-memory"
cargo build --offline --release \
  --target wasm32-unknown-unknown \
  --no-default-features \
  --features wasm \
  --lib \
  -j "$CARGO_BUILD_JOBS"
cp -f "$root/target/wasm32-unknown-unknown/release/clash_native_spectator.wasm" \
  "$root/target/wasm32-unknown-unknown/release/clash_painter.wasm"
ls -lh "$root/target/wasm32-unknown-unknown/release/clash_painter.wasm"
