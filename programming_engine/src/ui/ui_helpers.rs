use macroquad::prelude::*;

#[derive(Copy, Clone, Default)]
pub struct PRect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl PRect {
    pub fn _from_screen() -> PRect {
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



pub enum TickStyle {
    Continuous,
    TickOnInput,
    TickAutomatically,
}

pub struct Ticker {
    pub last_tick_time: f64,

    // Time between ticks in sec.
    tick_interval: f64,
}

impl Ticker {
    pub fn new() -> Ticker {
        Ticker {
            tick_interval: 0.5,
            last_tick_time: get_time(),
        }
    }

    pub fn reset_tick(&mut self) {
        self.last_tick_time = get_time();
    }

    pub fn tick_if_ready(&mut self) -> bool {
        let curr_time = get_time();
        if curr_time >= self.last_tick_time + self.tick_interval {
            self.reset_tick();
            true
        } else {
            false
        }
    }

    pub fn anim_state(&self) -> AnimState {
        let pc_through_tick = ((get_time() - self.last_tick_time) / self.tick_interval) as f32;
        AnimState {
            anim_pc: pc_through_tick % 1.0,
            slide_pc: pc_through_tick.min(1.0),
        }
    }
}

/// Seems like clear_background is mandatory for wasm, broken for windows.
/// Don't know about linux or android.
/// Don't know if fixed in more recent macroquad.
pub fn clear_background_for_current_platform(color: Color) {
    match std::env::consts::OS {
        "windows" => draw_rectangle(0., 0., screen_width(), screen_height(), color),
        _ => clear_background(color),
    }
}

/// Seems like clear_background is mandatory for wasm, broken for windows.
/// Don't know about linux or android.
/// Don't know if fixed in more recent macroquad.
pub fn sleep_between_frames_on_linux_windows() {
    match std::env::consts::OS {
        "windows" | "linux" => std::thread::sleep(std::time::Duration::from_millis(25)),
        _ => (),
    }
}
