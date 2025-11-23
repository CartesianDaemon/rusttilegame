#/bin/bash

# TODO: Copy new pushpuzz executable name, and html file.

# Run from source root
cargo build -r --target wasm32-unknown-unknown --bin push_puzz
cp target/wasm32-unknown-unknown/release/cartesian_tile_engine_demo.wasm docs/cartesian_tile_engine_demo.wasm
# Freeze version of assets used. TODO: Use rsync to remove old assets from docs/imgs.
cp imgs/* docs/imgs/
