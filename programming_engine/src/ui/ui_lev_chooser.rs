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
            let locked_col = DARKGRAY;

            let n_levs = game_state.num_levels();

            let mut curr_x = draw_coords.0 + 15.;
            let approx_half_char_width = 4.;
            let y = 20.;
            let txt_below_of_centre = 5.;
            let stride = 30.;
            let r = 10.;

            for lev_idx in 1..=n_levs {
                let digits = if lev_idx < 10 {1.} else {2.};
                draw_circle_lines(curr_x, y, r, 1., locked_col);
                let (text_x, text_y) = (curr_x - digits*approx_half_char_width, y + txt_below_of_centre);
                draw_text(format!("{lev_idx}").as_str(), text_x, text_y, 20., font_col);
                curr_x += stride;
            }
        }
}
