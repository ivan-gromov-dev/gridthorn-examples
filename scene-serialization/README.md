# Scene serialization

Run `cargo run -p gridthorn_example_scene_serialization` from this workspace.

The headless example persists a scene-owned player and a global score resource
through the public SDK. It prints the schema 1 TOML, reconstructs an omitted
presentation cache, rejects invalid health without changing the world, and
applies an explicit pure legacy field migration before loading.

The synthetic schema 0 input exists only to exercise migration hooks; no earlier
engine scene format is claimed. This example performs no filesystem writes.
Scene commit is an explicit load/reset operation, separate from game-state or
active-scene transitions. See the engine's `docs/SCENES.md` for scope and limits.
