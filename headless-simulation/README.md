# Provisional headless simulation

Run from the examples workspace:

```console
cargo run -p gridthorn_example_headless_simulation
```

Uses the public SDK `HeadlessSimulation` to execute 100 exact fixed ticks twice,
with different request partitions. The example consumes a one-shot command and
compares integer authoritative state. Presentation schedules fail if invoked.
No window, GPU, or audio device is initialized. See the engine's
[simulation contract](../../gridthorn-engine/docs/SIMULATION.md) for lifecycle,
exit, explicit stepping, dependency, and determinism limits.
