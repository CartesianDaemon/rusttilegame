use macroquad::prelude::*;

use crate::for_gamedata;

#[derive(Copy, Clone, Default)]
pub struct PRect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl PRect {
    pub fn from_screen() -> PRect {
        PRect {
            x: 0.,
            y: 0.,
            w: screen_width(),
            h: screen_height(),
        }
    }
}

// Current state of animation for UIs which implement that.
#[derive(Clone, Copy, Default)]
pub struct AnimState {
    // How far to slide drawing elements from screen coordinates for previous posn to
    // scren coordinates for current posn. Between 0 and 1.0.
    pub slide_pc: f32,
    // How far through an animation sequence we are. Between 0 and 1.0 if the game is
    // advanced on tick. >1 if move complete but game not advanced, ie all idle.
    pub anim_pc: f32
}

pub struct Ticker {
    pub last_tick_time: f64,

    // Time between ticks in sec.
    speed: f64,
}

impl Ticker {
    pub fn new() -> Ticker {
        Ticker {
            speed: 0.3,
            last_tick_time: get_time(),
        }
    }

    /// Defining when to advance game state.
    ///
    /// Should any of this be in Arena not Input? Or should Input be called UI?
    pub fn ready_to_advance_game_state(&mut self, most_recent_cmd: Option<for_gamedata::MoveCmd>, anim: &mut AnimState) -> bool {
        if most_recent_cmd.is_some() {
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

/// Seems like clear_background is mandatory for wasm, broken for windows.
/// Don't know about linux or android.
/// Don't know if fixed in more recent macroquad.
pub fn clear_background_for_current_platform(color: Color) {
    match std::env::consts::OS {
        "windows" => (),
        _ => clear_background(color),
    }
}
