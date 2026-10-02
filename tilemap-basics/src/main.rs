//! Headless public SDK tilemap demonstration.

mod tiles;

fn main() -> Result<(), gridthorn::grid::TileMapError> {
    tiles::run()
}
