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
#[path = "scene/_mod.rs"] mod scene;
mod obj;

// Public interface for starting the engine
pub use engine::Engine;

// Public interface for writing game scripts
pub use obj::Obj;
pub use field::{Field, Map};

// Public interface for writing a custom game
pub use gametrait::*;
pub use scene::{Scene, SceneEnding, Continuation};
pub use map_coords::*;

#[path = "tests/_mod.rs"] mod engine_tests;
