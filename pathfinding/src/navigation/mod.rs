mod visualization;

use gridthorn::grid::{
    ChunkSize, GridCell, GridFootprint, GridObjectId, GridPoint, GridProjection, NavigationBounds,
    PathStatus, PlacementMap, TileLayerId, TileMap, search_path,
};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let layer = TileLayerId(0);
    let mut terrain = TileMap::new(ChunkSize::new(4, 4)?);
    terrain.add_layer(layer)?;
    for column in -3..=3 {
        for row in -3..=3 {
            terrain.set_tile(
                layer,
                GridCell::new(column, row),
                Some(if row == 0 { 8_u32 } else { 1 }),
            )?;
        }
    }
    let mut occupancy = PlacementMap::new();
    occupancy.place(
        GridObjectId(1),
        GridCell::default(),
        GridFootprint::new([
            GridCell::new(0, -1),
            GridCell::default(),
            GridCell::new(0, 1),
        ])?,
    )?;
    let bounds = NavigationBounds::new(GridCell::new(-3, -3), GridCell::new(3, 3))?;
    let cost = |cell| {
        if occupancy.object_at(cell).is_some() {
            None
        } else {
            terrain.tile(layer, cell).copied()
        }
    };
    let result = search_path(bounds, GridCell::new(-3, 0), GridCell::new(3, 0), 49, cost)?;
    assert_eq!(result.status, PathStatus::Found);
    assert_eq!(
        result,
        search_path(bounds, GridCell::new(-3, 0), GridCell::new(3, 0), 49, cost)?
    );
    assert!(
        result
            .path
            .iter()
            .all(|cell| occupancy.object_at(*cell).is_none())
    );
    let limited = search_path(bounds, GridCell::new(-3, 0), GridCell::new(3, 0), 8, cost)?;
    assert_eq!(limited.status, PathStatus::BudgetExceeded);
    let output = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../target/pathfinding");
    std::fs::create_dir_all(&output)?;
    for (name, projection) in [
        (
            "square",
            GridProjection::square(40.0, GridPoint::new(200.0, 180.0))?,
        ),
        (
            "isometric",
            GridProjection::isometric(60.0, 30.0, GridPoint::new(280.0, 160.0))?,
        ),
    ] {
        visualization::write(
            &output.join(format!("{name}.svg")),
            projection,
            &result,
            &occupancy,
        )?;
        visualization::write(
            &output.join(format!("{name}-budget.svg")),
            projection,
            &limited,
            &occupancy,
        )?;
    }
    println!(
        "{:?}: cost {:?}, {} settled cells; SVG diagnostics: {}",
        result.status,
        result.cost,
        result.visited.len(),
        output.display()
    );
    Ok(())
}
