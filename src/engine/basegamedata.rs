/// Trait for interface needed for Games implemented in the Engine

use super::scene::{Scene, SceneEnding};

/// Manages game-specific state, e.g. which level to go to next.
pub trait BaseGamedata {
    fn new_game() -> Self;

    fn advance_scene(&mut self, continuation: SceneEnding);

    fn load_scene(&self) -> Scene;

    fn load_next_scene(&mut self, continuation: SceneEnding) -> Scene {
        self.advance_scene(continuation);
        self.load_scene()
    }
}
