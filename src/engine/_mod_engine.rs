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
mod gametrait;
mod input;
#[path = "scene/_mod_scene.rs"] mod scene;
mod obj;

pub use engine::Engine;

pub mod scripting {
    pub use super::obj::Obj;
    pub use super::field::{Field, RichMapHandle};
    pub use super::map_coords::*;
    pub use super::scene::{SceneEnding, Continuation};
}

// Public interface for writing a custom game
pub mod customgame {
    pub use super::obj::Obj;
    pub use super::gametrait::*;
    pub use super::scene::{Scene, Continuation};
    pub use super::map_coords::*;
}

#[path = "tests/_mod_tests.rs"] mod engine_tests;
