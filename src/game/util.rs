use macroquad::prelude::*;

use futures::executor::block_on;

pub fn load_texture_blocking_unwrap(path: &str) -> Texture2D {
    block_on(load_texture(path)).unwrap()
}


