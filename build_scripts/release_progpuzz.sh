#/bin/bash

# Run from source root
cargo build -r --target wasm32-unknown-unknown --bin prog_puzz
cp target/wasm32-unknown-unknown/release/prog_puzz.wasm docs/prog_puzz.wasm
# Freeze version of assets used. TODO: Use rsync to remove old assets from docs/imgs.
cp imgs/* docs/imgs/
cp test_wasm/*.js docs/
