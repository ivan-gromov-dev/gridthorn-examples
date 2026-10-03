# Desktop input monitor

Provisional Milestone 4 public SDK example. The native window requires no GPU:
it prints ordered keyboard, modifier, focus, pointer and wheel events.
F1 confines, F2 locks, F3 releases the pointer; Escape exits.
Capture feedback distinguishes requested/effective modes and platform errors.

```console
cargo run -p gridthorn_example_desktop_input
cargo run -p gridthorn_example_desktop_input -- --headless
cargo run -p gridthorn_example_desktop_input -- --smoke
cargo run -p gridthorn_example_desktop_input -- --gpu --smoke
cargo run -p gridthorn_example_desktop_input -- --gpu --smoke-immediate
```

The headless mode validates ordered physical/logical key events, Cyrillic shortcut
identity, repeat, a short tap, wheel units and focus cancellation. It is not an
IME or committed-text example. The native smoke requests confinement, locking
and release, waits for their feedback, then exits. It needs a focused desktop
window; denied modes print an error rather than selecting a fallback.

See [the input contract](../../gridthorn-engine/docs/INPUT.md) for platform evidence
and explicit deferrals. Native gamepad support remains planned for Milestone 5.

Use --gpu to initialize the renderer. The immediate GPU smoke exits at the first
frame boundary before presentation and guards the early swapchain teardown regression.
