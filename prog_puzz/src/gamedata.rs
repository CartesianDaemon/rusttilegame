use super::game_logic::{ProgpuzzGameLogic, ProgpuzzCustomProps};
use super::levels;

use tile_engine::for_gamedata::*;

use savegame::*;

#[derive(Debug)]
pub struct ProgpuzzGamedata {
    levset: levels::ProgpuzzLevset,

    // TODO: Or better to store "current level" in a higher layer?
    reload_needed: bool,

    save_game: SaveGame,
}

// TODO: Move into game engine?
// TODO: Support games with solutions other than Prog?
// TODO: High scores.
mod savegame {
    #[derive(Debug)]
    pub struct SaveGame {
        num_levels: u16,
    }

    impl SaveGame {
        pub fn new(num_levels: u16) -> Self {
            let datetime = time::UtcDateTime::now();

            let mut save_game = Self {num_levels};
            save_game.unlock_level(1);
            // TODO: Handle values from previous version?
            save_game.storage().set(&save_game.version_key(), save_game.current_version());
            save_game
        }

        fn current_version(&self) -> &str {
            // Version based on engine versions.
            // Expecting 1.6.1: version with plain "Level1" key for unlock and no solution key.
            // Expecting 1.6.2: version with "Level1_unlocked" and "Level1_solutions"
            return "1.6.1";
        }

        fn version_key(&self) -> &str {
            "version"
        }

        fn level_unlocked_key(&self, lev_idx: u16) -> String {
            format!("Level{lev_idx}_unlocked")
        }

        fn _level_solutions_key(&self, lev_idx: u16) -> String {
            format!("Level{lev_idx}_solutions")
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
