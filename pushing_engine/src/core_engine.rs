use crate::gamedata::BaseGamedata;

use super::gamedata;
use super::scene::*;
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
    state: Scene<Gamedata::GameLogic>,

    ///
    ui: UiBase,
}

impl<Gamedata: gamedata::BaseGamedata> Engine<Gamedata> {
    pub fn new() -> Engine<Gamedata> {
        let gamedata = Gamedata::new();
        let scene = gamedata.load_scene();
        Engine::<Gamedata> {
            gamedata: gamedata,
            state: scene,
            ui: UiBase::new(),
        }
    }

    /// Collect input. Draw frame. Advance logical Engine state, if tick scheduled.
    /// NB: Move into Ui
    pub async fn do_frame(&mut self) {
        let scene_continuation = self.ui.do_frame(&mut self.state, &self.gamedata).await;
        if let SceneContinuation::Break(scene_ending) = scene_continuation {
            self.state = self.gamedata.load_next_pane(scene_ending);
        }
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
