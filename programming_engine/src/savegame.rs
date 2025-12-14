use crate::for_gamedata::Prog;
use quad_timestamp::*;
use chrono::*;

#[derive(Debug)]
pub struct SaveGame {
    num_levels: u16,
}

impl SaveGame {
    pub fn new(num_levels: u16) -> Self {
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

    fn _level_outcomes_key(&self, lev_idx: u16) -> String {
        format!("Level{lev_idx}_outcomes")
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

    fn _datetime_str(&self) -> String {
        let datetime = DateTime::<chrono::Utc>::from_timestamp_secs(timestamp_utc().unwrap()).unwrap();
        log::debug!("Timestamp: {datetime}");
        datetime.to_string()
    }

    pub fn store_outcome(&self, lev_idx: u16, datetime: DateTime<chrono::Utc>, solution: &Prog) {
        let key = &self._level_outcomes_key(lev_idx);
        let prev_val = self.storage().get(key).unwrap_or_default();
        self.storage().set(key, &format!("{prev_val}{datetime} ({}): {solution}\n", self.current_version()));
    }
}
