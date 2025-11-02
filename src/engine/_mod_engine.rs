/// This directory contains the core game engine.
///
/// This is Copyright Jack Vickeridge (CartesianDaemon on github) but
/// licensed for other people to use under LGPL3 or later terms. (TBD:
/// Clarify details, allow pepole to ship combined binary with their
/// own copyrighted gamedata.)

// TODO: Try to tidy up so:
//          Engine is templated on gamedata. No reference to scripts.
//          Everything else can import (some part of?) engine.
//          Gamedata imports one or more scripts modules.
//          Engine tests might import scripts as helpers.
// TODO: Try to tidy up what types/functions each module exports.
// TODO: Remove warning on redundant braces in use statements.
// TODO: Give permanent names to biobot game and engine.
//
// TODO: Remove "emphasized items" for changed code?

mod engine;
mod map_coords;
mod render;
mod field;
mod basegamedata;
mod input;
#[path = "scene/_mod_scene.rs"] mod scene;
mod obj;
mod obj_scripting_properties;

// Used in main() function
pub use engine::run;

// Engine exports needed for writing game scripts.
pub mod for_scripting {
    pub use super::obj::ObjProperties;
    pub use super::field::{Field, RosterIndex};
    pub use super::map_coords::*;
    pub use super::scene::{SceneContinuation, SceneEnding};
    pub use super::engine::BaseScripts;
    pub use crate::engine::obj_scripting_properties::*;
}

// Engine exports needed for writing game data.
// TODO: Some only used by pushing puzzle, not programming puzzle?
#[allow(unused_imports)]
pub mod customgame {
    pub use super::obj::ObjProperties;
    pub use super::basegamedata::*;
    pub use super::scene::{Scene, SceneEnding};
    pub use super::map_coords::*;
    pub use crate::engine::obj_scripting_properties::*;
}

#[path = "tests/_mod_tests.rs"] mod engine_tests;
