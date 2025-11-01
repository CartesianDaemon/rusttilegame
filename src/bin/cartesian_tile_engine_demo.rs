#![feature(coroutines, coroutine_trait, iter_from_coroutine)]
#![feature(try_trait_v2)]

#![allow(unused_braces)]

// TODO: Use different name of mod in different exes.

#[path = "../engine/_mod_engine.rs"] mod engine;
#[path = "../pushing_puzzle_scripts/_mod_scripts.rs"] mod scripts;
#[path = "../pushing_puzzle_gamedata/_mod_gamedata.rs"] mod gamedata;

#[macroquad::main("Tile Game")]
async fn main() {
    engine::run::<gamedata::BiobotGame>().await;
}
