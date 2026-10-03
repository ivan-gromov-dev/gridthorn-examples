use super::sprite;
use crate::economy::model::{DOCK, GROUND, MAX, MIN};
use crate::game::session::{Session, Tool};
use gridthorn::grid::{GridCell, GridObjectId};
use gridthorn::{Camera2d, Color, RenderFrame, Sprite, SpriteRegion, TextureAsset, TexturedSprite};

#[allow(clippy::cast_possible_truncation)]
fn point(session: &Session, cell: GridCell) -> [f32; 2] {
    let p = session
        .projection()
        .cell_center(cell)
        .expect("finite map cell");
    [p.x as f32, p.y as f32]
}

pub fn extract(
    session: &Session,
    texture: &TextureAsset,
    expansion: &TextureAsset,
    worker_frame: SpriteRegion,
) -> RenderFrame {
    let mut ground = Vec::new();
    let mut sprites = Vec::new();
    let mut cells: Vec<_> = session
        .data
        .terrain
        .layer(GROUND)
        .expect("ground")
        .tiles()
        .map(|(cell, _)| cell)
        .collect();
    cells.sort_by_key(|c| (c.column + c.row, c.column));
    for cell in cells {
        let p = point(session, cell);
        let tile = session
            .data
            .terrain
            .tile(GROUND, cell)
            .copied()
            .expect("tile");
        if session.square {
            ground.push(Sprite::new(
                p,
                [45.0, 45.0],
                if tile == 2 {
                    Color::rgb(0.08, 0.36, 0.43)
                } else if session.data.roads.contains(&cell) {
                    Color::rgb(0.55, 0.55, 0.47)
                } else {
                    Color::rgb(0.23, 0.4, 0.28)
                },
            ));
        } else {
            sprites.push(sprite(
                texture,
                if session.data.roads.contains(&cell) {
                    1
                } else {
                    tile
                },
                p,
                [67.0, 44.0],
            ));
        }
    }
    let mut objects = buildings_and_scenery(session, texture, expansion);
    if session.paths {
        for building in session.data.buildings.values() {
            if let Some(path) = session.data.building_route(building.cell) {
                for cell in path {
                    let p = point(session, cell);
                    sprites.push(
                        sprite(texture, 14, p, [32.0, 22.0]).with_tint(Color::rgb(0.6, 0.85, 1.0)),
                    );
                }
            }
        }
        if let Some(search) = session.diagnostic() {
            for (cell, _) in search.visited {
                sprites.push(
                    sprite(texture, 14, point(session, cell), [18.0, 12.0])
                        .with_tint(Color::rgb(0.35, 0.7, 1.0)),
                );
            }
            for (cell, _) in search.frontier {
                sprites.push(sprite(texture, 15, point(session, cell), [20.0, 13.0]));
            }
        }
    }
    add_workers(session, texture, expansion, worker_frame, &mut objects);
    objects.sort_by_key(|(depth, _)| *depth);
    sprites.extend(objects.into_iter().map(|(_, sprite)| sprite));
    overlays(session, texture, expansion, &mut sprites);
    RenderFrame::new(Camera2d::new(session.camera, session.height), ground)
        .with_textured_sprites(sprites)
}

fn buildings_and_scenery(
    session: &Session,
    texture: &TextureAsset,
    expansion: &TextureAsset,
) -> Vec<(i32, TexturedSprite)> {
    let mut objects: Vec<(i32, TexturedSprite)> = Vec::new();
    for building in session.data.buildings.values() {
        let p = point(session, building.cell);
        let tint = if session.data.building_route(building.cell).is_some() {
            Color::rgb(1.0, 1.0, 1.0)
        } else {
            Color::rgb(0.7, 0.55, 0.5)
        };
        objects.push((
            building.cell.column + building.cell.row,
            building_sprite(texture, expansion, building.kind, [p[0], p[1] - 19.0]).with_tint(tint),
        ));
    }
    let boat = point(session, GridCell::new(6, -3));
    objects.push((
        3,
        sprite(texture, 9, [boat[0], boat[1] - 18.0], [80.0, 83.0]),
    ));
    objects
}

