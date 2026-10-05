# Multilingual workbench

An integrated Milestone 4 showcase using only the public Gridthorn facade.
APIs are provisional. It combines six controls, font fallback, four Fluent catalogs,
plural/decimal formatting, editable Unicode text, clipboard/IME platform requests,
scrolling/clipping, nested modal dialogs, a nonmodal context popup and presentation
transitions driven by unscaled host-frame time.

```console
cargo run -p gridthorn_example_multilingual_workbench --locked
cargo run -p gridthorn_example_multilingual_workbench --locked -- --headless
cargo run -p gridthorn_example_multilingual_workbench --locked -- --smoke
cargo test -p gridthorn_example_multilingual_workbench --locked
```

Run from the examples repository, or add `--manifest-path ../gridthorn-examples/Cargo.toml`
when running from the engine repository. The example shares the licensed Noto assets
in `../multilingual-text/assets/fonts`; those files and their OFL licenses must be
included when distributing it. Catalog assets belong to this example.

## Interactive tour

1. Select English, Russian, Arabic or Japanese. Control captions and the live worker/
   balance preview update. The English-only message makes fallback visible.
2. Drag the worker-count slider through 0, 1, 2, 5 and 21 to inspect plural rules.
3. Edit the prefilled mixed-script text. Try combining `é`, Arabic with Latin digits,
   Cyrillic and Japanese. Use Shift+arrows and Ctrl/Command+A/C/X/V for selection
   and clipboard. Committed text is preserved when changing language.
4. Open the review dialog to see your editor value interpolated into a localized
   greeting. Open nested confirmation; confirm or Escape to restore parent focus.
5. Open context actions from the review. Insert the multilingual sample or click
   outside to dismiss only the popup; the review's modal scope still blocks the base.
6. Close the review and reopen it. Toggle window animation, resize the window,
   scroll short viewports and use Tab/Shift+Tab or arrows to navigate.

Escape cancels active IME composition first, then closes the top dismissible window,
then clears control focus; a subsequent unconsumed Escape exits the application.
The context popup is opened by a button; secondary-click positioning is not implemented.

## Layout reuse

The example retains prepared layout/paint while its presentation inputs stay unchanged.
Control values/visuals, scrolling, focus, selection, IME preedit/cursor, layer order,
animation offsets and viewport/DPI changes invalidate that snapshot. Pointer motion
without a visible hover/selection/value change reuses it. Localization is reformatted
only for count, language, editor-value or sample-insertion changes.

The engine's shaping/rasterization and text geometry path remains unchanged. Active
editing, dragging and animation still prepare fresh UI; performance during those
operations can remain limited by the provisional engine implementation.

## CPU performance baseline

Native isolation runs are `--idle-smoke`, `--editing-smoke`, `--slider-smoke`,
`--locale-smoke`, `--scroll-smoke`, `--windows-smoke`, `--animation-smoke`,
`--selection-smoke` and `--preedit-smoke`, each exiting
after 120 host frames. Select initial captions with `--locale=en-US`, `ru`,
`ar-EG` or `ja`; smoke modes are mutually exclusive. Idle leaves the UI unchanged.

Add `--long-smoke` to idle, editing, windows or animation runs for 1200 host
frames. Window/animation actions repeat their 120-frame sequence; idle/editing
continue their existing workload. Other native smoke modes reject this flag. Renderer/window
diagnostics remain capped at 240 samples; use external display capture for the
long tail. A longer smoke is still injected input, not native IME acceptance.

Editing focuses the field, warms for ten frames, then selects its complete value
and injects one bounded mixed-script replacement per frame through the public
router, including localized preview refresh. This tests injected editing rather
than the OS IME. Slider warms for ten frames, presses the worker slider on frame
11, alternates between 80 and 20 while dragging, and releases on frame 120.
It routes injected physical pointer coordinates at the actual window DPI and
includes normal localized-preview updates. It does not inject OS mouse events.
Locale warms for ten frames, then selects the next catalog each frame through a
public tree command, cycling all four languages while preserving editor text.
Its normal Changed effect refreshes captions and preview. This isolates catalog/UI
invalidation from pointer routing, window transitions and animation. Separate the
first locale cycle from later warmed switches when analyzing results.
Scroll opens a 1000x400 window to guarantee overflow, warms ten frames, then
injects alternating physical-pixel wheel deltas corresponding to 96 logical
pixels down/up through the public router. The script rejects a nonoverflowing
layout instead of measuring idle. This short-viewport workload does not establish
the 1000x800 native acceptance budget or OS wheel-event latency.
Windows and animation run the same fixed action sequence at 1000x800: open review
on frame 11, open nested confirmation on 26, close confirmation on 41, open popup
on 56, close popup on 71, close review on 86, then reopen/close review on 101/116.
Only animation enables the normal 350-ms host-time transitions. Locale/editor stay
unchanged. These modes inject Activated effects through the normal application
handler, not OS pointer events. Frame-based actions can supersede transitions
before completion; do not treat the two modes as identical animation-time samples.
Selection/preedit focus the editor during ten warm frames without changing its
committed value. Selection alternates between a caret at byte zero and selecting
the complete value through the public selection API. Preedit alternates bounded
Japanese/Arabic combining-text composition events with the composition cursor at
the end, then cancels on frame 120. Normal layout/paint and native text-anchor
requests run. These are injected selection/composition workloads, not OS keyboard,
IME candidate-window or system clipboard acceptance.
Avoid manual input during measurements. Native logs record
physical viewport and engine-reported scale factor; changing these flags does
not emulate monitor DPI.

