use super::model::{Harbor, Kind, Resource, Role, Worker};
use super::navigation::neighbors;
use gridthorn::grid::{GridCell, GridObjectId};

impl Harbor {
    pub fn in_use(&self, cell: GridCell) -> bool {
        self.workers.values().any(|w| {
            w.cell == cell
                || [w.home, w.source, w.destination]
                    .into_iter()
                    .any(|id| self.buildings.get(&id).is_some_and(|b| b.cell == cell))
        })
    }

    pub fn assignment_valid(
        &self,
        role: Role,
        source: GridObjectId,
        destination: GridObjectId,
    ) -> bool {
        let (Some(a), Some(b)) = (
            self.buildings.get(&source),
            self.buildings.get(&destination),
        ) else {
            return false;
        };
        match role {
            Role::Lumberjack => a.kind == Kind::Forest && b.kind == Kind::RawStore,
            Role::Carpenter => a.kind == Kind::Sawmill && b.kind == Kind::PlankStore,
            Role::Porter => {
                matches!(a.kind, Kind::RawStore | Kind::PlankStore)
                    && ((a.kind == Kind::RawStore && b.kind == Kind::Sawmill)
                        || (b.kind == Kind::Port
                            && b.export
                                == if a.kind == Kind::RawStore {
                                    Resource::Raw
                                } else {
                                    Resource::Planks
                                }))
            }
        }
    }

    pub fn hire(&mut self, role: Role, source: GridObjectId) {
        let home = self
            .buildings
            .iter()
            .find(|(id, b)| {
                b.kind == Kind::Home && self.workers.values().filter(|w| w.home == **id).count() < 4
            })
            .map(|(id, _)| *id);
        let destination = self
            .buildings
            .keys()
            .copied()
            .find(|id| self.assignment_valid(role, source, *id));
        let cell = self.buildings.get(&source).and_then(|b| {
            neighbors(b.cell)
                .into_iter()
                .find(|c| self.roads.contains(c))
        });
        let occupied = role == Role::Lumberjack
            && self
                .workers
                .values()
                .any(|w| w.role == role && w.source == source);
        if self.coins < role.cost() || occupied || self.workers.len() >= 64 {
            self.message = "HIRE REJECTED: GOLD OR WORKPLACE UNAVAILABLE".into();
            return;
        }
        let (Some(home), Some(destination), Some(cell)) = (home, destination, cell) else {
            self.message = "HIRE REJECTED: NEED HOUSE SPACE, ROAD AND DESTINATION".into();
            return;
        };
        self.coins -= role.cost();
        self.workers.insert(
            self.next_worker,
            Worker {
                role,
                home,
                source,
                destination,
                cell,
                cargo: None,
                remaining: 0,
                outbound: false,
            },
        );
        self.next_worker += 1;
        self.message = format!("{} HIRED - HOUSE #{}", role.name(), home.0);
    }

    pub fn assign(&mut self, id: u64, source: GridObjectId, destination: GridObjectId) {
        let Some(worker) = self.workers.get(&id) else {
            return;
        };
        let occupied = worker.role == Role::Lumberjack
            && self
                .workers
                .iter()
                .any(|(other, w)| *other != id && w.role == Role::Lumberjack && w.source == source);
        if !self.assignment_valid(worker.role, source, destination)
            || occupied
            || worker.cargo.is_some()
            || worker.remaining > 0
        {
            self.message = "ASSIGN REJECTED: INCOMPATIBLE OR WORKER BUSY".into();
            return;
        }
        let worker = self.workers.get_mut(&id).expect("worker");
        worker.source = source;
        worker.destination = destination;
        worker.outbound = false;
        self.message = "WORKER ASSIGNED".into();
    }

    pub fn set_export(&mut self, id: GridObjectId, resource: Resource) {
        let Some(port) = self.buildings.get(&id) else {
            return;
        };
        if port.kind != Kind::Port {
            return;
        }
        if self
            .workers
            .values()
            .any(|w| w.destination == id && w.cargo.is_some_and(|cargo| cargo != resource))
        {
            self.message = "PORT BUSY: WAIT FOR EXISTING CARGO".into();
            return;
        }
        self.buildings.get_mut(&id).expect("port").export = resource;
        self.message = format!(
            "PORT NOW SELLS ONLY {} - CHECK PORTER ASSIGNMENTS",
            resource.name()
        );
    }
}
