#![feature(coroutines, coroutine_trait, iter_from_coroutine)]

#![allow(unused_braces)]

#[path = "engine/_mod.rs"]
mod engine;

#[path = "scripts/_mod.rs"]
mod scripts;

#[path = "gamedata/_mod.rs"]
mod _mod;

use engine::Engine;
use _mod::BiobotGame;

#[macroquad::main("Tile Game")]
async fn main() {
    let mut engine = Engine::<BiobotGame>::new();

    loop {
        engine.do_frame().await;
        macroquad::prelude::next_frame().await;
    }
}
