/// Trait for interface needed for Games implemented in the Engine

use super::scene::{Scene, SceneEnding};

// Trait for scripts which the scripts for each game needs to implement.
// TODO: Move to separate file??
use super::map::Map;
use super::map::RosterIndex;
use super::for_gamedata::Cmd;
use super::scene::SceneContinuation;
pub trait BaseMovementLogic {
    fn move_mov(field: &mut Map, mov: RosterIndex, cmd: Cmd) -> SceneContinuation;
}
pub trait BaseScripts {
    type MovementLogic : BaseMovementLogic;
}

/// Manages game-specific state, e.g. which level to go to next.
pub trait BaseGamedata {
    type Scripts : BaseScripts;
    type XAI;

    fn new() -> Self;

    fn advance_scene(&mut self, continuation: SceneEnding);

    fn load_scene(&self) -> Scene;

    fn load_next_scene(&mut self, continuation: SceneEnding) -> Scene {
        self.advance_scene(continuation);
        self.load_scene()
    }
}
