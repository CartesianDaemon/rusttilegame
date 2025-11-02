#![feature(coroutines, coroutine_trait, iter_from_coroutine)]
#![feature(try_trait_v2)]

#![allow(unused_braces)]

#[path = "../engine/_mod_engine.rs"] mod engine;
#[path = "../pushing_puzzle_scripts/_mod_scripts.rs"] mod pushpuzz_scripts;
#[path = "../pushing_puzzle_gamedata/_mod_gamedata.rs"] mod pushing_puzzle_gamedata;

#[macroquad::main("Tile Game")]
async fn main() {
    engine::run::<pushing_puzzle_gamedata::BiobotGame, pushpuzz_scripts::PushpuzzScripts>().await;
}
