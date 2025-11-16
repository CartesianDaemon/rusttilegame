use macroquad::prelude::*;

use crate::ui::AnimState;

use super::map_coords::MoveCmd;

/// Interaction from user. Including timing.
///
/// NB: Want to keep some utility code here and merge the Pane-specific info into Panes.
pub struct Input {
    pub last_tick_time: f64,

    // Time between ticks in sec.
    speed: f64,

    // Should change to list.
    most_recent_cmd: Option<MoveCmd>,
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

    pub fn from_cmd(cmd: MoveCmd) -> Self
    {
        Self {
            most_recent_cmd: Some(cmd),
            ..Self::new()
        }
    }

    pub fn consume_cmd(&mut self) -> Option<MoveCmd> {
        self.most_recent_cmd.take()
    }

    pub fn read_input(&mut self) {
        if let Some(key) = get_last_key_pressed() {
            self.most_recent_cmd = Some( match key {
                KeyCode::Left  => MoveCmd::Left,
                KeyCode::Right => MoveCmd::Right,
                KeyCode::Up    => MoveCmd::Up,
                KeyCode::Down  => MoveCmd::Down,
                _              => MoveCmd::Stay,
            })
        } else if is_mouse_button_pressed(MouseButton::Left) {
            let pp = mouse_position();
            let up_right: bool = pp.0 / pp.1 >= screen_width() / screen_height();
            let up_left: bool = (screen_width() - pp.0) / pp.1 >= screen_width() / screen_height();
            self.most_recent_cmd = Some(
                match (up_right, up_left) {
                    (true, true) => MoveCmd::Up,
                    (true, false) => MoveCmd::Right,
                    (false, true) => MoveCmd::Left,
                    (false, false) => MoveCmd::Down,
                }
            )
        }
    }

    /// Defining when to advance game state.
    ///
    /// Should any of this be in Arena not Input? Or should Input be called UI?
    pub fn ready_to_advance_game_state(&mut self, anim: &mut AnimState) -> bool {
        if self.most_recent_cmd.is_some() {
            self.last_tick_time = get_time();
            *anim = AnimState::default();
            true
        } else {
            let pc_through_tick = ((get_time() - self.last_tick_time) / self.speed) as f32;
            anim.anim_pc = pc_through_tick % 1.0;
            anim.slide_pc = pc_through_tick.min(1.0);
            false
        }
    }

}


