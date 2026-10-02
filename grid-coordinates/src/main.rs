//! Headless public SDK coordinate demonstration.

mod coordinates;

fn main() -> Result<(), gridthorn::grid::GridError> {
    coordinates::run()
}
