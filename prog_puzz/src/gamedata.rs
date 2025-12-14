use super::game_logic::{ProgpuzzGameLogic, ProgpuzzCustomProps};
use super::levels;

use tile_engine::for_gamedata::*;

#[derive(Debug)]
pub struct ProgpuzzGameData {
    levset: levels::ProgpuzzLevset,

    // TODO: Or better to store "current level" in a higher layer?
    reload_needed: bool,

    save_game_data: DefaultProgSaveGameData,
}

impl SaveGame for ProgpuzzGameData {
}

impl DefaultProgSaveGame for ProgpuzzGameData {
}

impl BaseGameData for ProgpuzzGameData {
    type GameLogic = ProgpuzzGameLogic;
    type CustomProps = ProgpuzzCustomProps;

    fn new() -> Self {
        let levset = levels::ProgpuzzLevset::new();
        let num_levels = levset.num_levels();
        ProgpuzzGameData {
            levset,
            reload_needed: false,
            save_game_data: <ProgpuzzGameData as DefaultProgSaveGame>::new_data(num_levels),
        }
    }

    fn advance_scene(&mut self, continuation: SceneConclusion) {
        self.levset.advance_scene(continuation);
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
        self.save_game_data.get_unlocked_levels()
    }

    fn goto_level(&mut self, lev_idx: u16) {
        log::debug!("Progpuzz going to level {lev_idx}");
        self.levset.goto_level(lev_idx);
        self.reload_needed = true;
    }
}
