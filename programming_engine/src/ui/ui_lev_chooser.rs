use macroquad::prelude::*;

use crate::ui::PRect;
use crate::gamedata::{BaseGameData};

struct DragInfo {
    lev_idx: u16,
    mouse_down_time: f64,
}

#[derive(Default)]
pub struct LevChooser {
    drag_origin: Option<DragInfo>,
}

#[derive(PartialEq)]
enum MouseOverState {
    Neutral,
    Over,
    PressedOn,
    PressedOff,
}

struct MouseOverCols {
    text: Color,
    fill: Color,
    border: Color,
    border_width: f32,
}

impl LevChooser {
    fn col_active(mouseover: MouseOverState) -> MouseOverCols {
        MouseOverCols {border: BLUE, border_width: 2., ..LevChooser::col_unlocked(mouseover)}
    }

    fn col_unlocked(mouseover: MouseOverState) -> MouseOverCols {
        use MouseOverState::*;
        let neutral_cols = MouseOverCols {text: DARKGRAY, fill: WHITE, border: DARKGRAY, border_width: 1.};
        match mouseover {
            Neutral => neutral_cols,
            Over => MouseOverCols {fill: YELLOW, border_width: 2., ..neutral_cols},
            PressedOn => MouseOverCols {fill: ORANGE, border_width: 2., ..neutral_cols},
            PressedOff => MouseOverCols {fill: YELLOW, border_width: 2., ..neutral_cols},
        }
    }

    fn col_locked(mouseover: MouseOverState) -> MouseOverCols {
        use MouseOverState::*;
        let border_width = match mouseover {
            Neutral | PressedOff => 1.0,
            Over | PressedOn => 1.2,
        };
        MouseOverCols {text: LIGHTGRAY, fill: DARKGRAY, border: BLACK, border_width}
    }

    pub fn do_frame<GameData: BaseGameData>(&mut self, game_state: &mut GameData, draw_coords: (f32, f32)) {
            let n_levs = game_state.num_levels();
            let buttons_per_row = 10;

            let approx_half_char_width = 4.;
            let h_stride = 30.;

            let txt_below_of_centre = 5.;
            let r = 10.;
            let v_stride = 30.;

            let hold_for = 0.1;

            if !is_mouse_button_down(MouseButton::Left) {
                self.drag_origin = None;
            }

            let init_x = draw_coords.0 + 15.;
            let mut curr_x = init_x;
            let init_y = 20.;
            for lev_idx in 1..=n_levs {
                if lev_idx % buttons_per_row == 1 {
                    curr_x = init_x;
                } else {
                    curr_x += h_stride;
                }
                let curr_y = init_y + ((lev_idx-1) / 10) as f32 * v_stride;

                let rect = PRect { x: curr_x - r, y: curr_y - r, w: r * 2., h: r * 2.};
                let mouse_in = rect.contains(mouse_position());

                if mouse_in && is_mouse_button_down(MouseButton::Left) && game_state.get_unlocked_levels().contains(&lev_idx) {
                    if let Some(drag_info) = &mut self.drag_origin {
                        if drag_info.lev_idx == lev_idx && get_time() > drag_info.mouse_down_time + hold_for {
                            game_state.goto_level(lev_idx);
                            // Invalidate drag until mouse released
                            drag_info.lev_idx = 0;
                        }
                    } else {
                        self.drag_origin = Some(DragInfo { lev_idx, mouse_down_time: get_time() });
                    }
                }

                let mouse_over_state = if let Some(drag_info) = &mut self.drag_origin && drag_info.lev_idx == lev_idx {
                    if mouse_in {
                        MouseOverState::PressedOn
                    } else {
                        MouseOverState::PressedOff
                    }
                } else if self.drag_origin.is_none() && mouse_in {
                    MouseOverState::Over
                } else {
                    MouseOverState::Neutral
                };
                let cols = if lev_idx == game_state.get_current_level() {
                    Self::col_active(mouse_over_state)
                } else if game_state.get_unlocked_levels().contains(&lev_idx) {
                    Self::col_unlocked(mouse_over_state)
                } else {
                    Self::col_locked(mouse_over_state)
                };

                draw_circle(curr_x, curr_y, r, cols.fill);
                draw_circle_lines(curr_x, curr_y, r, cols.border_width, cols.border);

                let digits = if lev_idx < 10 {1.} else {2.};
                let (text_x, text_y) = (curr_x - digits*approx_half_char_width, curr_y + txt_below_of_centre);
                draw_text(format!("{lev_idx}").as_str(), text_x, text_y, 20., cols.text);
            }
        }
}
