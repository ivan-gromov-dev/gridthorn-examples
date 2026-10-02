use gridthorn::grid::{GridCell, GridProjection, PathSearch, PlacementMap};
use std::fmt::Write;

pub fn write(
    path: &std::path::Path,
    projection: GridProjection,
    search: &PathSearch,
    occupancy: &PlacementMap,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut svg = String::from(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"560\" height=\"400\" viewBox=\"0 0 560 400\"><rect width=\"560\" height=\"400\" fill=\"#101827\"/>",
    );
    for column in -3..=3 {
        for row in -3..=3 {
            let cell = GridCell::new(column, row);
            let fill = if occupancy.object_at(cell).is_some() {
                "#ef6666"
            } else if search.path.contains(&cell) {
                "#65de9c"
            } else if search.visited.iter().any(|(visited, _)| *visited == cell) {
                "#4478a8"
            } else if search
                .frontier
                .iter()
                .any(|(frontier, _)| *frontier == cell)
            {
                "#eac45e"
            } else {
                "#273449"
            };
            let mut points = String::new();
            for (dc, dr) in [(0, 0), (1, 0), (1, 1), (0, 1)] {
                let point = projection.cell_vertex(GridCell::new(column + dc, row + dr))?;
                write!(points, "{},{} ", point.x, point.y)?;
            }
            write!(
                svg,
                "<polygon points=\"{points}\" fill=\"{fill}\" stroke=\"#101827\"/>"
            )?;
        }
    }
    write!(
        svg,
        "<text x=\"16\" y=\"24\" fill=\"white\" font-family=\"monospace\">{:?} | cost {:?} | visited {}</text><text x=\"16\" y=\"378\" fill=\"white\" font-family=\"monospace\">Red: blocked | Blue: visited | Yellow: frontier | Green: path</text></svg>",
        search.status,
        search.cost,
        search.visited.len()
    )?;
    std::fs::write(path, svg)?;
    Ok(())
}
