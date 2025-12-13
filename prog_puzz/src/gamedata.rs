use super::game_logic::{ProgpuzzGameLogic, ProgpuzzCustomProps};
use super::levels;

use tile_engine::for_gamedata::*;

#[derive(Debug)]
pub struct ProgpuzzGamedata {
    levset: levels::ProgpuzzLevset,

    // TODO: Or better to store "current level" in a higher layer?
    reload_needed: bool,

    save_game: SaveGame,
}

#[derive(Debug)]
pub struct SaveGame {
}

impl SaveGame {
    fn new() -> Self {
        let mut save_game = Self {};
        save_game.unlock_level(1);
        save_game
    }

    fn level_unlocked_key(&self, lev_idx: u16) -> String {
        format!("Level{lev_idx}")
    }

    fn unlock_level(&mut self, lev_idx: u16) {
        let storage = &mut quad_storage::STORAGE.lock().unwrap();
        storage.set(&self.level_unlocked_key(lev_idx), "unlocked");
    }
}

impl BaseGamedata for ProgpuzzGamedata {
    type GameLogic = ProgpuzzGameLogic;
    type CustomProps = ProgpuzzCustomProps;

    fn new() -> Self {
        ProgpuzzGamedata {
            levset: levels::ProgpuzzLevset::new(),
            reload_needed: false,
            save_game: SaveGame::new(),
        }
    }

    fn advance_scene(&mut self, continuation: SceneConclusion) {
        self.levset.advance_scene(continuation);
        self.save_game.unlock_level(self.levset.get_current_level());
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
        let storage = &mut quad_storage::STORAGE.lock().unwrap();
        (1..self.num_levels()).filter(|lev_idx| storage.get(&self.save_game.level_unlocked_key(*lev_idx)).is_some()).collect()
    }

    fn goto_level(&mut self, lev_idx: u16) {
        log::debug!("Progpuzz going to level {lev_idx}");
        self.levset.goto_level(lev_idx);
        self.reload_needed = true;
    }
}
