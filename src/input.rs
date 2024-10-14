use macroquad::prelude::*;

/// Interaction from user. Including timing.
pub struct Input {
    // Time of last frame.
    // Could be set via a ready_for_tick() fn
    pub last_update: f64,

    // Time between ticks in sec.
    speed: f64,

    // Should change to list.
    // Ideally contain Move(1,0) action not KeyRight.
    last_key_pressed: Option<KeyCode>,
}

impl Input {
    pub fn new_blank() -> Input {
        Input {
            speed: 0.15,
            last_update: 0.,
            last_key_pressed: None,
        }
    }

    pub fn new_begin() -> Input {
        Input {
            last_update: get_time(),
            .. Input::new_blank()
        }
    }

    pub fn from_key(last_key_pressed: KeyCode) -> Input {
        Input {last_key_pressed: Some(last_key_pressed), ..Input::new_blank()}
    }

    pub fn read_input(&mut self) {
        if let Some(key) = get_last_key_pressed() {
            self.last_key_pressed = Some(key);
        } else if is_mouse_button_pressed(MouseButton::Left) {
            let pp = mouse_position();
            let up_right: bool = pp.0 / pp.1 >= screen_width() / screen_height();
            let up_left: bool = (screen_width() - pp.0) / pp.1 >= screen_width() / screen_height();
            self.last_key_pressed = Some(
                match (up_right, up_left) {
                    (true, true) => KeyCode::Up,
                    (true, false) => KeyCode::Right,
                    (false, true) => KeyCode::Left,
                    (false, false) => KeyCode::Down,
                }
            )
        }
    }

    /// Defining when to advance game state.
    ///
    /// Should any of this be in Play not Input? Or should Input be called UI?
    pub fn ready_to_advance_game_state(&mut self) -> bool {
        if self.last_key_pressed.is_some() {
            self.last_update = get_time();
            true
        } else {
            false
        }
    }

    pub fn ready_to_advance_ghost_state(&mut self) -> bool {
        if get_time() - self.last_update > self.speed {
            self.last_update = get_time();
            true
        } else {
            false
        }
    }

    pub fn consume_keypresses(&mut self) -> Option<KeyCode> {
        self.last_key_pressed.take()
    }
}


