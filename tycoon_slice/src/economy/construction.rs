use super::model::{Command, DOCK, Harbor, Kind, ROADS};
use gridthorn::grid::{GridFootprint, GridObjectId};
impl Harbor {
    /// Apply game commands atomically at a fixed tick; rejected edits preserve funds.
    pub fn apply(&mut self, command: Command) {
        match command {
            Command::Hire(role, source) => {
                self.hire(role, source);
            }
            Command::Assign(worker, source, destination) => {
                self.assign(worker, source, destination);
            }
            Command::Export(id, resource) => {
                self.set_export(id, resource);
            }

            Command::Build(kind, cell)
                if self.coins >= kind.cost()
                    && self.can_build_kind(kind, cell, GridObjectId(self.next_id)) =>
            {
                self.coins -= kind.cost();
                self.insert_building(kind, cell);
                self.message = format!("{} BUILT - CHECK ROAD CONNECTION", kind.name());
            }
            Command::Road(cell)
                if self.coins >= 3
                    && self.land(cell)
                    && self.occupancy.object_at(cell).is_none()
                    && !self.roads.contains(&cell) =>
            {
                self.coins -= 3;
                self.add_road(cell);
                self.message = "ROAD BUILT".into();
            }
            Command::Remove(cell) if cell != DOCK && !self.in_use(cell) => {
                if let Some(id) = self.occupancy.object_at(cell) {
                    let building = self.buildings.remove(&id).expect("occupancy owner");
                    self.occupancy.remove(id);
                    self.coins += building.kind.cost() / 2;
                    self.message = "BUILDING SALVAGED FOR HALF PRICE".into();
                } else if self.roads.remove(&cell) {
                    self.terrain
                        .set_tile(ROADS, cell, None)
                        .expect("road layer");
                    self.message = "ROAD REMOVED - WORKERS WILL REROUTE".into();
                }
            }
            Command::Move(id, cell)
                if self.coins >= 5
                    && self.can_build(cell, id)
                    && self
                        .buildings
                        .get(&id)
                        .is_some_and(|b| b.kind != Kind::Port || cell.column == 4)
                    && !self.buildings.get(&id).is_some_and(|b| self.in_use(b.cell)) =>
            {
                if self
                    .occupancy
                    .relocate(id, cell, GridFootprint::single_cell())
                    .is_ok()
                {
                    self.buildings.get_mut(&id).expect("building").cell = cell;
                    self.coins -= 5;
                    self.message = "BUILDING MOVED".into();
                }
            }
            _ => self.message = "REJECTED: NEED FREE LAND AND ENOUGH COINS".into(),
        }
    }
}
