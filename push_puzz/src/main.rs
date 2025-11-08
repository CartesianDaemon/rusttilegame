// Breadcrumb: Avoid warnings of unused code in each separare binary.
// Try using lib crate + two binary crates in a workspace?

mod pushpuzz;
mod engine_tests;

#[macroquad::main("Tile Game")]
async fn main() {
    tile_engine::run::<pushpuzz::PushpuzzGamedata>().await;
}
