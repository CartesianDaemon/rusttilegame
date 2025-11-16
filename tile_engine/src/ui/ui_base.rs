use std::collections::HashMap;

use macroquad::prelude::*;

use crate::gamedata::BaseGameLogic;
use crate::widget::*;

use super::ui_splash::*;
use super::ui_coding_arena::*;
use super::ui_arena::*;

pub type TextureCache = HashMap<String, Texture2D>;

struct _PRect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl _PRect {
    pub fn _from_screen() -> _PRect {
        _PRect {
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

/// Handles drawing and often input for a specific game state.
/// Delegates drawing to a variety of UiSomething classes. Could be
/// more unified with a base trait. Could rationalise the relationship
/// between a Ui class and a Widget.
pub struct UiBase {
    /// Loaded textures
    texture_cache: TextureCache,
    render_split: UiCodingArena,
}

impl UiBase {
    pub fn new() -> UiBase {
        UiBase {
            texture_cache: HashMap::new(),
            render_split: UiCodingArena::new(),
        }
    }

    /// Draw current gameplay to screen.
    /// TODO: Avoid passing slide and anim through so many layers? Add to struct?
    pub async fn draw_frame<GameLogic: BaseGameLogic>(
        &mut self, state: &mut Widget<GameLogic>, slide_pc: f32, anim_pc: f32
    ) {
        let anim = AnimState {slide_pc, anim_pc};
        match state {
            Widget::Arena(state) => {
                UiArena::render(state, &mut self.texture_cache, anim).await;
            }
            Widget::Splash(state) => {
                let _r = UiSplash::render(state);
            }
            Widget::CodingArena(state) => {
                self.render_split.render(state);
            }
        }
    }
}

/// Sync load macroquad texture. Panic on failure.
pub async fn load_texture_unwrap(path: &str) -> Texture2D {
    // futures::executor::block_on(load_texture(path))

    // TODO: Remove this fallback again. But have some way of outputting errors in wasm?
    match load_texture(path).await {
        Result::Ok(tex_data) => tex_data,
        Result::Err(_err) => {
            // display error somewhere?
            Texture2D::empty()
        }
    }
}
