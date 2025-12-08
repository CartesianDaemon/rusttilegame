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

    fn advance_scene(&mut self, continuation: SceneConclusion) {
        self.levset.advance_scene(continuation)
    }

    fn load_scene(&self) -> Scene::<Self::GameLogic> {
        self.levset.load_scene()
    }

    fn get_level_str(&self) -> String {
        match self.levset.current_levid {
            levels::ProgpuzzPaneId::LevCodingArena(lev_num) => format!("Level: {}", lev_num),
            _ => panic!(),
        }
    }

    fn num_levels(&self) -> u16 {
        self.levset.num_levels()
    }

    fn goto_level(&mut self, lev_idx: u16) {
        self.levset.goto_level(lev_idx);
    }
}
