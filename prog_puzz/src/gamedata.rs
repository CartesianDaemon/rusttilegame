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
    num_levels: u16,
}

use chrono::prelude::*;
impl SaveGame {
    fn new(num_levels: u16) -> Self {
        let mut save_game = Self {num_levels};
        save_game.unlock_level(1);
        save_game.storage().set(&save_game.version_key(), "1.6.1");
        save_game
    }

    fn version_key(&self) -> &str {
        "version"
    }

    fn level_unlocked_key(&self, lev_idx: u16) -> String {
        format!("Level_unlocked{lev_idx}")
    }

    fn level_solutions_key(&self, lev_idx: u16) -> String {
        format!("Level_solutions{lev_idx}")
    }

    fn storage(&self) -> std::sync::MutexGuard<quad_storage::LocalStorage> {
        quad_storage::STORAGE.lock().unwrap()
    }

    pub fn unlock_level(&mut self, lev_idx: u16) {
        self.storage().set(&self.level_unlocked_key(lev_idx), "unlocked");
    }

    pub fn get_unlocked_levels(&self) -> std::collections::HashSet<u16> {
        (1..self.num_levels).filter(|lev_idx| self.storage().get(&self.level_unlocked_key(*lev_idx)).is_some()).collect()
    }

    pub fn _store_solution(&self, lev_idx: u16, datetime: DateTime<Local>, solution: &Subprog) {
        let key = &self.level_solutions_key(lev_idx);
        let prev = self.storage().get(key).unwrap_or_default();
        self.storage().set(key, &format!("{prev}{datetime}: {solution}\n"));
    }
}

impl BaseGamedata for ProgpuzzGamedata {
    type GameLogic = ProgpuzzGameLogic;
    type CustomProps = ProgpuzzCustomProps;

    fn new() -> Self {
        let levset = levels::ProgpuzzLevset::new();
        let num_levels = levset.num_levels();
        ProgpuzzGamedata {
            levset,
            reload_needed: false,
            save_game: SaveGame::new(num_levels),
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
        self.save_game.get_unlocked_levels()
    }

    fn goto_level(&mut self, lev_idx: u16) {
        log::debug!("Progpuzz going to level {lev_idx}");
        self.levset.goto_level(lev_idx);
        self.reload_needed = true;
    }
}
