#![feature(coroutines, coroutine_trait, iter_from_coroutine)]

#[path = "engine/_mod.rs"]
mod engine;

#[path = "levset_biobot/_mod.rs"]
mod levset_biobot;

use engine::game::Game;

#[macroquad::main("Tile Game")]
async fn main() {
    let mut game = Game::new(levset_biobot::BiobotLevels {});

    loop {
        game.do_frame().await;
        macroquad::prelude::next_frame().await;
    }
}
