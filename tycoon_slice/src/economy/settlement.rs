use super::model::{Building, DOCK, GROUND, Harbor, Kind, MAX, MIN, ROADS, Resource, Role, Worker};
use gridthorn::grid::{ChunkSize, GridCell, GridFootprint, GridObjectId, PlacementMap, TileMap};
use std::collections::{BTreeMap, BTreeSet};
impl Harbor {
    pub fn new() -> Self {
        let mut terrain = TileMap::new(ChunkSize::new(4, 4).expect("chunk geometry"));
        terrain.add_layer(GROUND).expect("ground layer");
        terrain.add_layer(ROADS).expect("road layer");
        for column in MIN..=MAX {
            for row in MIN..=MAX {
                let tile = if column >= 5 && GridCell::new(column, row) != DOCK {
                    2
                } else if column == 4 {
                    3
                } else {
                    0
                };
                terrain
                    .set_tile(GROUND, GridCell::new(column, row), Some(tile))
                    .expect("ground");
            }
        }
        let mut state = Self {
            coins: 120,
            timber: 4,
            shipped: 0,
            next_id: 1,
            buildings: BTreeMap::new(),
            roads: BTreeSet::new(),
            workers: BTreeMap::new(),
            next_worker: 5,
            terrain,
            occupancy: PlacementMap::new(),
            message: "CONNECT BUILDINGS TO THE DOCK WITH ROADS".into(),
        };
        for column in -4..=4 {
            state.add_road(GridCell::new(column, 0));
        }
        state.insert_building(Kind::Forest, GridCell::new(-4, -1));
        state.insert_building(Kind::RawStore, GridCell::new(-2, -1));
        state.insert_building(Kind::Sawmill, GridCell::new(0, -1));
        state.insert_building(Kind::PlankStore, GridCell::new(2, -1));
        state.insert_building(Kind::Home, GridCell::new(-1, 1));
        state.insert_building(Kind::Port, DOCK);
        state
            .buildings
            .get_mut(&GridObjectId(2))
            .expect("store")
            .raw = 4;
        for (id, role, source, destination, column) in [
            (1, Role::Lumberjack, 1, 2, -4),
            (2, Role::Carpenter, 3, 4, 0),
            (3, Role::Porter, 2, 3, -2),
            (4, Role::Porter, 2, 6, -2),
        ] {
            state.workers.insert(
                id,
                Worker {
                    role,
                    home: GridObjectId(5),
                    source: GridObjectId(source),
                    destination: GridObjectId(destination),
                    cell: GridCell::new(column, 0),
                    cargo: None,
                    remaining: 0,
                    outbound: false,
                },
            );
        }
        state
    }

    pub fn homes(&self) -> usize {
        self.buildings
            .values()
            .filter(|b| b.kind == Kind::Home)
            .count()
    }

    pub fn land(&self, cell: GridCell) -> bool {
        self.terrain.tile(GROUND, cell).is_some_and(|t| *t != 2) && cell != DOCK
    }

    pub fn can_build(&self, cell: GridCell, id: GridObjectId) -> bool {
        self.land(cell)
            && !self.roads.contains(&cell)
            && self
                .occupancy
                .validate(id, cell, &GridFootprint::single_cell())
                .is_ok()
    }

    pub fn can_build_kind(&self, kind: Kind, cell: GridCell, id: GridObjectId) -> bool {
        self.can_build(cell, id) && (kind != Kind::Port || cell.column == 4)
    }

    pub fn add_road(&mut self, cell: GridCell) {
        self.roads.insert(cell);
        self.terrain
            .set_tile(ROADS, cell, Some(1))
            .expect("road layer");
    }

    pub fn insert_building(&mut self, kind: Kind, cell: GridCell) {
        let id = GridObjectId(self.next_id);
        self.occupancy
            .place(id, cell, GridFootprint::single_cell())
            .expect("validated building");
        self.buildings.insert(
            id,
            Building {
                kind,
                cell,
                raw: 0,
                planks: 0,
                export: Resource::Raw,
                last_porter: 0,
            },
        );
        self.next_id += 1;
    }
}
