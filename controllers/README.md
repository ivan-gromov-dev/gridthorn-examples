# Controller lab

A native visual input monitor built through public Gridthorn APIs. The device list
always contains **Клавиатура и мышь** and adds each controller found by the last explicit request.
Click «Обновить устройства» to refresh discovery. Disconnected controllers disappear on the next request. If the selected controller is unplugged, the
selection returns to keyboard/mouse; identical models have separate connection rows.

Click a device to switch immediately. Keyboard or controller navigation only highlights
a row; activate «Применить устройство» to confirm it. Moving a stick never switches
the active device by itself. For a selected controller, D-pad or left
stick navigates the menu, shoulders cycle focus, South activates and East cancels.
The live panel highlights held buttons, displays both triggers as numeric pressure
and bars, and displays each stick as a moving point with X/Y coordinates. Selecting
keyboard/mouse instead displays held WASD/Space/Enter, mouse buttons and cursor
position. Escape exits. Window resizing and DPI scale the presentation.

The explicit vibration button requests 200 ms on the selected controller. Typed
unsupported, disconnected and native errors appear in the feedback line. It is
disabled for keyboard/mouse. Selecting a controller never automatically vibrates it.

```console
cargo run -p gridthorn_example_controllers --locked
cargo run -p gridthorn_example_controllers --locked -- --headless
cargo run -p gridthorn_example_controllers --locked -- --smoke
```

`--headless` injects a controller and checks UI navigation, frame state, indicators
and disconnection fallback. `--smoke` renders the native window, initializes device
discovery and exits after six frames; zero devices is valid. Physical mappings,
hotplug and rumble still require interactive hardware acceptance. The example uses
the sibling multilingual-text example's bundled Noto Sans font and license.

Connection IDs are ephemeral; model UUIDs do not identify individual units. See
[the engine controller contract](../../gridthorn-engine/docs/CONTROLLERS.md).

The UI explicitly requests discovery at startup and on refresh. When a controller
is selected it requests live state every 50 ms; keyboard/mouse mode does not poll
periodically. The engine performs no automatic per-frame controller polling.
