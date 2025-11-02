/// This direcrtory contains scripts defining common enemy types etc.
///
/// It is used in testing the game engine and writing a game.
///
/// It is written in rust but intended to be easy for modders to edit.
///
/// This is Copyright Jack Vickeridge (CartesianDaemon on github), released
/// under the same terms as the pushing puzzle game data (CC-like).
///
/// TODO: Move some subset of scripts to a tile_engine_demo_scripts
/// directory, released as open-source like the game engine, to use for
/// testing, seeding new games, etc.

mod movement_logic;

// TODO: Imports expected by game engine: Pass, AI, Effect.
// Make Engine or Gamedata template on a class with a trait exposing those.

// Called by game engine
pub use movement_logic::*;
