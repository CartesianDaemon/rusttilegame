#/bin/bash

# Run from source root
cargo build -r --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/release/tilegame.wasm docs/tilegame_release.wasm
# Freeze version of assets used. TODO: Use rsync to remove old assets from docs/imgs.
cp imgs/* docs/imgs/
