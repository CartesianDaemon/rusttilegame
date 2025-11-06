/// This direcrtory contains scripts defining common enemy types etc.
///
/// It is used in testing the game engine and writing a game.
///
/// It is written in rust but intended to be easy for modders to edit.
///
/// This is Copyright Jack Vickeridge (CartesianDaemon on github), released
/// under the same terms as the pushing puzzle game data (CC-like).

mod movement_logic;

pub use movement_logic::ProgpuzzMovementLogic;
