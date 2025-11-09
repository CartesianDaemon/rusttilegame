mod pushpuzz;
#[cfg(test)]
mod push_puzz_tests;

#[macroquad::main("Tile Game")]
async fn main() {
    tile_engine::run::<pushpuzz::PushpuzzGamedata>().await;
}
