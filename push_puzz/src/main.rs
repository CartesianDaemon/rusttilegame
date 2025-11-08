// Breadcrumb: Avoid warnings of unused code in each separare binary.
// Try using lib crate + two binary crates in a workspace?

#![feature(coroutines, coroutine_trait, iter_from_coroutine)]
#![feature(try_trait_v2)]

#![allow(unused_braces)]

mod pushpuzz;

#[macroquad::main("Tile Game")]
async fn main() {
    tile_engine::run::<pushpuzz::PushpuzzGamedata>().await;
}
