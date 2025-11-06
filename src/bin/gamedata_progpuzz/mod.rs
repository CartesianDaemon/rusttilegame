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
mod obj_properties;

use crate::engine::for_gamedata::*;

#[derive(Debug)]
pub struct ProgpuzzGamedata {
    levset: levels::ProgpuzzLevset,
}

// Need to move into scripts?
pub type ProgpuzzCustomProps = obj_scripting_properties::DefaultCustomProps;

impl BaseGamedata for ProgpuzzGamedata {
    type Scripts = super::scripts_progpuzz::ProgpuzzScripts;
    type CustomProps = ProgpuzzCustomProps;

    fn new() -> Self {
        ProgpuzzGamedata {
            levset: levels::ProgpuzzLevset::new()
        }
    }

    fn advance_pane(&mut self, continuation: PaneEnding) {
        self.levset.advance_pane(continuation)
    }

    fn load_pane(&self) -> Pane::<Self::CustomProps> {
        self.levset.load_pane()
    }
}
