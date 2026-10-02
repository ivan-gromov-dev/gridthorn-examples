# Provisional scenarios and snapshots

```console
cargo run -p gridthorn_example_scenarios_snapshots
```

The public SDK example defines a typed integer economy scenario with explicit
master seed and separate economy/weather streams. It compares full state after
100 ticks, restores a checkpoint into a fresh runner, and repeats from the initial
snapshot with the same command injection boundary. It prints a game-encoded
state fingerprint; snapshot capture itself does not infer a canonical encoding.

See the engine [scenario contract](../../gridthorn-engine/docs/SCENARIOS.md).
Snapshots are in memory and cover the declared authoritative root, commands,
RNG, controls, exit, and tick clock. Arbitrary ECS capture and user-save files
are separate work.
# CLI headless launch

This example also declares `economy` in `gridthorn.toml` and provides the separate
`scenario-headless` Cargo binary. From the sibling engine checkout:

```console
cargo run -p gridthorn_cli -- scenario list ../gridthorn-examples/scenarios-snapshots
cargo run -p gridthorn_cli -- simulate ../gridthorn-examples/scenarios-snapshots --scenario economy --ticks 100 --seed 42
```

Repeat the second command to compare the game-owned authoritative fingerprint.
`--ticks 0` initializes without fixed updates; `--release` selects Cargo's release
profile. The binary constructs `ScenarioRuntime`, consumes the explicit seed,
executes requested ticks, prints completed ticks and a fingerprint, and shuts down
without creating a window or device. The ordinary example binary still validates
snapshot continuation. The launch contract remains provisional.

