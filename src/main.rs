mod types;
mod util;

pub mod game;
mod play;
mod input;
mod render;
mod map;
mod ent;
mod load;

use game::Game;

#[macroquad::main("Tile Game")]
async fn main() {
    let mut game = Game::new_default();

    loop {
        game.do_frame();

        macroquad::prelude::next_frame().await;
    }
}
