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
mod map_coords;
pub mod input; // For engine_tests in push_puzz??
mod pane;
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
    pub use super::pane::*;
    pub use super::map_coords::*;
    pub use super::simple_custom_props::*;
    pub use super::pane::Arena;
    pub use super::pane::pane_arena::RosterIndex;
}
