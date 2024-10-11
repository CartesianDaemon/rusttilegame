mod map_coords;
mod util;

pub mod game;
mod play;
mod input;
mod render;
mod map;
mod obj;
mod levset;
mod levset_biobot;

mod test;

use game::Game;

#[macroquad::main("Tile Game")]
async fn main() {
    let mut game = Game::new(levset_biobot::BiobotLevSet {});

    loop {
        game.do_frame();
        macroquad::prelude::next_frame().await;
    }
}
