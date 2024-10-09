mod types;
mod util;

pub mod game;
mod play;
mod input;
mod render;
mod map;
mod ent;
mod levset;
mod biobot;

mod test;

use game::Game;

#[macroquad::main("Tile Game")]
async fn main() {
    let mut game = Game::new(biobot::BiobotLevSet {});

    loop {
        /* Collect input and advance state each frame.
         *
         * During gameplay, Game advances animation each frame but only advances
         * logical state each fixed tick interval.
         */
        game.do_frame();
        macroquad::prelude::next_frame().await;
    }
}
