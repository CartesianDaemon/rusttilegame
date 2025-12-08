use macroquad::prelude::*;

use crate::gamedata::{BaseGamedata};

pub struct LevChooser {
}

impl LevChooser {
    pub fn do_frame<GameData: BaseGamedata>(
            game_state: &GameData,
            draw_coords: (f32, f32),
        ) {
            let active = (DARKGRAY, WHITE, BLUE);
            let unlocked = (DARKGRAY, WHITE, DARKGRAY);
            let locked = (LIGHTGRAY, DARKGRAY, BLACK);

            let n_levs = game_state.num_levels();

            let mut curr_x = draw_coords.0 + 15.;
            let approx_half_char_width = 4.;
            let y = 20.;
            let txt_below_of_centre = 5.;
            let stride = 30.;
            let r = 10.;

            for lev_idx in 1..=n_levs {
                let digits = if lev_idx < 10 {1.} else {2.};
                let cols = if lev_idx == game_state.get_current_level() {
                    active
                } else if game_state.get_unlocked_levels().contains(&lev_idx) {
                    unlocked
                } else {
                    locked
                };
                draw_circle(curr_x, y, r, cols.1);
                draw_circle_lines(curr_x, y, r, 1., cols.2);
                let (text_x, text_y) = (curr_x - digits*approx_half_char_width, y + txt_below_of_centre);
                draw_text(format!("{lev_idx}").as_str(), text_x, text_y, 20., cols.0);
                curr_x += stride;
            }
        }
}
