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
/// As long as the game is under development you are encouraged to arena it.
/// If I finish the game I will specify how the game is distributed.

// NB: Consider moving types back into a file like mod.rs?

mod levels;
mod objs;
mod movement_logic;

use movement_logic::{ProgpuzzMovementLogic, ProgpuzzCustomProps};

use tile_engine::for_gamedata::*;

#[derive(Debug)]
pub struct ProgpuzzGamedata {
    levset: levels::ProgpuzzLevset,
}

impl BaseGamedata for ProgpuzzGamedata {
    type MovementLogic = ProgpuzzMovementLogic;
    type CustomProps = ProgpuzzCustomProps;

    fn new() -> Self {
        ProgpuzzGamedata {
            levset: levels::ProgpuzzLevset::new()
        }
    }

    fn advance_pane(&mut self, continuation: PaneConclusion) {
        self.levset.advance_pane(continuation)
    }

    fn load_pane(&self) -> Pane::<Self::MovementLogic> {
        self.levset.load_pane()
    }
}

#[macroquad::main("Tile Game")]
async fn main() {
    tile_engine::run::<ProgpuzzGamedata>().await;
}
