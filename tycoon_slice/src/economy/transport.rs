use super::model::{Harbor, Kind, MAX, MIN, Resource, Role, Worker};
use super::navigation::neighbors;
use gridthorn::RandomStreams;
use gridthorn::grid::{GridCell, NavigationBounds, PathStatus, search_path};

impl Harbor {
    /// Advance individual work and transport on authoritative fixed ticks.
    pub fn step(&mut self, tick: u64, random: &mut RandomStreams) {
        self.advance_workers(tick, random);
        self.timber = self.raw_total();
    }
    pub fn raw_total(&self) -> u64 {
        self.buildings.values().map(|b| b.raw).sum::<u64>()
            + u64::try_from(
                self.workers
                    .values()
                    .filter(|w| w.cargo == Some(Resource::Raw))
                    .count(),
            )
            .expect("bounded workers")
    }

    pub fn advance_workers(&mut self, tick: u64, random: &mut RandomStreams) {
        let ids: Vec<_> = self.workers.keys().copied().collect();
        for id in ids {
            let mut worker = self.workers.remove(&id).expect("worker");
            self.work_tick(id, &mut worker, tick, random);
            self.workers.insert(id, worker);
        }
    }

    fn work_tick(&mut self, id: u64, worker: &mut Worker, tick: u64, random: &mut RandomStreams) {
        if worker.remaining > 0 {
            worker.remaining -= 1;
            if worker.remaining == 0 {
                worker.cargo = Some(if worker.role == Role::Carpenter {
                    Resource::Planks
                } else {
                    Resource::Raw
                });
                worker.outbound = true;
            }
            return;
        }
        let target = if worker.outbound {
            worker.destination
        } else {
            worker.source
        };
        let Some(building) = self.buildings.get(&target) else {
            return;
        };
        let adjacent = neighbors(building.cell).contains(&worker.cell);
        if !adjacent {
            if tick.is_multiple_of(2)
                && let Some(next) = self.next_step(worker.cell, building.cell)
            {
                worker.cell = next;
            }
            return;
        }
        if worker.outbound {
            self.deliver(worker, random);
        } else {
            self.collect(id, worker);
        }
    }

    fn collect(&mut self, id: u64, worker: &mut Worker) {
        if !self.assignment_valid(worker.role, worker.source, worker.destination) {
            return;
        }
        if worker.role == Role::Porter && self.next_porter(id, worker) != id {
            return;
        }
        let source = self.buildings.get_mut(&worker.source).expect("source");
        match worker.role {
            Role::Lumberjack => {
                worker.remaining = 30;
            }
            Role::Carpenter if source.raw > 0 => {
                source.raw -= 1;
                worker.cargo = Some(Resource::Raw);
                worker.remaining = 40;
            }
            Role::Porter => {
                let (stock, cargo) = if source.kind == Kind::RawStore {
                    (&mut source.raw, Resource::Raw)
                } else {
                    (&mut source.planks, Resource::Planks)
                };
                if *stock > 0 {
                    *stock -= 1;
                    source.last_porter = id;
                    worker.cargo = Some(cargo);
                    worker.outbound = true;
                }
            }
            Role::Carpenter => {}
        }
    }

    fn deliver(&mut self, worker: &mut Worker, random: &mut RandomStreams) {
        let Some(cargo) = worker.cargo else {
            return;
        };
        let destination = self
            .buildings
            .get_mut(&worker.destination)
            .expect("destination");
        match (destination.kind, cargo) {
            (Kind::RawStore | Kind::Sawmill, Resource::Raw) if destination.raw < 9999 => {
                destination.raw += 1;
            }
            (Kind::PlankStore, Resource::Planks) if destination.planks < 9999 => {
                destination.planks += 1;
            }
            (Kind::Port, resource) if destination.export == resource => {
                let value = loop {
                    let value = random.next_u64("market").expect("market stream");
                    if value < u64::MAX - 2 {
                        break value % 3;
                    }
                };
                self.coins = (self.coins
                    + if resource == Resource::Raw {
                        6 + value
                    } else {
                        14 + value
                    })
                .min(999_999);
                self.shipped += 1;
            }
            _ => return,
        }
        worker.cargo = None;
        worker.outbound = false;
    }

    fn next_step(&self, start: GridCell, target: GridCell) -> Option<GridCell> {
        let bounds = NavigationBounds::new(GridCell::new(MIN, MIN), GridCell::new(MAX, MAX))
            .expect("bounds");
        neighbors(target)
            .into_iter()
            .filter(|c| self.roads.contains(c))
            .filter_map(|goal| {
                let result = search_path(bounds, start, goal, 144, |cell| {
                    self.roads.contains(&cell).then_some(1)
                })
                .ok()?;
                (result.status == PathStatus::Found).then_some(result.path)
            })
            .min_by_key(Vec::len)
            .and_then(|path| path.get(1).copied())
    }
}

impl Harbor {
    fn next_porter(&self, current: u64, worker: &Worker) -> u64 {
        let last = self.buildings[&worker.source].last_porter;
        let source_cell = self.buildings[&worker.source].cell;
        let mut candidates: Vec<_> = self
            .workers
            .iter()
            .filter(|(_, other)| {
                other.role == Role::Porter
                    && other.source == worker.source
                    && !other.outbound
                    && other.cargo.is_none()
                    && neighbors(source_cell).contains(&other.cell)
                    && self.assignment_valid(other.role, other.source, other.destination)
            })
            .map(|(id, _)| *id)
            .collect();
        candidates.push(current);
        candidates.sort_unstable();
        candidates
            .iter()
            .copied()
            .find(|id| *id > last)
            .unwrap_or(candidates[0])
    }
}
