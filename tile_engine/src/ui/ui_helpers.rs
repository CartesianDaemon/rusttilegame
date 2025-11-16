use macroquad::prelude::*;

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
pub struct AnimState {
    // How far to slide drawing elements from screen coordinates for previous posn to
    // scren coordinates for current posn. Between 0 and 1.0.
    pub slide_pc: f32,
    // How far through an animation sequence we are. Between 0 and 1.0 if the game is
    // advanced on tick. >1 if move complete but game not advanced, ie all idle.
    pub anim_pc: f32
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
