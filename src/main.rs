#![feature(coroutines, coroutine_trait, iter_from_coroutine)]

mod engine;
mod levset_biobot;

use engine::game::Game;

#[macroquad::main("Tile Game")]
async fn main() {
    let mut game = Game::new(levset_biobot::BiobotLevSet {});

    loop {
        game.do_frame().await;
        macroquad::prelude::next_frame().await;
    }
}
