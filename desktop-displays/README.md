# Game settings / desktop displays

Run from the sibling examples repository:

```console
cargo run -p gridthorn_example_desktop_displays --locked
cargo run -p gridthorn_example_desktop_displays --locked -- --smoke
cargo run -p gridthorn_example_desktop_displays --locked -- --headless
cargo run -p gridthorn_example_desktop_displays --locked -- --presentation-smoke --auto-adapter
cargo run -p gridthorn_example_desktop_displays --locked -- --list-adapters
cargo run -p gridthorn_example_desktop_displays --locked -- --adapter 0 --render-api dx12
cargo run -p gridthorn_example_desktop_displays --locked -- --adapter 0 --render-api dx12 --adapter-smoke
cargo run -p gridthorn_example_desktop_displays --locked -- --auto-adapter
```

The GPU-rendered Russian menu uses the public retained UI and window SDK. Noto
Sans comes from the sibling multilingual-text assets. Graphics is implemented;
Audio/Controls remain navigable future sections.

## Testing manually

1. Select a monitor. Click the mode/size buttons to cycle staged choices; toggle
   user resizing. Nothing is applied until **Применить**.
2. Apply windowed size/policy. Try resizing by dragging the window border. The
   resizable minimum is 640 × 480 physical pixels. Check the actual state below
   the inventory; scroll the content if needed.
3. Choose **Без рамки**, apply, then return to **Оконный** and apply. Rendering
   stays in the existing window. Borderless uses desktop resolution/refresh;
   windowed sizing does not change the desktop video mode.
4. Where `exclusive` capability is available, choose **Эксклюзивный**, resolution
   and frequency. Frequency cycles exact advertised modes at that resolution.
   Windows supports all three modes. **Фактически** shows the OS-reported refresh,
   which can be 59 Hz for a nominal 60 Hz selection. Rejected mode switches are
   currently subject to the upstream rejection/recovery limitation described below.
5. **Сбросить выбор** restores staged settings from actual observed state without
   another native operation. **Обновить список** queries once; returning from
   another tab queries once. Change desktop resolution/scaling or disconnect a
   second display, then refresh. IDs survive only their documented connection.
6. Scroll to **Видеокарта**. Select a card, then choose **Графический API** in
   its separate list and click **Сохранить карту и API для следующего запуска**.
   Only compatible APIs for that card are offered. **Сейчас работает** continues to
   show the effective GPU; the highlighted choice becomes active after closing
   and launching the example again. **Автоматический выбор** clears the saved
   device preference when saved; API selection can be automatic independently.
   Unsupported or ambiguous device/API combinations cannot be saved.
7. Scroll to **Показ кадров**. **VSync** switches between FIFO (on) and Immediate
   (off) when the target is supported. Unsupported toggles are disabled with an
   explicit caption. **Режим показа** cycles only advertised FIFO/adaptive FIFO/
   Immediate/Mailbox choices; adaptive FIFO may tear when late, while Mailbox
   displays the latest queued frame at vertical blank. **Лимит FPS** cycles no
   cap, 30/60/90/144/1 FPS. Choices remain staged until applied. Use **Применить
   VSync и FPS** to apply only presentation settings; the main **Применить** also
   requests the window settings. These are separate operations with separate
   feedback, not an atomic native transaction. **Фактический режим показа** and
   **Действующий лимит** report observed state. **Сбросить выбор** restores both
   window and presentation choices from observations.
8. Close with **Закрыть**, Escape or the native window close button.

Status distinguishes pending, confirmed and failed requests; native failures can
leave partial changes, so actual state is displayed separately. Resize/DPI events
update layout and the rendering surface. Monitor selection moves rendering to
that display. The GPU inventory and effective adapter appear below the window
state. `--list-adapters` lists grouped cards and their APIs without a window;
`--adapter INDEX` selects a card and `--render-api auto|vulkan|dx12|metal|opengl`
selects the rendering API independently. Initialization revalidates actual surface
compatibility. Indices are temporary; obtain a fresh list before choosing.
The menu stores the chosen key in `desktop-displays/adapter-preference.txt`,
independently of the working directory. Explicit `--adapter INDEX` overrides the
card field and `--render-api` overrides the API field for the current run;
`--auto-adapter` bypasses saved fields to recover from a stale or
corrupt preference and lets you save a new choice in the UI. There is no silent
fallback. Changing GPU requires a new run. Missing/incompatible/ambiguous preferences fail
without silently choosing another GPU. Presentation caps use an independent
monotonic redraw deadline; fixed simulation continues between displayed frames,
including at 1 FPS. The cap neither selects monitor refresh nor changes fixed
tick duration. VSync, the GPU and OS scheduling may reduce actual FPS below
the requested cap. Display frequency remains a separate exclusive-fullscreen
choice; windowed and borderless use the desktop video mode.
Only the card/API preferences are persisted; window/display/presentation choices and timed
confirmation/revert remain outside this example's current persistence scope.

