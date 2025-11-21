use std::collections::HashMap;

use macroquad::prelude::*;

use crate::gamedata::BaseGameLogic;
use crate::widget::*;

use super::ui_helpers::*;
use super::ui_splash::*;
use super::ui_coding_arena::*;
use super::ui_arena::*;

pub type TextureCache = HashMap<String, Texture2D>;

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
    pub async fn draw_frame<GameLogic: BaseGameLogic>(&mut self, state: &mut Widget<GameLogic>, anim: AnimState) {
        match state {
            Widget::Arena(state) => {
                UiArena::render(state, &mut self.texture_cache, PRect::from_screen(), anim).await;
            }
            Widget::Splash(state) => {
                let _r = UiSplash::render(state);
            }
            Widget::CodingArena(state) => {
                self.render_split.render(state, &mut self.texture_cache, anim).await;
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
