use super::model::{Harbor, MAX, MIN};
use gridthorn::grid::{GridCell, NavigationBounds, PathSearch, PathStatus, search_path};
impl Harbor {
    pub fn route(&self, start: GridCell) -> Option<Vec<GridCell>> {
        let result = self.diagnostic(start, 144)?;
        (result.status == PathStatus::Found).then_some(result.path)
    }

    pub fn diagnostic(&self, start: GridCell, budget: usize) -> Option<PathSearch> {
        if !self.roads.contains(&start) {
            return None;
        }
        let bounds = NavigationBounds::new(GridCell::new(MIN, MIN), GridCell::new(MAX, MAX))
            .expect("bounds");
        let result = search_path(bounds, start, GridCell::new(4, 0), budget, |cell| {
            self.roads.contains(&cell).then_some(1)
        })
        .expect("route query");
        Some(result)
    }

    pub fn building_route(&self, cell: GridCell) -> Option<Vec<GridCell>> {
        neighbors(cell)
            .into_iter()
            .filter_map(|start| self.route(start))
            .min_by_key(Vec::len)
    }
}
pub fn neighbors(cell: GridCell) -> [GridCell; 4] {
    [
        GridCell::new(cell.column - 1, cell.row),
        GridCell::new(cell.column + 1, cell.row),
        GridCell::new(cell.column, cell.row - 1),
        GridCell::new(cell.column, cell.row + 1),
    ]
}
