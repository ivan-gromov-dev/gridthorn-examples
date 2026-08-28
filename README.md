# Gridthorn Examples

Runnable examples for the sibling `gridthorn-engine` repository.

Every example owns a top-level directory and Cargo package:

```text
gridthorn-examples/
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
