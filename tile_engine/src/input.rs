use macroquad::prelude::*;

use super::map_coords::MoveCmd;

/// Interaction from user. Including timing.
///
/// NB: Want to keep some utility code here and merge the Pane-specific info into Panes.
pub struct Input {
    pub most_recent_cmd: Option<MoveCmd>,
}

impl Input {
    pub fn new() -> Input {
        Self {
            most_recent_cmd: None,
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
}
