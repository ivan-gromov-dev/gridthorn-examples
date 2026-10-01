# Gridthorn Examples

Runnable examples for the sibling `gridthorn-engine` repository.

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
