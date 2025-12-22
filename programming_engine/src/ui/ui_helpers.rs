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

    pub fn contains(&self, pt: (f32, f32)) -> bool {
        (self.x..self.x+self.w).contains(&pt.0) && (self.y..self.y+self.h).contains(&pt.1)
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum InputCmd {
    Continue, // Move to next scene, or start executing.
    Tick, // Advance map. From timer, or from ui_coding_arena.
    Cancel, // Cancel execution
}

pub enum KeyType {
    Ok,
    Normal,
    Escape,
    Modifier,
}

pub fn was_key_pressed() -> Option<KeyType> {
    use macroquad::input::KeyCode::*;
    match macroquad::input::get_last_key_pressed() {
        Some(Space | Enter ) => Some(KeyType::Ok),
        Some(Escape | Backspace ) => Some(KeyType::Escape),
        Some(key_code) if key_code as u16 <= 65362 => Some(KeyType::Normal),
        Some(_) => Some(KeyType::Modifier),
        None => None,
    }
}

pub fn was_any_input() -> bool {
    use KeyType::*;
    matches!(was_key_pressed(), Some(Ok | Normal | Escape)) || is_mouse_button_pressed(MouseButton::Left)
}

// Current state of animation for UIs which implement that.
#[derive(Clone, Copy, Default)]
pub struct AnimState {
    // How far to slide drawing elements from screen coordinates for previous posn to
    // scren coordinates for current posn. Between 0 and 1.0.
    pub slide_frac: f32,
    // How far through an animation sequence we are. Between 0 and 1.0 if the game is
    // advanced on tick. >1 if move complete but game not advanced, ie all idle.
    pub anim_frac: f32
}


pub struct Ticker {
    pub last_tick_time: f64,

    // Time between ticks in sec.
    tick_interval_set: Vec<f64>,
    tick_interval_idx: usize,
}

impl Ticker {
    pub fn new() -> Ticker {
        Ticker {
            tick_interval_set: vec![0.6, 0.4, 0.15, 0.025],
            tick_interval_idx: 1,
            last_tick_time: get_time(),
        }
    }

    fn tick_interval(&self) -> f64 {
        self.tick_interval_set[self.tick_interval_idx]
    }

    fn next_expected_tick(&self) -> f64 {
        self.last_tick_time + self.tick_interval()
    }

    pub fn cycle_tick_intervals(&mut self) {
        self.tick_interval_idx +=1 ;
        if self.tick_interval_idx >= self.tick_interval_set.len() {
            self.tick_interval_idx = 0
        }
        log::debug!("Set tick interval to {}s", self.tick_interval());
    }

    pub fn reset_tick(&mut self) {
        self.last_tick_time = get_time();
    }

    pub fn tick_if_ready(&mut self) -> bool {
        let curr_time = get_time();
        if curr_time >= self.next_expected_tick() {
            self.reset_tick();
            true
        } else {
            false
        }
    }

    pub fn anim_state(&self) -> AnimState {
        let pc_through_tick = ((get_time() - self.last_tick_time) / self.tick_interval()) as f32;
        AnimState {
            anim_frac: pc_through_tick % 1.0,
            slide_frac: pc_through_tick.min(1.0),
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
