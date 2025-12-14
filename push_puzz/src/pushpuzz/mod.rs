/// This directory contains the artistic parts of my "Biobot Game"
/// game in progress. e.g. gameplay, plot, art, characters, dialogue,
/// level layout, etc.
///
/// This is Copyright Jack Vickeridge (CartesianDaemon on github) but is
/// an artistic work and not released as free software.
///
/// I believe the following uses fall under fair use or corresponding
/// doctrines and am happy for it to be used in these ways:
/// * Producing non-commercial fanworks.
/// * Testing that the game engine builds correctly.
/// * Writing a new game using the same file layout.
/// * Small-scale educational use.
/// * Incorporating as a very small part of another artistic work.
///
/// As long as the game is under development you are encouraged to play it.
/// If I finish the game I will specify how the game is distributed.

// TODO: Move imgs into data. Test that builds and preview html still work.

mod levels;
mod objs;

mod movement_logic;

pub use movement_logic::PushpuzzMovementLogic;

use tile_engine::for_gamedata::*;

#[derive(Debug)]
pub struct PushpuzzGameData {
    levset: levels::PushpuzzLevset,
}

impl BaseGameData for PushpuzzGameData {
    type MovementLogic = PushpuzzMovementLogic;
    type CustomProps = tile_engine::simple_custom_props::SimpleCustomProps;

    fn new() -> Self {
        PushpuzzGameData {
            levset: levels::PushpuzzLevset::new()
        }
    }

    fn advance_scene(&mut self, continuation: SceneConclusion) {
        self.levset.advance_scene(continuation)
    }

    fn load_scene(&self) -> Scene<Self::MovementLogic> {
        self.levset.load_scene()
    }
}
