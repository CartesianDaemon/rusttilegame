use crate::gamedata::BaseGamedata;
use crate::ui::AnimState;
use crate::ui::Ticker;

use super::gamedata;
use super::widget::*;
use super::input::Input;
use super::ui::UiBase;

/// Overall Engine state.
///
/// Including set of levels in current Engine, and state of current level being played.
///
/// Templated on Game (either a  builtin Game, or a load-from-file Game).
/// Could instead take a &dyn Game trait object so that it could load a Game object
/// from a library, but that probably doesn't help that much.
struct Engine<Gamedata: BaseGamedata> {
    /// Level set currently playing through, e.g. the biobot Engine.
    pub gamedata: Gamedata,

    /// Current state of gameplay, current level, mostly map etc.
    state: Widget<Gamedata::GameLogic>,

    /// Smoothly from 0 to 1 transition from previous state to current state
    /// TODO: Move into arena?
    /// TODO: Updated by input::ready_to_advance. Is that right? Could return tuple.
    /// TODO: Combine anim and slide..?
    anim: crate::ui::AnimState,

    /// Record input from user ready for use.
    input: Input,
    ticker: Ticker,

    ///
    ui: UiBase,
}

impl<Gamedata: gamedata::BaseGamedata> Engine<Gamedata> {
    pub fn new() -> Engine<Gamedata> {
        let gamedata = Gamedata::new();
        let arena = gamedata.load_pane();
        Engine::<Gamedata> {
            gamedata: gamedata,
            state: arena,
            anim: AnimState::default(),
            input: Input::new(),
            ui: UiBase::new(),
            ticker: Ticker::new(),
        }
    }

    fn advance(&mut self) {
        let cmd = self.input.consume_cmd();
        let widget_continuation = self.state.advance(cmd);
        if let PaneContinuation::Break(widget_ending) = widget_continuation {
            self.state = self.gamedata.load_next_pane(widget_ending);
            self.ticker.reset();
        }
    }

    /// Collect input. Draw frame. Advance logical Engine state, if tick scheduled.
    pub async fn do_frame(&mut self) {
        self.input.read_input();

        let advance_automatically = false;
        if advance_automatically {
            // Automatic advancing not implemented yet.
            unimplemented!();
        } else if self.state.tick_based() {
            // Advances whenever key pressed. Animation proceeds for tick-interval afterwards.
            if self.input.most_recent_cmd.is_some() {
                self.ticker.reset();
                self.advance();
            }
            self.anim = self.ticker.anim_state();
        } else {
            // Advances whenever key pressed. Animation always at 0 or 1.
            self.advance();
        }

        self.ui.draw_frame(&mut self.state, self.anim).await;
    }
}

pub async fn run<Gamedata: gamedata::BaseGamedata>()
{
    let mut log_builder = env_logger::Builder::new();
    log_builder.format_timestamp(None);
    log_builder.format_target(false);
    for arg in std::env::args() {
        if let Some(log_opts) = arg.strip_prefix("--rust-log=") {
            log_builder.parse_filters(log_opts);
            break;
        }
    }
    log_builder.init();
    // WIP: Configure logging to have shorter prefix.
    log::info!("Started logging!");

    let mut engine = Engine::<Gamedata>::new();

    loop {
        engine.do_frame().await;
        macroquad::prelude::next_frame().await;
    }
}
