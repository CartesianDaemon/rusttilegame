use macroquad::prelude::*;

use crate::ui::PRect;
use crate::gamedata::{BaseGamedata};

struct DragInfo {
    lev_idx: u16,
    mouse_down_time: f64,
}

#[derive(Default)]
pub struct LevChooser {
    drag_origin: Option<DragInfo>,
}

impl LevChooser {
    fn col_active(mouseover: bool) -> (Color, Color, Color, f32) {
        match mouseover {
            false => (DARKGRAY, WHITE, BLUE, 2.),
            true => (DARKGRAY, YELLOW, BLUE, 2.2),
        }
    }

    fn col_unlocked(mouseover: bool) -> (Color, Color, Color, f32) {
        match mouseover {
            false => (DARKGRAY, WHITE, DARKGRAY, 1.),
            true => (DARKGRAY, YELLOW, DARKGRAY, 2.),
        }
    }

    fn col_locked(mouseover: bool) -> (Color, Color, Color, f32) {
        match mouseover {
            false => (LIGHTGRAY, DARKGRAY, BLACK, 1.),
            true => (LIGHTGRAY, DARKGRAY, BLACK, 1.2),
        }
    }

    pub fn do_frame<GameData: BaseGamedata>(&mut self, game_state: &mut GameData, draw_coords: (f32, f32)) {
            let n_levs = game_state.num_levels();

            let mut curr_x = draw_coords.0 + 15.;
            let approx_half_char_width = 4.;
            let y = 20.;
            let txt_below_of_centre = 5.;
            let stride = 30.;
            let r = 10.;

            let hold_for = 0.3;

            if !is_mouse_button_down(MouseButton::Left) {
                self.drag_origin = None;
            }

            for lev_idx in 1..=n_levs {
                let rect = PRect { x: curr_x - r, y: y - r, w: r * 2., h: r * 2.};
                let mouseover = rect.contains(mouse_position());

                if mouseover && is_mouse_button_down(MouseButton::Left) && game_state.get_unlocked_levels().contains(&lev_idx) {
                    if let Some(drag_info) = &self.drag_origin {
                        if drag_info.lev_idx == lev_idx && get_time() > drag_info.mouse_down_time + hold_for {
                            game_state.goto_level(lev_idx);
                        }
                    } else {
                        self.drag_origin = Some(DragInfo { lev_idx, mouse_down_time: get_time() });
                    }
                }

                let cols = if lev_idx == game_state.get_current_level() {
                    Self::col_active(mouseover)
                } else if game_state.get_unlocked_levels().contains(&lev_idx) {
                    Self::col_unlocked(mouseover)
                } else {
                    Self::col_locked(mouseover)
                };

                draw_circle(curr_x, y, r, cols.1);
                draw_circle_lines(curr_x, y, r, cols.3, cols.2);

                let digits = if lev_idx < 10 {1.} else {2.};
                let (text_x, text_y) = (curr_x - digits*approx_half_char_width, y + txt_below_of_centre);
                draw_text(format!("{lev_idx}").as_str(), text_x, text_y, 20., cols.0);
                curr_x += stride;
            }
        }
}