## Automatic checks and limits

`--presentation-smoke` scrolls and clicks the actual settings controls to toggle
VSync off/on where supported, cycle every advertised present mode at 1/30/90 FPS
and uncapped, apply through **Применить VSync и FPS**, and reset staged choices.
It checks Applied feedback and continued fixed ticks against elapsed time,
restores the initial presentation configuration and closes through the UI.
It asserts that window/display state and the monitor inventory revision stay
unchanged. This smoke does not request fullscreen or modify saved GPU preferences.
Use one smoke workflow per run.

On 2026-10-06, this routed presentation smoke passed on the RTX 3070/Vulkan:
all four explicit modes, VSync off/on, every smoke cap and Reset were exercised;
1266 fixed ticks at 16.666667 ms continued during the workflow. Package tests
(13), Clippy and headless font/layout/cache checks passed. The existing window
smoke also passed with ordinary desktop permissions, including 1600 × 900 /
60 Hz selection and restoration to the initial 1920 × 1080 / 144 Hz desktop.
Under the sandbox token that same exclusive workflow hit the already documented
upstream `DISP_CHANGE_FAILED` panic; presentation-only smoke passed in the sandbox.

`--adapter-smoke` checks that the surface-specific inventory is available at
Startup and that the effective GPU matches the explicit preference, then renders
briefly and exits. It does not change display modes. Use a fresh adapter index.

`--adapter-ui-smoke` uses the actual menu router to scroll, select the current
card, choose another compatible API and click Save, then verifies the file and unchanged effective
GPU. Use `--adapter-preference PATH` with a disposable file to isolate this check
from your saved choice. A subsequent `--adapter-smoke` with that path checks the
saved selection at native initialization. This UI-save/relaunch workflow passed
from Vulkan to Direct3D 12 on the RTX 3070 on 2026-10-06.

`--smoke` clicks the monitor list, resizing policy, mode and Apply controls through
`UiRouter`, verifies supported fullscreen transitions and returns to a resizable
1100 × 860 window. Exclusive also stages a supported alternate resolution/rate
(preferring 1600 × 900 / 60 Hz), checks a fresh OS inventory against actual
feedback, and verifies desktop restoration. It fails on timeout/rejection and
checks two seconds of idle without additional queries. Exclusive is exercised
only if supported. Windows success smoke requires ordinary desktop permissions;
a restricted test token can reject native display changes. The engine uses
unmodified upstream winit and currently supports only the happy path: native
exclusive activation/restoration rejection may panic instead of producing
recoverable feedback. Recovery is deferred in engine Milestone 5.

`--headless` checks font layout at DPI 1/1.25/2 and idle cache reuse. Domain tests
cover tabs/refresh, disabled unsupported actions, staged controls, DPI clicks,
resize/zero viewport and layout reuse. Unchanged frames reuse prepared layout.
GPU tests cover routed selection/save, rejected incompatible/ambiguous choices,
scroll preservation, file roundtrips, replacement/reset and read/write failures.
The card list groups backend observations by model metadata, which does not
guarantee physical identity. Indistinguishable identical cards are rejected for
an explicit card/API choice. Differing backend metadata may leave separate card
groups. See the engine graphics contract for these provisional limits.

Windows native controls/exclusive/refresh/monitor transfer and GPU smoke were checked.
Adapter enumeration, automatic adapter smoke and explicit Direct3D 12 adapter
smoke passed on an RTX 3070 on 2026-10-06. The engine's native renderer test also
covered Vulkan, OpenGL and the Direct3D 12 software adapter on that machine.
Visual acceptance is maintainer-owned. Linux/macOS, real cable removal and
non-unit native DPI remain manual/deferred. Native work happens only on explicit
requests or window events/pending confirmation; synchronous queries can still
stall the requesting frame. Timing/allocation and binary-size measurements remain
deferred. OS scaling is not physical panel DPI. Do not persist monitor IDs.

Engine contracts: `docs/DISPLAYS.md`, `docs/WINDOWS.md` and `docs/PRESENTATION.md`. Domain source lives in
`src/settings/`: graphics owns staged window choices; model owns menu status;
composition owns separate window/pacing/adapter controls; presentation owns routing/layout; runtime dispatches
SDK requests; frame_pacing owns staged VSync/FPS and observed feedback;
pacing_smoke owns presentation acceptance. Extend these domains for future settings.
