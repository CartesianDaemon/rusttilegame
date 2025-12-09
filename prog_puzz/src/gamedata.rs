use super::game_logic::{ProgpuzzGameLogic, ProgpuzzCustomProps};
use super::levels;

use tile_engine::for_gamedata::*;

use std::collections::HashSet;

#[derive(Debug)]
pub struct ProgpuzzGamedata {
    levset: levels::ProgpuzzLevset,

    unlocked_levels: HashSet<u16>,
    // TODO: Or better to store "current level" in a higher layer?
    reload_needed: bool,
}

impl BaseGamedata for ProgpuzzGamedata {
    type GameLogic = ProgpuzzGameLogic;
    type CustomProps = ProgpuzzCustomProps;

    fn new() -> Self {
        ProgpuzzGamedata {
            levset: levels::ProgpuzzLevset::new(),
            unlocked_levels: [1].into(),
            reload_needed: false,
        }
    }

    fn advance_scene(&mut self, continuation: SceneConclusion) {
        self.levset.advance_scene(continuation);
        self.unlocked_levels.insert(self.levset.get_current_level());
    }

    fn load_scene(&mut self) -> Scene::<Self::GameLogic> {
        log::debug!("Progpuzz loading scene");
        self.reload_needed = false;
        self.levset.load_scene()
    }

    fn get_level_str(&self) -> String {
        match self.levset.current_levid {
            levels::ProgpuzzSceneId::LevCodingArena(lev_num) => format!("Level: {}", lev_num),
            _ => panic!(),
        }
    }

    fn num_levels(&self) -> u16 {
        self.levset.num_levels()
    }

    fn get_current_level(&self) -> u16 {
        self.levset.get_current_level()
    }

    fn reload_needed(&self) -> bool {
        self.reload_needed
    }

    fn get_unlocked_levels(&self) -> std::collections::HashSet<u16> {
        self.unlocked_levels.clone()
    }

    fn goto_level(&mut self, lev_idx: u16) {
        log::debug!("Progpuzz going to level {lev_idx}");
        self.levset.goto_level(lev_idx);
        self.reload_needed = true;
    }
}
