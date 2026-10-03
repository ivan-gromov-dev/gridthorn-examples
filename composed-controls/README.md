# Composed runtime controls

Provisional public `gridthorn::ui` example: multilingual label, button, toggle,
slider, scrolling list and editable Unicode text field. Nested layout/clips use
logical pixels and follow window DPI/resize.

```console
cargo run -p gridthorn_example_composed_controls --locked
cargo run -p gridthorn_example_composed_controls --locked -- --headless
cargo run -p gridthorn_example_composed_controls --locked -- --smoke
cargo run -p gridthorn_example_composed_controls --locked -- --layers
cargo run -p gridthorn_example_composed_controls --locked -- --layers --headless
cargo run -p gridthorn_example_composed_controls --locked -- --layers --smoke
cargo run -p gridthorn_example_composed_controls --locked -- --animations
cargo run -p gridthorn_example_composed_controls --locked -- --animations --headless
cargo run -p gridthorn_example_composed_controls --locked -- --animations --smoke
cargo run -p gridthorn_example_composed_controls --locked -- --layers --animations --smoke
cargo test -p gridthorn_example_composed_controls --locked
```

Click to focus/activate, drag the slider or field selection, and wheel-scroll.
Tab/Shift+Tab changes focus; Enter/Space activates; arrows navigate controls,
adjust slider/list values or move the text caret. Shift extends text selection;
Control/Command+A/C/X/V selects all, copies, cuts and pastes. Type Unicode text
or use an installed IME in the field. Escape cancels preedit, then clears focus;
a subsequent Escape outside UI focus exits.

The example routes during `Input`, forwards explicit text-session/clipboard
requests and prepares caret/selection/preedit through `UiRouter::layout`. World
Escape mapping uses only `UiRoute::world_events`; games with continuous movement
must also obey `keyboard_blocked` and `pointer_blocked` before mapping held state.
Controller adapters can call the same device-independent `UiNavigation` hooks.
Native controller discovery remains a later engine milestone.

Headless checks cover control effects, clamped values, list/scroll bounds,
responsive layout at 320/900 logical pixels and DPI 1/1.25/2, ordered clicks,
world consumption, navigation hooks, multilingual selection and IME cancellation.
Native smoke presents 120 frames then exits. It proves window/render lifecycle;
interactive keyboard layouts, native IME candidate positioning and clipboard
shortcuts still require manual device acceptance. No clipboard writes occur in
headless or smoke mode without user keyboard interaction.
Font assets reuse licensed Noto fixtures in `../multilingual-text/assets/fonts`.

`--layers` opens a modal dialog and a context popup above it. Escape closes the
top layer and restores focus; an outside pointer press closes the popup without
activating controls beneath it. The remaining dialog blocks background input.
The headless variant checks ordering, modal blocking and nested focus restoration
at DPI 2; the smoke variant renders 120 native frames. These checks do not prove
interactive device acceptance or Linux/macOS behavior.

`--animations` slides the Apply control into place and fades its background;
combined with `--layers`, it animates the dialog panel. Background alpha affects
only its surface, not the child text or controls. Transitions advance during
`Update` with unscaled `FrameTiming::frame_elapsed()`, then refresh layout and
text-session anchors. Input/modal ownership remains the router's responsibility.
The headless workflow checks both trees at DPI 1/2, interruption and local pause;
the engine facade test additionally checks simulation pause/speed independence.
Both control and layered animation smoke modes completed 120 native frames on
Windows on 2026-10-03; visual/interactive and Linux/macOS acceptance remain open.
