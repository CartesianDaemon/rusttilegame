/// This directory contains the core game engine.
///
/// This is Copyright Jack Vickeridge (CartesianDaemon on github) but
/// licensed for other people to use under LGPL3 or later terms. (TBD:
/// Clarify details, allow pepole to ship combined binary with their
/// own copyrighted gamedata.)
///
/// Maybe: Move engine into a lib.

mod core_engine;
mod map_coords;
mod render;
mod map;
mod gamedata;
mod input;
mod scene;
mod obj;
mod obj_scripting_properties;

// Used in main() function
pub use core_engine::run;

// Engine exports needed for writing game scripts.
pub mod for_scripting {
    pub use super::obj::*;
    pub use super::map::{Map, RosterIndex};
    pub use super::map_coords::*;
    pub use super::scene::{SceneContinuation, SceneEnding};
    pub use super::gamedata::{BaseScripts, BaseMovementLogic};
    pub use crate::engine::obj_scripting_properties::*;
}

// Engine exports needed for writing game data.
// TODO: Some only used by pushing puzzle, not programming puzzle?
#[allow(unused_imports)]
pub mod for_gamedata {
    pub use super::obj::*;
    pub use super::gamedata::*;
    pub use super::scene::{Scene, SceneEnding};
    pub use super::map_coords::*;
    pub use crate::engine::obj_scripting_properties::*;
}

mod engine_tests;
