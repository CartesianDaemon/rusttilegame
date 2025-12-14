use crate::for_gamedata::Prog;
use quad_timestamp::*;
use chrono::*;

pub trait BaseSaveGame : std::fmt::Debug {
    // Levels available to go to, if levels are identified by numeric index. Else empty set.
    fn get_unlocked_levels(&self) -> std::collections::HashSet<u16>;

    fn unlock_level(&mut self, _lev_idx: u16);
}

#[derive(Debug)]
pub struct UnimplementedSaveGame;

impl BaseSaveGame for UnimplementedSaveGame {
    // Levels available to go to, if levels are identified by numeric index. Else empty set.
    fn get_unlocked_levels(&self) -> std::collections::HashSet<u16> {
        std::collections::HashSet::new()
    }

    fn unlock_level(&mut self, _lev_idx: u16) {
    }
}

#[derive(Debug)]
pub struct DefaultProgSaveGame {
    num_levels: u16,
}

impl DefaultProgSaveGame {
    pub fn new(num_levels: u16) -> Self {
        let mut save_game_data = Self {num_levels};
        save_game_data.unlock_level(1);
        // TODO: Handle values from previous version?
        save_game_data.storage().set(&save_game_data.version_key(), save_game_data.current_version());
        save_game_data
    }

    fn version_key(&self) -> &str {
        "version"
    }

    fn level_unlocked_key(&self, lev_idx: u16) -> String {
        format!("Level{lev_idx}_unlocked")
    }

    fn _level_outcomes_key(&self, lev_idx: u16) -> String {
        format!("Level{lev_idx}_outcomes")
    }

    fn storage(&self) -> std::sync::MutexGuard<quad_storage::LocalStorage> {
        quad_storage::STORAGE.lock().unwrap()
    }

    fn _datetime_str(&self) -> String {
        let datetime = DateTime::<chrono::Utc>::from_timestamp_secs(timestamp_utc().unwrap()).unwrap();
        log::debug!("Timestamp: {datetime}");
        datetime.to_string()
    }

    fn current_version(&self) -> &str {
        // Version based on engine versions.
        // Expecting 1.6.1: version with plain "Level1" key for unlock and no solution key.
        // Expecting 1.6.2: version with "Level1_unlocked" and "Level1_solutions"
        return "1.6.1";
    }

    fn _store_outcome(&mut self, lev_idx: u16, datetime: DateTime<chrono::Utc>, solution: &Prog) {
        let key = &self._level_outcomes_key(lev_idx);
        let prev_val = self.storage().get(key).unwrap_or_default();
        self.storage().set(key, &format!("{prev_val}{datetime} ({}): {solution}\n", self.current_version()));
    }
}

impl BaseSaveGame for DefaultProgSaveGame {
    fn unlock_level(&mut self, lev_idx: u16) {
        self.storage().set(&self.level_unlocked_key(lev_idx), "unlocked");
    }

    fn get_unlocked_levels(&self) -> std::collections::HashSet<u16> {
        (1..self.num_levels).filter(|lev_idx| self.storage().get(&self.level_unlocked_key(*lev_idx)).is_some()).collect()
    }
}
