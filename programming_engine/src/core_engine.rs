use crate::gamedata::BaseGameData;
use crate::savegame::BaseSaveGame;

use super::gamedata;
use super::scene::*;
use super::ui::Ui;

/// Overall Engine state.
///
/// Including set of levels in current Engine, and state of current level being played.
///
/// Templated on Game (either a  builtin Game, or a load-from-file Game).
/// Could instead take a &dyn Game trait object so that it could load a Game object
/// from a library, but that probably doesn't help that much.
struct Engine<GameData: BaseGameData> {
    /// State of current game, e.g. which level you've reached.
    pub game_data: GameData,

    /// State of current scene.
    scene: Scene<GameData::MovementLogic>,

    /// Overarching ui. Instantiates different uis for different scenes.
    ui: Ui,
}

impl<GameData: gamedata::BaseGameData> Engine<GameData> {
    pub fn new() -> Engine<GameData> {
        let mut game_data = GameData::new();
        let scene = game_data.load_scene();
        Engine::<GameData> {
            game_data,
            scene,
            ui: Ui::new(),
        }
    }

    /// Collect input. Draw frame. Advance logical Engine state, if tick scheduled.
    /// NB: Move into Ui
    pub async fn do_frame(&mut self) {
        self.ui.do_frame(&mut self.scene, &mut self.game_data).await;

        // Record any during scene
        if let Some(outcome_to_store) = self.scene.consume_outcome_to_store() {
            let lev_idx = self.game_data.get_current_level();
            self.game_data.save_game().store_outcome(
                lev_idx,
                &outcome_to_store.outcome,
                &outcome_to_store. solution
            );
        }

        // If scene concluded, calculcate next scene/level
        if let Some(scene_ending) = self.scene.ready_for_next_level() {
            self.scene = self.game_data.load_next_scene(scene_ending);
        }

        // If scene concluded, or level chooser used goto level, load new scene.
        if self.game_data.reload_needed() {
            self.scene = self.game_data.load_scene();
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
pub async fn run<GameData: gamedata::BaseGameData>()
{
    if let Some(log_opts) = get_arg("--rust-log=") {
        crate::logging::enable_logging(&log_opts);
    }

    let mut engine = Engine::<GameData>::new();

    loop {
        engine.do_frame().await;
        macroquad::prelude::next_frame().await;
    }
}
