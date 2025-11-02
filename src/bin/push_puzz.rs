#![feature(coroutines, coroutine_trait, iter_from_coroutine)]
#![feature(try_trait_v2)]

#![allow(unused_braces)]

#[path = "../engine/mod.rs"] mod engine;
mod pushpuzz_scripts;
mod pushpuzz_gamedata;

#[macroquad::main("Tile Game")]
async fn main() {
    engine::run::<pushpuzz_gamedata::PushpuzzGamedata, pushpuzz_scripts::PushpuzzScripts>().await;
}
