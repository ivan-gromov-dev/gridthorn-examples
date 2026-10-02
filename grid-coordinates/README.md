# Grid coordinates

Provisional square and diamond-isometric projection through the opt-in public
`gridthorn::grid` API. Runs without creating a window or GPU resources.

```console
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_grid_coordinates
```

The example projects cell centers and checks inverse conversion for positive
and negative coordinates. Projection uses positive Y downward; a renderer or
camera adapter must convert those presentation units to its own convention.
