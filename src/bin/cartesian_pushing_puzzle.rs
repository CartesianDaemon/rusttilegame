#![feature(coroutines, coroutine_trait, iter_from_coroutine)]
#![feature(try_trait_v2)]

#![allow(unused_braces)]

#[path = "../engine/_mod_engine.rs"] mod engine;
#[path = "../pushpuzz_scripts/_mod_scripts.rs"] mod pushpuzz_scripts;
#[path = "../pushpuzz_gamedata/_mod_gamedata.rs"] mod pushpuzz_gamedata;

#[macroquad::main("Tile Game")]
async fn main() {
    engine::run::<pushpuzz_gamedata::BiobotGame, pushpuzz_scripts::PushpuzzScripts>().await;
}
