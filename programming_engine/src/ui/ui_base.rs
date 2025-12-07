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

    /// Smoothly from 0 to 1 transition from previous state to current state
    /// TODO: Move into arena?
    /// TODO: Updated by input::ready_to_advance. Is that right? Could return tuple.
    /// TODO: Combine anim and slide..?
    anim: crate::ui::AnimState,

    /// Record input from user ready for use.
    ticker: Ticker,
}

impl Ui {
    pub fn new() -> Ui {
        Ui {
            texture_cache: HashMap::new(),
            ui_coding_arena: UiCodingArena::new(),
            anim: AnimState::default(),
            ticker: Ticker::new(),
        }
    }

    /// Draw current gameplay to screen.
    /// TODO: Avoid passing slide and anim through so many layers? Add to struct?
    pub async fn do_frame<GameData: BaseGamedata>(&mut self, scene: &mut Scene<GameData::GameLogic>, state: &GameData) {
        match scene {
            Scene::Splash(_) => {
                if was_any_input() {
                    scene.advance(InputCmd::NextPhase);
                }
            }
            Scene::CodingArena(_) => {
                match scene.tick_based() {
                    TickStyle::TickAutomatically => {
                        if self.ticker.tick_if_ready() {
                            scene.advance(InputCmd::Tick);
                        }
                        self.anim = self.ticker.anim_state();
                    },
                    TickStyle::Continuous => {
                        // Handle inside ui_coding_arena.advance()
                        // scene_continuation = self.advance_continuous::<GameData>(scene, InputCmd::NextPhase);
                    }
                    TickStyle::TickOnInput => {
                        panic!();
                    },
                }
            }
        }

        match scene {
            Scene::Splash(scene) => {
                let _r = UiSplash::do_frame(scene);
            }
            Scene::CodingArena(scene) => {
                self.ui_coding_arena.do_frame(scene, &mut self.texture_cache, self.anim, state).await;
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
