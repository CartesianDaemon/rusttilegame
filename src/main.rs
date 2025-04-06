#![feature(coroutines, coroutine_trait, iter_from_coroutine)]

#![allow(unused_braces)]

#[path = "engine/_mod.rs"] mod engine;
#[path = "scripts/_mod.rs"] mod scripts;
#[path = "gamedata/_mod.rs"] mod gamedata;

#[macroquad::main("Tile Game")]
async fn main() {
    let mut engine = engine::Engine::<gamedata::BiobotGame>::new();

    loop {
        engine.do_frame().await;
        macroquad::prelude::next_frame().await;
    }
}
