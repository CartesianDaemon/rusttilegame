/// Helpers which define movement logic and object properties for a range of games.
///
/// Somewhere between game engine and game data. Something like "game scripting".
///
/// Also GPLv3, but might add caveats.

mod movement_logic;
mod obj_types;

// Used in game-specific instantiation.
pub use obj_types::*;

// Called by game engine
pub use movement_logic::*;
