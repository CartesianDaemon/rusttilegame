#![feature(coroutines, coroutine_trait, iter_from_coroutine)]
#![feature(try_trait_v2)]

#![allow(unused_braces)]

#[path = "../engine/mod.rs"] mod engine;
#[path = "../progpuzz_scripts/_mod_scripts.rs"] mod progpuzz_scripts;
#[path = "../progpuzz_gamedata/_mod_gamedata.rs"] mod progpuzz_gamedata;

#[macroquad::main("Tile Game")]
async fn main() {
    engine::run::<progpuzz_gamedata::ProgpuzzGamedata, progpuzz_scripts::ProgpuzzScripts>().await;
}
