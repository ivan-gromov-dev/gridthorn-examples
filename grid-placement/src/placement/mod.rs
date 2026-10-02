use gridthorn::grid::{
    ChunkSize, GridCell, GridFootprint, GridObjectId, GridPoint, GridProjection, PlacementError,
    PlacementMap, TileLayerId, TileMap,
};

/// Exercises terrain validation, preview, occupancy picking, and atomic moves.
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let layer = TileLayerId(0);
    let mut terrain = TileMap::new(ChunkSize::new(16, 16)?);
    terrain.add_layer(layer)?;
    for column in -2..=3 {
        terrain.set_tile(layer, GridCell::new(column, 0), Some(true))?;
    }
    terrain.set_tile(layer, GridCell::new(3, 0), Some(false))?;
    let footprint = GridFootprint::new([GridCell::new(0, 0), GridCell::new(1, 0)])?;
    let building = GridObjectId(1);
    let obstacle = GridObjectId(2);
    for projection in [
        GridProjection::square(32.0, GridPoint::new(0.0, 0.0))?,
        GridProjection::isometric(64.0, 32.0, GridPoint::new(0.0, 0.0))?,
    ] {
        let mut occupancy = PlacementMap::new();
        let anchor = projection.cell_at(projection.cell_center(GridCell::new(-1, 0))?)?;
        let preview = occupancy.validate(building, anchor, &footprint)?;
        assert!(
            preview
                .iter()
                .all(|cell| terrain.tile(layer, *cell) == Some(&true))
        );
        assert!(occupancy.object_at(anchor).is_none());
        occupancy.place(building, anchor, footprint.clone())?;
        let picked_cell = projection.cell_at(projection.cell_center(GridCell::new(0, 0))?)?;
        assert_eq!(occupancy.object_at(picked_cell), Some(building));
        occupancy.place(obstacle, GridCell::new(2, 0), GridFootprint::single_cell())?;
        assert!(matches!(
            occupancy.relocate(building, GridCell::new(1, 0), footprint.clone()),
            Err(PlacementError::Occupied { .. })
        ));
        assert_eq!(occupancy.placement(building).unwrap().anchor(), anchor);
        let blocked_terrain = footprint.cells_at(GridCell::new(2, 0))?;
        assert!(
            !blocked_terrain
                .iter()
                .all(|cell| terrain.tile(layer, *cell) == Some(&true))
        );
        occupancy.relocate(building, GridCell::new(0, 0), footprint.clone())?;
        assert_eq!(occupancy.object_at(anchor), None);
        occupancy.remove(building);
        assert_eq!(occupancy.object_at(picked_cell), None);
        println!(
            "placement preview, picking, rollback, move, and removal passed for {projection:?}"
        );
    }
    Ok(())
}
