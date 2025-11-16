use super::game_logic::{ProgpuzzGameLogic, ProgpuzzCustomProps};
use super::levels;

use tile_engine::for_gamedata::*;

#[derive(Debug)]
pub struct ProgpuzzGamedata {
    levset: levels::ProgpuzzLevset,
}

impl BaseGamedata for ProgpuzzGamedata {
    type GameLogic = ProgpuzzGameLogic;
    type CustomProps = ProgpuzzCustomProps;

    fn new() -> Self {
        ProgpuzzGamedata {
            levset: levels::ProgpuzzLevset::new()
        }
    }

    fn advance_pane(&mut self, continuation: WidgetConclusion) {
        self.levset.advance_pane(continuation)
    }

    fn load_pane(&self) -> Widget::<Self::GameLogic> {
        self.levset.load_pane()
    }
}
