#![feature(coroutines, coroutine_trait, iter_from_coroutine)]

#[path = "engine/_mod.rs"]
mod engine;

#[path = "biobot_game/_mod.rs"]
mod biobot_game;

use engine::Engine;
use engine::Game;
use biobot_game::BiobotGame;

#[macroquad::main("Tile Game")]
async fn main() {
    let mut engine = Engine::new(BiobotGame::new_game());

    loop {
        engine.do_frame().await;
        macroquad::prelude::next_frame().await;
    }
}
