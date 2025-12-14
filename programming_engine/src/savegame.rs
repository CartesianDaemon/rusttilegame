use crate::for_gamedata::Prog;
use quad_timestamp::*;
use chrono::*;

pub trait SaveGame {
    // Levels available to go to, if levels are identified by numeric index. Else empty set.
    fn get_unlocked_levels(&self) -> std::collections::HashSet<u16> {
        unimplemented!()
    }

    fn unlock_level(&self, _lev_idx: u16) {
        unimplemented!()
    }
}

pub trait UnimplementedSaveGame : SaveGame {
    // Levels available to go to, if levels are identified by numeric index. Else empty set.
    fn get_unlocked_levels(&self) -> std::collections::HashSet<u16> {
        std::collections::HashSet::new()
    }

    fn unlock_level(&self, _lev_idx: u16) {
    }
}

#[derive(Debug)]
pub struct DefaultProgSaveGameData {
    num_levels: u16,
}

impl DefaultProgSaveGameData {

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

    fn _datetime_str(&self) -> String {
        let datetime = DateTime::<chrono::Utc>::from_timestamp_secs(timestamp_utc().unwrap()).unwrap();
        log::debug!("Timestamp: {datetime}");
        datetime.to_string()
    }
}

pub trait DefaultProgSaveGame {
    fn new_data(num_levels: u16) -> DefaultProgSaveGameData {
        let mut save_game_data = DefaultProgSaveGameData {num_levels};
        Self::unlock_level(&mut save_game_data, 1);
        // TODO: Handle values from previous version?
        save_game_data.storage().set(&save_game_data.version_key(), save_game_data.current_version());
        save_game_data
    }

    fn unlock_level(save_game_data: &mut DefaultProgSaveGameData, lev_idx: u16) {
        save_game_data.storage().set(&save_game_data.level_unlocked_key(lev_idx), "unlocked");
    }

    fn get_unlocked_levels(save_game_data: &DefaultProgSaveGameData) -> std::collections::HashSet<u16> {
        (1..save_game_data.num_levels).filter(|lev_idx| save_game_data.storage().get(&save_game_data.level_unlocked_key(*lev_idx)).is_some()).collect()
    }

    fn store_outcome(save_game_data: &DefaultProgSaveGameData, lev_idx: u16, datetime: DateTime<chrono::Utc>, solution: &Prog) {
        let key = &save_game_data._level_outcomes_key(lev_idx);
        let prev_val = save_game_data.storage().get(key).unwrap_or_default();
        save_game_data.storage().set(key, &format!("{prev_val}{datetime} ({}): {solution}\n", save_game_data.current_version()));
    }
}
