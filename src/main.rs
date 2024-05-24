use macroquad::prelude::*;

pub mod game;
use game::*;

#[macroquad::main("Tile Game")]
async fn main() {
    let mut g = Game::new_default();

    loop {
        g.do_frame();

        next_frame().await;
    }
}
