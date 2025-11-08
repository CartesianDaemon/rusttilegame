use macroquad::prelude::*;

use super::map_coords::Cmd;

/// Interaction from user. Including timing.
///
/// TODO: Nice to simplify interface and move the correct things here and into Arena.
pub struct Input {
    pub last_tick_time: f64,

    // Time between ticks in sec.
    speed: f64,

    // Should change to list.
    most_recent_cmd: Option<Cmd>,
}

impl Input {
    pub fn new() -> Input {
        Self {
            speed: 0.3,
            last_tick_time: 0.,
            most_recent_cmd: None,
        }
    }

    pub fn new_begin() -> Input {
        Self {
            last_tick_time: get_time(),
            .. Self::new()
        }
    }

    pub fn from_cmd(cmd: Cmd) -> Self
    {
        Self {
            most_recent_cmd: Some(cmd),
            ..Self::new()
        }
    }

    pub fn consume_cmd(&mut self) -> Option<Cmd> {
        self.most_recent_cmd.take()
    }

    pub fn read_input(&mut self) {
        if let Some(key) = get_last_key_pressed() {
            self.most_recent_cmd = Some( match key {
                KeyCode::Left  => Cmd::Left,
                KeyCode::Right => Cmd::Right,
                KeyCode::Up    => Cmd::Up,
                KeyCode::Down  => Cmd::Down,
                _              => Cmd::Stay,
            })
        } else if is_mouse_button_pressed(MouseButton::Left) {
            let pp = mouse_position();
            let up_right: bool = pp.0 / pp.1 >= screen_width() / screen_height();
            let up_left: bool = (screen_width() - pp.0) / pp.1 >= screen_width() / screen_height();
            self.most_recent_cmd = Some(
                match (up_right, up_left) {
                    (true, true) => Cmd::Up,
                    (true, false) => Cmd::Right,
                    (false, true) => Cmd::Left,
                    (false, false) => Cmd::Down,
                }
            )
        }
    }

    /// Defining when to advance game state.
    ///
    /// Should any of this be in Arena not Input? Or should Input be called UI?
    pub fn ready_to_advance_game_state(&mut self, anim_pc: &mut f32, slide_pc: &mut f32) -> bool {
        if self.most_recent_cmd.is_some() {
            self.last_tick_time = get_time();
            *anim_pc = 0.;
            *slide_pc = 0.;
            true
        } else {
            let frame_time_pc = ((get_time() - self.last_tick_time) / self.speed) as f32;
            *anim_pc = frame_time_pc % 1.0;
            *slide_pc = frame_time_pc.min(1.0);
            false
        }
    }

}


