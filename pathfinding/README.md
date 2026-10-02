# Pathfinding diagnostics

Run `cargo run -p gridthorn_example_pathfinding` from the examples workspace.
The headless example combines weighted tile terrain and placement occupancy,
checks repeatability, and writes found-route and budget-limited square/isometric
SVG diagrams into `target/pathfinding/`. Open these SVGs in a browser.

Red: blocked; blue: settled; yellow: tentative frontier; green: found route.
The public API is provisional and enabled with the `grid` feature.