fn overlays(
    session: &Session,
    texture: &TextureAsset,
    expansion: &TextureAsset,
    sprites: &mut Vec<TexturedSprite>,
) {
    if let Some(cell) = session.hover {
        let p = point(session, cell);
        let valid = match session.tool {
            Tool::Select => true,
            Tool::Build(kind) => {
                session.data.coins >= kind.cost()
                    && session
                        .data
                        .can_build_kind(kind, cell, GridObjectId(session.data.next_id))
            }
            Tool::Road => {
                session.data.coins >= 3
                    && session.data.land(cell)
                    && session.data.occupancy.object_at(cell).is_none()
                    && !session.data.roads.contains(&cell)
            }
            Tool::Move => session.data.can_build(
                cell,
                session.moving.unwrap_or(GridObjectId(session.data.next_id)),
            ),
            Tool::Remove => {
                session.data.occupancy.object_at(cell).is_some()
                    || cell != DOCK && session.data.roads.contains(&cell)
            }
        };
        sprites.push(sprite(
            texture,
            if valid { 14 } else { 15 },
            p,
            [66.0, 45.0],
        ));
        if let Tool::Build(kind) = session.tool {
            sprites.push(
                building_sprite(texture, expansion, kind, [p[0], p[1] - 19.0])
                    .with_tint(Color::rgba(1.0, 1.0, 1.0, 0.45)),
            );
        }
    }
    if session.chunks {
        for column in MIN..=MAX {
            for row in MIN..=MAX {
                if column.rem_euclid(4) == 0 || row.rem_euclid(4) == 0 {
                    let p = point(session, GridCell::new(column, row));
                    sprites.push(sprite(texture, 14, p, [15.0, 10.0]));
                }
            }
        }
    }
}

fn add_workers(
    session: &Session,
    texture: &TextureAsset,
    expansion: &TextureAsset,
    worker_frame: SpriteRegion,
    objects: &mut Vec<(i32, TexturedSprite)>,
) {
    for worker in session.data.workers.values() {
        let cell = worker.cell;
        let p = point(session, cell);
        let phase = usize::from(worker_frame == super::region(11));
        let regions = match worker.role {
            crate::economy::model::Role::Lumberjack => {
                [[1090, 160, 255, 338], [1485, 160, 260, 338]]
            }
            crate::economy::model::Role::Carpenter => [[125, 526, 252, 345], [617, 526, 258, 345]],
            crate::economy::model::Role::Porter => [[1094, 519, 233, 355], [1489, 519, 235, 355]],
        };
        objects.push((
            cell.column + cell.row,
            TexturedSprite::new([p[0], p[1] - 16.0], [25.0, 38.0], expansion.clone())
                .with_region(super::skin::region(regions[phase], expansion)),
        ));
        if let Some(cargo) = worker.cargo {
            objects.push((
                cell.column + cell.row,
                sprite(
                    texture,
                    if cargo == crate::economy::model::Resource::Raw {
                        12
                    } else {
                        13
                    },
                    [p[0] + 10.0, p[1] - 8.0],
                    [13.0, 10.0],
                ),
            ));
        }
    }
}

fn building_sprite(
    texture: &TextureAsset,
    expansion: &TextureAsset,
    kind: crate::economy::model::Kind,
    position: [f32; 2],
) -> TexturedSprite {
    use crate::economy::model::Kind;
    if matches!(kind, Kind::RawStore | Kind::PlankStore) {
        TexturedSprite::new(position, [67.0, 72.0], expansion.clone()).with_region(
            super::skin::region(
                if kind == Kind::RawStore {
                    [0, 0, 484, 526]
                } else {
                    [484, 0, 500, 526]
                },
                expansion,
            ),
        )
    } else {
        sprite(texture, kind.sprite(), position, [67.0, 72.0])
    }
}
