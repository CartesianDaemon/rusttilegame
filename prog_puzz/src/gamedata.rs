use super::movement_logic::{ProgpuzzMovementLogic, ProgpuzzCustomProps};
use super::levels;

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
