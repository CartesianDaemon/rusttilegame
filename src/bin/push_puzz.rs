#![feature(coroutines, coroutine_trait, iter_from_coroutine)]
#![feature(try_trait_v2)]

#![allow(unused_braces)]

mod engine;
mod pushpuzz;

#[macroquad::main("Tile Game")]
async fn main() {
    engine::run::<pushpuzz::PushpuzzGamedata>().await;
}
