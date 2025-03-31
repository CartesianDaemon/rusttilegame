#![feature(coroutines, coroutine_trait, iter_from_coroutine)]

#[path = "engine/_mod.rs"]
mod engine;

#[path = "levset_biobot/_mod.rs"]
mod levset_biobot;

use engine::game::Engine;

#[macroquad::main("Tile Game")]
async fn main() {
    let mut engine = Engine::new(levset_biobot::BiobotLevels {});

    loop {
        engine.do_frame().await;
        macroquad::prelude::next_frame().await;
    }
}
