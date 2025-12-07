use std::collections::HashMap;

use macroquad::prelude::*;

use crate::gamedata::BaseGamedata;
use crate::scene::*;

use super::ui_helpers::*;
use super::ui_splash::*;
use super::ui_coding_arena::*;

pub type TextureCache = HashMap<String, Texture2D>;

/// Handles drawing and often input for a specific game state.
/// Delegates drawing to a variety of UiSomething classes. Could be
/// more unified with a base trait. Could rationalise the relationship
/// between a Ui class and a Scene.
pub struct Ui {
    /// Loaded textures
    texture_cache: TextureCache,
    ui_coding_arena: UiCodingArena,
}

impl Ui {
    pub fn new() -> Ui {
        Ui {
            texture_cache: HashMap::new(),
            ui_coding_arena: UiCodingArena::new(),
        }
    }

    /// Draw current gameplay to screen.
    /// TODO: Avoid passing slide and anim through so many layers? Add to struct?
    pub async fn do_frame<GameData: BaseGamedata>(&mut self, scene: &mut Scene<GameData::GameLogic>, state: &GameData) {
        match scene {
            Scene::Splash(scene_struct) => {
                UiSplash::advance(scene_struct);
                let _r = UiSplash::do_frame(scene_struct);
            }
            Scene::CodingArena(scene_struct) => {
                self.ui_coding_arena.advance::<GameData>(scene_struct);
                self.ui_coding_arena.do_frame(scene_struct, &mut self.texture_cache, state).await;
            }
        }
        sleep_between_frames_on_linux_windows();
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