```powershell
$env:GRIDTHORN_RENDER_PERFORMANCE = '1'
cargo run -p gridthorn_example_multilingual_workbench --release --locked -- --idle-smoke --locale=ja
cargo run -p gridthorn_example_multilingual_workbench --release --locked -- --editing-smoke --locale=ja
cargo run -p gridthorn_example_multilingual_workbench --release --locked -- --slider-smoke --locale=ja
cargo run -p gridthorn_example_multilingual_workbench --release --locked -- --locale-smoke --locale=en-US
cargo run -p gridthorn_example_multilingual_workbench --release --locked -- --scroll-smoke --locale=ja
cargo run -p gridthorn_example_multilingual_workbench --release --locked -- --windows-smoke --locale=ja
cargo run -p gridthorn_example_multilingual_workbench --release --locked -- --animation-smoke --locale=ja
cargo run -p gridthorn_example_multilingual_workbench --release --locked -- --selection-smoke --locale=ja
cargo run -p gridthorn_example_multilingual_workbench --release --locked -- --preedit-smoke --locale=ja
Remove-Item Env:GRIDTHORN_RENDER_PERFORMANCE
```

The engine collects bounded CPU diagnostics and asynchronous render-pass GPU
timestamps when supported. GPU timestamps exclude uploads and display/compositor
timing. CPU host present-call cadence does not establish displayed intervals.

Set `GRIDTHORN_WORKBENCH_PERFORMANCE=1` to collect up to 240 successful input-system
samples and print `workbench_cpu` CSV rows at shutdown. Columns separate prepare
before input, real input routing, scripted workload routing, effects (localized
preview refresh), prepare after effects and empty-event anchor refresh. Snapshot
capture and router layout columns are nested within prepare costs; do not add
them again. Samples use host frame numbers, not renderer/present frame numbers.
Only input-system work is profiled: animation, render extraction, platform request
application, stdout printing and native event-loop callbacks are excluded.
Startup/headless preparation is excluded. Unset the variable for normal runs.

Run `cargo run -p gridthorn_example_multilingual_workbench --release --locked -- --performance`
and repeat without `--release` for development overhead. The CSV output measures
four catalog locales at 1000×800 logical pixels and DPI 1/2, with ten warmups and
100 samples per operation. Construction and first preparation are single samples
per configuration; their repeated percentile columns do not represent a distribution.
Times include destruction of temporary results. Editing changes the mixed-script
field directly, excluding command dispatch, localization refresh and native input.
Operations share a warmed service and execute in the printed order. Idle reuse,
empty routing and render-input cloning exclude renderer geometry and GPU work.
No native FPS, GPU/present, allocation or memory measurement is implied.

The manual long-field probe runs through public UI/font APIs:

```console
cargo test -p gridthorn_example_multilingual_workbench --release --locked measure_long_field_editing -- --ignored --nocapture --test-threads=1
```

Run alone with UI/text diagnostic flags unset. It repeats four script phrases
8/64/256 times at synthetic DPI 1/2 in three overlapping fields with a top modal
layer. Each cycle selects the whole focused value, injects preedit, prepares paint,
commits one of two alternating replacements and prepares paint again. Ten warmups
precede 100 individual samples. Font loading, initial layout, source generation
and final validation are outside timing. Initial layouts rejected with
`Text(TooLarge)` are reported as `long_field_rejected` and have no timing samples.
Successful cases verify committed text, cleared preedit, focus and layer order.
This measures warm repeated text, not cold unique-glyph costs, allocations,
native DPI, OS IME or display performance. The font service locale remains en-US;
script labels describe field content rather than changing catalogs/service locale.

For phase attribution, enable `GRIDTHORN_UI_PERFORMANCE=1` and
`GRIDTHORN_TEXT_PERFORMANCE=1`, then run the same command with
`measure_long_field_phases` as the test filter. This separate probe includes timer
overhead and prints bounded layout/geometry/paint/decoration and text layout/raster
diagnostics. Decoration is nested within paint; do not add the two timings.

## Verification matrix

