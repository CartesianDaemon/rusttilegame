#![feature(coroutines, coroutine_trait, iter_from_coroutine)]
#![feature(try_trait_v2)]

#![allow(unused_braces)]

mod engine;
mod progpuzz_scripts;
mod progpuzz_gamedata;

#[macroquad::main("Tile Game")]
async fn main() {
    engine::run::<progpuzz_gamedata::ProgpuzzGamedata, progpuzz_scripts::ProgpuzzScripts>().await;
}
