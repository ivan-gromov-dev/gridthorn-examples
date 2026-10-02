use gridthorn::grid::{GridCell, GridError, GridPoint, GridProjection};

/// Demonstrates projection and inverse conversion without window resources.
pub fn run() -> Result<(), GridError> {
    let origin = GridPoint::new(320.0, 100.0);
    for (name, projection) in [
        ("square", GridProjection::square(32.0, origin)?),
        ("isometric", GridProjection::isometric(64.0, 32.0, origin)?),
    ] {
        for cell in [GridCell::new(0, 0), GridCell::new(3, -2)] {
            let center = projection.cell_center(cell)?;
            let selected = projection.cell_at(center)?;
            assert_eq!(selected, cell);
            println!("{name}: {cell:?} -> {center:?} -> {selected:?}");
        }
    }
    Ok(())
}
