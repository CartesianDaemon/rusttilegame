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
pub mod simple_custom_props;
mod map_coords;
mod render;
mod map;
mod gamedata;
pub mod input; // For engine_tests in push_puzz??
mod pane;
mod obj;

// Used in main() function
pub use core_engine::run;

// Engine exports needed for writing game scripts.
// NB breadcrumb: Merge with gamedata. May or may not become sub-module.
pub mod for_scripting {
    pub use super::map::{Map, RosterIndex};
    pub use super::map_coords::*;
    pub use super::pane::{PaneContinuation, PaneConclusion};
    pub use super::pane::{Prog, Instr};
    pub use super::gamedata::BaseMovementLogic;
    pub use super::simple_custom_props::*;
}

// Engine exports needed for writing game data.
// TODO: Some only used by pushing puzzle, not programming puzzle?
// TODO: For some types, put types in GameData already parameterised by appropritate custom types?
//#[allow(unused_imports)]
pub mod for_gamedata {
    pub use super::obj::*;
    pub use super::gamedata::*;
    pub use super::pane::{Pane, PaneConclusion};
    pub use super::pane::{Split, Arena, Code};
    pub use super::map_coords::*;
    pub use super::simple_custom_props::*;
}

pub mod for_testing {
    pub use crate::input::Input;
}
