mod pushpuzz;

// TODO: Reinstate tests once compile errors from during prog puzz development are fixed.
#[cfg(any())]
#[cfg(test)]
mod push_puzz_tests;

#[macroquad::main("Tile Game")]
async fn main() {
    tile_engine::run::<pushpuzz::PushpuzzGameData>().await;
}
