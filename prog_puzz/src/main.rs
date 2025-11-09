/// This directory contains the artistic parts of my "Biobot Game"
/// game in progress. e.g. gameplay, plot, art, characters, dialogue,
/// level layout, etc.
///
/// This is Copyright Jack Vickeridge (CartesianDaemon on github) but is
/// an artistic work and not released as free software.
///
/// I believe the following uses fall under fair use or corresponding
/// doctrines and am happy for it to be used in these ways:
/// * Producing non-commercial fanworks.
/// * Testing that the game engine builds correctly.
/// * Writing a new game using the same file layout.
/// * Small-scale educational use.
/// * Incorporating as a very small part of another artistic work.
///
/// As long as the game is under development you are encouraged to arena it.
/// If I finish the game I will specify how the game is distributed.

mod gamedata;
mod levels;
mod objs;
mod movement_logic;

#[cfg(test)]
mod prog_puzz_tests;

use gamedata::*;

#[macroquad::main("Tile Game")]
async fn main() {
    tile_engine::run::<ProgpuzzGamedata>().await;
}
