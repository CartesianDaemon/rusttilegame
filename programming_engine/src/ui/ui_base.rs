use std::collections::HashMap;

use macroquad::prelude::*;

use crate::gamedata::BaseGamedata;
use crate::gamedata;
use crate::widget::*;

use super::ui_helpers::*;
use super::ui_splash::*;
use super::ui_coding_arena::*;

pub type TextureCache = HashMap<String, Texture2D>;

/// Handles drawing and often input for a specific game state.
/// Delegates drawing to a variety of UiSomething classes. Could be
/// more unified with a base trait. Could rationalise the relationship
/// between a Ui class and a Widget.
pub struct UiBase {
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

impl UiBase {
    pub fn new() -> UiBase {
        UiBase {
            texture_cache: HashMap::new(),
            ui_coding_arena: UiCodingArena::new(),
            anim: AnimState::default(),
            ticker: Ticker::new(),
        }
    }

    // NB: Move into Widget. Need to move reset_tick into Ui. First need to move
    // gamedata (ie levidx) into state widget?
    fn advance<Gamedata: gamedata::BaseGamedata>(&mut self, widget: &mut Widget<Gamedata::GameLogic>, cmd: MoveCmd) -> WidgetContinuation {
        let widget_continuation = widget.advance(cmd);
        if let WidgetContinuation::Break(_) = widget_continuation {
            self.ticker.reset_tick();
        }
        widget_continuation
    }

    /// Draw current gameplay to screen.
    /// TODO: Avoid passing slide and anim through so many layers? Add to struct?
    pub async fn do_frame<GameData: BaseGamedata>(&mut self, widget: &mut Widget<GameData::GameLogic>, state: &GameData) -> WidgetContinuation {
        let mut widget_continuation = WidgetContinuation::Continue(());
        match widget.tick_based() {
            TickStyle::TickAutomatically => {
                if self.ticker.tick_if_ready() {
                    widget_continuation = self.advance::<GameData>(widget, MoveCmd::Tick);
                }
                self.anim = self.ticker.anim_state();
            },
            TickStyle::TickOnInput => {
                if false { // TODO Get this driven from ui_coding_arena
                    self.ticker.reset_tick();
                    widget_continuation = self.advance::<GameData>(widget, MoveCmd::Tick);
                }
                self.anim = self.ticker.anim_state();
            },
            TickStyle::Continuous => {
                // TODO: ui_coding_arena should handle this
                //if let Some(cmd) = self.input.consume_cmd() {
                //    widget_continuation = self.advance::<GameData>(widget, cmd);
                //}
                // Treat any movement as completed
                self.anim = AnimState { slide_pc: 1., .. self.ticker.anim_state() }
            }
        }

        match widget {
            Widget::Splash(widget) => {
                let _r = UiSplash::do_frame(widget);
            }
            Widget::CodingArena(widget) => {
                self.ui_coding_arena.do_frame(widget, &mut self.texture_cache, self.anim, state).await;
            }
        }
        sleep_between_frames_on_linux_windows();

        return widget_continuation;
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
