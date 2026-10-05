# Gridthorn Examples

Runnable examples for the sibling `gridthorn-engine` repository.

Run the [game settings menu and desktop displays](desktop-displays/README.md):

```console
cargo run -p gridthorn_example_desktop_displays --locked
```

Run the integrated [multilingual workbench](multilingual-workbench/README.md),
with four languages, Unicode editing, localization, six controls and nested windows:

```console
cargo run -p gridthorn_example_multilingual_workbench --locked
cargo run -p gridthorn_example_multilingual_workbench --locked -- --headless
cargo run -p gridthorn_example_multilingual_workbench --locked -- --smoke
```

Run the provisional [composed controls example](composed-controls/README.md):

```console
cargo run -p gridthorn_example_composed_controls --locked
cargo run -p gridthorn_example_composed_controls --locked -- --headless
```

Run the provisional [localization runtime example](localization/README.md):

```console
cargo run -p gridthorn_example_localization --locked
```

Run the provisional [multilingual font example](multilingual-text/README.md):

```console
cargo run -p gridthorn_example_multilingual_text
cargo run -p gridthorn_example_multilingual_text -- --headless
```

Run the provisional [Unicode/IME input example](text-input/README.md):

```console
cargo run -p gridthorn_example_text_input
cargo run -p gridthorn_example_text_input -- --headless
```

Run the provisional [desktop input monitor](desktop-input/README.md), with no GPU initialization:

```console
cargo run -p gridthorn_example_desktop_input
cargo run -p gridthorn_example_desktop_input -- --headless
```

Play [Timber Harbor](tycoon_slice/README.md), the integrated Milestone 3 tycoon
showcase with original workforce/UI assets, housed employees, resource-specific
ports, a pause/save/load menu and diagnostics:

```console
cargo run -p tycoon_slice
cargo run -p tycoon_slice -- --headless-smoke
```

Run the provisional [world save/load example](world-saving/README.md):

```console
cargo run -p gridthorn_example_world_saving
```

Run the provisional [scenario/snapshot example](scenarios-snapshots/README.md):

```console
cargo run -p gridthorn_example_scenarios_snapshots
```

Run the provisional [headless simulation example](headless-simulation/README.md):

```console
cargo run -p gridthorn_example_headless_simulation
```

Run the provisional [object placement example](grid-placement/README.md):

```console
cargo run -p gridthorn_example_grid_placement
```

Run the provisional [tilemap example](tilemap-basics/README.md) without a window:

```console
cargo run -p gridthorn_example_tilemap_basics
```

Run the provisional [grid coordinate example](grid-coordinates/README.md)
without a window:

```console
cargo run -p gridthorn_example_grid_coordinates
```

Play [Crystal Trail](classic_2d/README.md), the basic 2D integration game:

```console
cargo run -p classic_2d
cargo run -p classic_2d -- --headless-smoke
```

Every example owns a top-level directory and Cargo package:

```text
gridthorn-examples/
├── asset-reload/
├── collision-basics/
├── deterministic-replay/
├── diagnostics-flow/
├── headless-schedule/
├── schedule-loop/
└── window-surface/
    ├── Cargo.toml
    └── src/
```

Run the current window and GPU surface lifecycle example:

```console
cargo run -p gridthorn_example_window_surface
```

The examples currently use local path dependencies and expect
`gridthorn-engine` and `gridthorn-examples` to be sibling directories.

Run the ECS lifecycle schedule example:

```console
cargo run -p gridthorn_example_schedule_loop
```

Run texture hot reload with dependency diagnostics, or its headless file-edit smoke:

```console
cargo run -p gridthorn_example_asset_reload
cargo run -p gridthorn_example_asset_reload -- --smoke
```

See [asset-reload](asset-reload/README.md) for editable fixtures and limitations.

Use `-- --smoke` to run three updates and close the window automatically.

Run the same fixed-update schedule boundary without an application or renderer:

```console
cargo run -p gridthorn_example_headless_schedule -- --ticks 10
```

Run structured CLI, application, and renderer diagnostics through the engine
CLI from the sibling engine repository:

```console
cargo run -p gridthorn_cli -- run ../gridthorn-examples/diagnostics-flow
```

Run the same fixed-step command stream twice and verify its state fingerprint:

```console
cargo run -p gridthorn_example_deterministic_replay
```

Run engine-owned circle and axis-aligned box collision queries without a
window, renderer, or physics backend:

```console
cargo run -p gridthorn_example_collision_basics
```

Run the read-only component/resource reflection example:

cargo run -p gridthorn_example_reflection_basics

Run the headless versioned scene round-trip, rejected-edit, and migration example:

```console
cargo run -p gridthorn_example_scene_serialization
```

Provisional weighted navigation and square/isometric SVG diagnostics:
`cargo run -p gridthorn_example_pathfinding`.
See [pathfinding](pathfinding/README.md).

Run [simulation clock controls](simulation-clock/README.md):

```console
cargo run -p gridthorn_example_simulation_clock
```
