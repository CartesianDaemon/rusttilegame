// Breadcrumb: Avoid warnings of unused code in each separare binary.
// Try using lib crate + two binary crates in a workspace?

#![feature(coroutines, coroutine_trait, iter_from_coroutine)]
#![feature(try_trait_v2)]

#![allow(unused_braces)]

mod engine;
mod simple_custom_props;
mod pushpuzz;

#[macroquad::main("Tile Game")]
async fn main() {
    run::<pushpuzz::PushpuzzGamedata>().await;
}
