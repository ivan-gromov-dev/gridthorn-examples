use gridthorn::Camera2d;
use gridthorn::grid::{
    ChunkSize, GridCell, GridPoint, GridProjection, GridView, TileLayerId, TileMap, TileMapError,
};

/// Exercises sparse storage, layer pass-through, and camera-aware picking.
pub fn run() -> Result<(), TileMapError> {
    let ground = TileLayerId(0);
    let roof = TileLayerId(10);
    let mut map = TileMap::new(ChunkSize::new(16, 16)?);
    map.add_layer(ground)?;
    map.add_layer(roof)?;
    for cell in [
        GridCell::new(-17, -1),
        GridCell::new(0, 0),
        GridCell::new(16, 2),
    ] {
        map.set_tile(ground, cell, Some("grass"))?;
    }
    let selected = GridCell::new(0, 0);
    map.set_tile(roof, selected, Some("roof"))?;
    for projection in [
        GridProjection::square(32.0, GridPoint::default())?,
        GridProjection::isometric(64.0, 32.0, GridPoint::new(0.0, -16.0))?,
    ] {
        let camera = Camera2d::new([16.0, 16.0], 200.0);
        let view = GridView::new(
            GridPoint::new(f64::from(camera.center()[0]), f64::from(camera.center()[1])),
            f64::from(camera.viewport_height()),
            GridPoint::new(800.0, 600.0),
        )?;
        let center = projection.cell_center(selected)?;
        let cursor = GridPoint::new(
            400.0 + (center.x - 16.0) * 3.0,
            300.0 + (center.y - 16.0) * 3.0,
        );
        let hit = map
            .pick_screen(projection, view, cursor)?
            .expect("occupied roof");
        assert_eq!((hit.layer, hit.cell, *hit.tile), (roof, selected, "roof"));
        map.layer_mut(roof)
            .expect("registered roof")
            .set_visible(false);
        assert_eq!(
            map.pick_screen(projection, view, cursor)?
                .expect("ground below roof")
                .layer,
            ground
        );
        map.layer_mut(roof)
            .expect("registered roof")
            .set_visible(true);
        println!("picked {selected:?} through camera in {projection:?}");
    }
    let chunks: Vec<_> = map
        .layer(ground)
        .expect("registered ground")
        .chunks()
        .collect();
    assert_eq!(chunks.len(), 3);
    map.set_tile(ground, GridCell::new(-17, -1), None)?;
    assert_eq!(
        map.layer(ground)
            .expect("registered ground")
            .chunks()
            .count(),
        2
    );
    println!("ordered occupied chunks: {chunks:?}; empty chunk released");
    Ok(())
}
