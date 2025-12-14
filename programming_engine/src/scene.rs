// Game data moves between possible scene types, including levels, splash screens and menus.
//
// Top-level Scenes can incorporate widgets or other Scenes.

mod scene_base;
pub mod arena;
pub mod coding;
pub mod splash;
pub mod coding_arena;

pub use scene_base::*;
