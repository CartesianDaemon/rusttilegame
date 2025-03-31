/// This directory contains the game engine.
///
/// This is Copyright Jack Vickeridge (CartesianDaemon on github) but
/// licensed for other people to use under GPL3 or later terms.
///
/// My main intention is that anyone can build on this engine or use
/// it to make their own games, as long as I am credited a proportionate
/// amount for the design of the engine, and any further development is
/// equally freely available for other people to use.
///
/// I don't really expect it to come up, but my intention is that you can
/// use the engine to develop commercial games if you want. And you can keep
/// copyright over the gameplay, art, plot, etc. But modifications to the
/// engine stay open source. But I haven't thought through the details yet --
/// ask me if it comes up.
///
/// I think of the engine and the game design as two separate crates even
/// though I haven't implemented that yet.

// Non-public
mod engine;
mod map_coords;
mod render;
mod field;
// Public for tests. Not needed if tests move back into this dir?
pub mod gametrait;
pub mod input;
pub mod play;
pub mod obj;

pub use engine::Engine;
pub use gametrait::{Game, LevelNumBase, LevelNum};
pub use map_coords::*;

// For tests?
pub use gametrait::*;

#[path = "tests/_mod.rs"]
mod engine_tests;
