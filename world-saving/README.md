# Provisional world saving and loading

Run `cargo run -p gridthorn_example_world_saving` from the examples workspace.
No window or GPU is initialized. The example writes a schema 1 save into a
temporary directory, replaces it at tick 25 with a pending command, loads into
a fresh runtime, and compares full canonical documents after tick 100.
It also checks unsupported-schema rollback and removes its temporary files.

The game codec validates a versioned integer payload. Real games should provide
their own nested data encoding, durable object identities, invariant validation,
and transient reconstruction. All authoritative state belongs to ScenarioState;
arbitrary ECS entities and external resources are outside this save contract.
