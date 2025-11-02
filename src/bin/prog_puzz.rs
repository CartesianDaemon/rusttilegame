#![feature(coroutines, coroutine_trait, iter_from_coroutine)]
#![feature(try_trait_v2)]

#![allow(unused_braces)]

mod engine;
mod scripts_progpuzz;
mod gamedata_progpuzz;

#[macroquad::main("Tile Game")]
async fn main() {
    engine::run::<gamedata_progpuzz::ProgpuzzGamedata, scripts_progpuzz::ProgpuzzScripts>().await;
}
