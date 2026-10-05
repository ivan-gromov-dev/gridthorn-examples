# Game settings / desktop displays

Run from the sibling examples repository:

```console
cargo run -p gridthorn_example_desktop_displays --locked
cargo run -p gridthorn_example_desktop_displays --locked -- --smoke
cargo run -p gridthorn_example_desktop_displays --locked -- --headless
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
   reported in status without crashing the application.
5. **Сбросить выбор** restores staged settings from actual observed state without
   another native operation. **Обновить список** queries once; returning from
   another tab queries once. Change desktop resolution/scaling or disconnect a
   second display, then refresh. IDs survive only their documented connection.
6. Close with **Закрыть**, Escape or the native window close button.

Status distinguishes pending, confirmed and failed requests; native failures can
leave partial changes, so actual state is displayed separately. Resize/DPI events
update layout and the rendering surface. Monitor selection moves rendering to
that display; GPU adapter selection, VSync and frame caps remain future work.
This example does not yet persist settings or supply a timed confirmation/revert.

## Automatic checks and limits

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

Windows native controls/exclusive/refresh/monitor transfer and GPU smoke were checked.
Visual acceptance is maintainer-owned. Linux/macOS, real cable removal and
non-unit native DPI remain manual/deferred. Native work happens only on explicit
requests or window events/pending confirmation; synchronous queries can still
stall the requesting frame. Timing/allocation and binary-size measurements remain
deferred. OS scaling is not physical panel DPI. Do not persist monitor IDs.

Engine contracts: `docs/DISPLAYS.md` and `docs/WINDOWS.md`. Domain source lives in
`src/settings/`: graphics owns staged window choices; model owns menu status;
composition owns controls; presentation owns routing/layout; runtime dispatches
SDK requests; smoke owns acceptance. Extend these domains for future settings.
