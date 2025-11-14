use std::collections::HashMap;

use macroquad::prelude::*;

use crate::gamedata::BaseMovementLogic;
use crate::pane::*;

use super::ui_splash::*;
use super::ui_split::*;
use super::ui_arena::*;

pub type TextureCache = HashMap<String, Texture2D>;

/// Seems like clear_background is mandatory for wasm, broken for windows.
/// Don't know about linux or android.
/// Don't know if fixed in more recent macroquad.
pub fn clear_background_for_current_platform(color: Color) {
    match std::env::consts::OS {
        "windows" => (),
        _ => clear_background(color),
    }
}

/// NB: Split Render up into a struct or trait implemented for each
/// Pane. Or Screen of multiple Panes.
/// NB: Or have ProgpuzzView (elsewhere called screen), which might
/// delegate drawing to one preimplemented View, or multuple tiled.
/// Which combine Render code in a View struct or trait specialised
/// for the relevant Widget/Gamedata.
/// And Gamedata contains one or more Widgets.
/// And Views may display data from one Widget, or from more than one.
pub struct UiBase {
    /// Loaded textures
    texture_cache: TextureCache,
    render_split: UiSplit,
}

impl UiBase {
    pub fn new() -> UiBase {
        UiBase {
            texture_cache: HashMap::new(),
            render_split: UiSplit::new(),
        }
    }

    /// Draw current gameplay to screen.
    /// TODO: Avoid passing slide and anim through so many layers? Add to struct?
    pub async fn draw_frame<MovementLogic: BaseMovementLogic>(
        &mut self, state: &mut Pane<MovementLogic>, slide_pc: f32, anim_pc: f32
    ) {
        match state {
            Pane::Arena(state) => {
                UiArena::render(state, &mut self.texture_cache, slide_pc, anim_pc, state.map_w(), state.map_h()).await;
            }
            Pane::Splash(state) => {
                let _r = UiSplash::render(state);
            }
            Pane::Split(state) => {
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
