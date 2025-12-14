/// This directory contains the core game engine.
///
/// This is Copyright Jack Vickeridge (CartesianDaemon on github) but
/// licensed for other people to use under LGPL3 or later terms. (TBD:
/// Clarify details, allow pepole to ship combined binary with their
/// own copyrighted gamedata.)
///
/// Maybe: Move engine into a lib.

// TODO Breadcrumb: Need to check that imgs still work ok after moving everything into crates.
// TODO: Need to move assets into each game folder?

mod core_engine;
mod gamedata;
mod logging;
mod map_coords;
mod savegame;
mod scene;
mod obj;
mod ui;
pub mod simple_custom_props;

// Used in main() function
pub use core_engine::run;

// Engine exports needed for writing game data.
// NB: Check which things ought to be exported, which shouldn't be needed.
pub mod for_gamedata {
    pub use super::obj::*;
    pub use super::gamedata::*;
    pub use super::scene::*;
    pub use super::map_coords::*;
    pub use super::simple_custom_props::*;
    pub use super::scene::Arena;
    pub use super::scene::arena::RosterIndex;
    pub use super::savegame::SaveGame;
}

pub mod infra {
    pub use crate::core_engine::get_arg;
    pub use crate::logging::log_builder;
    pub use crate::logging::initialise_logging_for_tests;
}