| Language/content | Headless | Windows native smoke | Interactive native IME |
| --- | --- | --- | --- |
| English: captions, plurals, decimals, ligatures | Automated | Automated submission | Manual checklist below; not yet accepted |
| Russian: Cyrillic, plurals, decimals | Automated | Automated submission | Manual checklist below; not yet accepted |
| Arabic (Egypt): bidi, Arabic digits, plural categories | Automated | Automated submission | Manual checklist below; not yet accepted |
| Japanese: fallback fonts, editor/preedit | Automated | Automated submission | Manual checklist below; not yet accepted |
| Linux/macOS, all four languages | Not exercised on these hosts | Not exercised | Not exercised |

The headless flow checks the native interface's catalog publication, editable-value
preservation, modal blocking, focus restoration, context actions, reopening, resize
and DPI 1/2. The editor flow injects ordered commits/preedit/cancellation, grapheme
selection replacement and focus loss. These injected events do not validate an OS IME.
Native smoke presents 120 frames, switches all four catalogs and cycles through review,
confirmation and context actions. It verifies native lifecycle/submission, not visual
acceptance. Real monitor/DPI changes, accessibility and performance remain unmeasured.

For native acceptance on each platform/language, record OS, layout/IME, DPI, result
and any defect. Check glyph coverage and wrapping, mixed-direction selection/caret,
copy/cut/paste, and Japanese composition/commit/cancel. During composition, press
Escape once (cancel preedit), then again (focus/window dismissal); switch focus and
Alt+Tab away to ensure composition cancels without duplicate commits. Resize while
editing and check the candidate-window anchor. Confirm nested dialogs block base
controls and outside popup dismissal cannot activate the control underneath.

The supplied fonts cover the showcased scripts; emoji coverage is not promised.
Editing arrows follow logical grapheme order for bidi text. The renderer's span-based
font path still requires broader throughput and native acceptance coverage.

A manual release CPU probe covers three overlapping field layers, four scripts
and DPI 1/2. It measures 32 alternating pointer events and a complete injected
preedit → paint → commit → paint cycle with two alternating bounded values:

```powershell
cargo test -p gridthorn_example_multilingual_workbench --release --locked measure_overlapping_font_editing -- --ignored --nocapture --test-threads=1
```

Run alone without `GRIDTHORN_UI_PERFORMANCE` or `GRIDTHORN_TEXT_PERFORMANCE`.
Each configuration has ten warm calls and 100 individually timed calls. Font
loading, tree construction/opening and initial text-session stop feedback are
outside timing; editing includes selection, routing, shaping/raster preparation,
replacement and release. This is a small warm workload, with no GPU, presentation,
allocator accounting, cold-cache, long/unique-text or real OS IME claim. Detailed
results and remaining limits are in the engine's
[performance review](../../gridthorn-engine/docs/PERFORMANCE_REVIEW.md).

A separate expanded asset-font probe measures full public router layout/paint for
3/16/64 overlapping layers with1/16 unique short fields per layer, four scripts
and synthetic DPI1/2, up to1024 fields. Each configuration has10 warm calls and
20 individually timed calls. Tree/font construction and opening layers are outside
timing; top modal focus/hit scope and the expected primitive count are checked.

```powershell
cargo test -p gridthorn_example_multilingual_workbench --release --locked measure_expanded_font_layers -- --ignored --nocapture --test-threads=1
```

Run alone with UI/text performance diagnostics unset. These are CPU preparation
measurements, not GPU/display intervals, allocation counts or real OS IME tests.
Unique field values intentionally pressure the bounded layout cache. Fully
overlapping layers still contribute paint; this fixture does not benchmark
occlusion culling or virtualized fields. It does not establish a realtime budget
for1024 asset-font fields.

The Text/UI CPU review now attributes repeated shaping across arrangement, editing
geometry and paint, and repeats the matrix after bounded per-pass prepared text
and service-local glyph-span reuse. Japanese1024-field preparation is faster but
still exceeds a16.67ms CPU budget; closed managed layers retain sizing work.
Long-field DPI2 requests can still reject at the raster work limit. These limits
and concrete backend/incremental-layout follow-ups are recorded in the engine
review; native OSIME and whole-frame/display acceptance remain outstanding.

Use `measure_font_layer_visibility` with diagnostics unset to compare64x16
registered closed fields with one open16-field layer. It checks0/16 primitives and
records10 warm+20 timed calls for English/Japanese at syntheticDPI1/2.
Use `measure_expanded_font_phases` with both `GRIDTHORN_UI_PERFORMANCE` and
`GRIDTHORN_TEXT_PERFORMANCE` set to1 to attribute1024-field English/Japanese
preparation atDPI1/2. The existing `measure_long_field_phases` also prints backend
shape/extract and raster-loop/snapshot subphases. Each diagnostic operation has
its own sample cap; their percentiles and indices cannot be summed or paired.

```powershell
cargo test -p gridthorn_example_multilingual_workbench --release --locked measure_font_layer_visibility -- --ignored --nocapture --test-threads=1
```
