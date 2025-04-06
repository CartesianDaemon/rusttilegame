/// This direcrtory contains scripts defining common enemy types etc.
///
/// It is used in testing the game engine and writing a game.
///
/// It is written in rust but intended to be easy for modders to edit.
///
/// This is Copyright Jack Vickeridge (CartesianDaemon on github). It is
/// more like an artistic work than a program, but it is licensed for other
/// people to use or adapt under the same terms as the game engine (GPLv3).

// TODO: Rename to "base scripts" or "reptonlike scripts"

mod movement_logic;
mod obj_types;

// Used in game-specific instantiation.
pub use obj_types::*;

// Called by game engine
pub use movement_logic::*;
