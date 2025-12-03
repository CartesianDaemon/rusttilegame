use crate::gamedata::BaseGamedata;
use crate::map_coords::MoveCmd;
use crate::ui::AnimState;
use crate::ui::TickStyle;
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
        let arena = gamedata.load_scene();
        Engine::<Gamedata> {
            gamedata: gamedata,
            state: arena,
            anim: AnimState::default(),
            input: Input::new(),
            ui: UiBase::new(),
            ticker: Ticker::new(),
        }
    }

    // NB: Move into Widget. Need to move reset_tick into Ui. First need to move
    // gamedata (ie levidx) into state widget?
    fn advance(&mut self, cmd: MoveCmd) {
        let widget_continuation = self.state.advance(cmd);
        if let WidgetContinuation::Break(widget_ending) = widget_continuation {
            self.state = self.gamedata.load_next_pane(widget_ending);
            self.ticker.reset_tick();
        }
    }

    /// Collect input. Draw frame. Advance logical Engine state, if tick scheduled.
    /// NB: Move into Ui
    pub async fn do_frame(&mut self) {
        self.input.read_input();

        match self.state.tick_based() {
            TickStyle::TickAutomatically => {
                if self.ticker.tick_if_ready() {
                    let cmd = self.input.consume_cmd().unwrap_or(MoveCmd::default());
                    self.advance(cmd);
                }
                self.anim = self.ticker.anim_state();
            },
            TickStyle::TickOnInput => {
                if let Some(cmd) = self.input.consume_cmd() {
                    self.ticker.reset_tick();
                    self.advance(cmd);
                }
                self.anim = self.ticker.anim_state();
            },
            TickStyle::Continuous => {
                if let Some(cmd) = self.input.consume_cmd() {
                    self.advance(cmd);
                }
            }
        }

        self.ui.draw_frame(&mut self.state, self.anim, &self.gamedata).await;
    }
}

pub fn get_arg(prefix: &str) -> Option<String> {
    std::env::args().map(|arg| arg.strip_prefix(prefix).map(str::to_string)).flatten().next()

    // With my putative chain macros
    //chain![ std::env::args() | x.strip_prefix(prefix) || x.to_string() ].next()
    // With hypothetical syntax || for flatten
    // With hypothetical syntax to return iterator not Vec??

    // Linear code:
    //for arg in std::env::args() {
    //    if let Some(log_opts) = arg.strip_prefix(prefix) {
    //        return Some(log_opts.to_string());
    //    }
    //}
    //None
}

/// Arguments:
///  --rust-log=...
///  --debug-coding=...
///  --start-at=...
pub async fn run<Gamedata: gamedata::BaseGamedata>()
{
    if let Some(log_opts) = get_arg("--rust-log=") {
        crate::logging::enable_logging(&log_opts);
    }

    let mut engine = Engine::<Gamedata>::new();

    loop {
        engine.do_frame().await;
        macroquad::prelude::next_frame().await;
    }
}
