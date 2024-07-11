use macroquad::prelude::*;

pub mod game;
mod play;
mod input;
mod render;
mod map;
mod ent;
mod load;
mod util;

use game::*;

#[macroquad::main("Tile Game")]
async fn main() {
    let mut g = Game::new_default();

    loop {
        g.do_frame();

        next_frame().await;
    }
}
