# Unicode and IME input

Provisional Milestone 4 SDK example. The native window has no GPU renderer;
text and composition are printed to the terminal.

```console
cargo run -p gridthorn_example_text_input -- --headless
cargo run -p gridthorn_example_text_input
cargo run -p gridthorn_example_text_input -- --smoke
```

F4 opens/updates the text session, F5 closes it. Control/Super+C copies the
game-owned document; Control/Super+V appends clipboard text. Escape exits when
there is no preedit. Focus return explicitly reopens the example's text session.
Use a Cyrillic layout, a dead-key layout and an installed Japanese/Chinese IME
to inspect commits, byte cursor offsets, preedit replacement and cancellation.
IME candidates are anchored near the top-left of the window.

Headless mode validates Cyrillic, combining marks, emoji sequences, Japanese
preedit/commit and focus cancellation. Native smoke validates session activation
and Unicode clipboard round-trip, restoring the original text. It skips writes
when original clipboard text is unavailable and fails after ten seconds without
window focus. The smoke does not type through a native input method.

Text editing controls, Unicode rendering, shaping and UI input consumption are
later roadmap items. See the engine's [input contract](../../gridthorn-engine/docs/INPUT.md).
