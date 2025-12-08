use macroquad::prelude::*;

use crate::gamedata::{BaseGamedata};

pub struct LevChooser {
}

impl LevChooser {
    pub fn do_frame<GameData: BaseGamedata>(
            game_state: &GameData,
            draw_coords: (f32, f32),
        ) {
            let font_col = DARKGRAY;
            draw_text(game_state.get_level_str().as_str(), draw_coords.0, draw_coords.1, 20., font_col);
        }
}
