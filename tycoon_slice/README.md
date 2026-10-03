# Timber Harbor — playable Gridthorn tycoon slice

A cozy isometric settlement game using Gridthorn's provisional public SDK.
Windowed play and the headless binary execute the same integer workforce,
commands, road routing, scenario root and seeded market.

## Play

From the examples repository:

```console
cargo run -p tycoon_slice
```

From the engine repository:

```console
cargo run -p gridthorn_cli -- check ../gridthorn-examples/tycoon_slice
cargo run -p gridthorn_cli -- run ../gridthorn-examples/tycoon_slice
```

Press Enter, Escape or **Resume Game** to leave the initial menu. Start with
120 gold, one house with four beds, one lumberjack, one carpenter, two porters,
a forest, log warehouse, sawmill, plank warehouse, one raw-log port and roads.
Four initial logs keep both transport branches active immediately.

This is a continuous expansion game. Gold comes from port sales. The initial
port accepts **logs only**; finished planks accumulate in their warehouse.
To export planks, save up, build another port on the sandy coast (column 4),
select it and choose **Change to Sell Planks**. Build a house for additional
staff, then select the plank warehouse and hire a porter. Each house has four
places. All construction and hiring consume gold.

| Purchase | Gold |
| --- | ---: |
| Road cell | 3 |
| House / four beds | 40 |
| Forest plot / either warehouse | 60 |
| Sawmill | 100 |
| Port | 180 |
| Lumberjack | 50 |
| Carpenter | 75 |
| Porter | 35 |
| Move an unused building | 5 |

Logs sell for 6–8 gold; planks sell for 14–16. Quotes use the named deterministic
market RNG with unbiased sampling. There is no passive rent or automatic upkeep.
Demolition refunds half the building purchase price. Buildings with residents
or job assignments, the initial port and road cells under workers are protected.

## Workforce and selection

- A lumberjack owns one forest assignment and one log warehouse destination.
  Each forest accepts one lumberjack. Chopping takes 30 fixed ticks, then the
  worker carries one log along roads to its warehouse and returns.
- A carpenter works at one sawmill and delivers to a plank warehouse. It reserves
  one log at the sawmill, processes it over 40 ticks and carries the resulting
  plank to the warehouse. It waits if no logs are available.
- A porter owns one warehouse assignment and one destination: log warehouse to
  sawmill or log port, plank warehouse to plank port. Each trip carries one unit.
  Several porters can share a warehouse, sawmill or port. Warehouses rotate
  dispatch among waiting porters so the sawmill cannot starve the export route.
- Every worker owns a house reference; no house can exceed four residents.
  Hiring requires gold, a vacant bed, a suitable workplace with an adjacent road
  and a compatible destination. Allocation selects the first house with capacity
  in deterministic ID order.

Use **Select / Inspect** and click a building or forest, including its roof,
to see stock, residents or assigned workers. The list shows roles, house IDs,
source/destination IDs and work/transport status. Previous/Next Worker selects
an employee; **Assign Active Worker** then takes two map clicks: source followed
by destination. Right click cancels assignment and returns to selection.
A carrying or processing worker must finish its current task before reassignment;
rejected commands explain the reason in the footer. Changing a port resource
waits for incompatible in-flight cargo to finish, then existing incompatible
empty porters stop collecting until reassigned.

Movement advances one road cell every two ticks. A broken road stalls deliveries;
repair resumes them without losing cargo. Renewable forest plots produce one log
per chopping cycle. Full warehouses/mills preserve undelivered cargo until space
is available. There are up to 64 workers and 9,999 units per stock slot.

## Interface and menu

The top bar shows gold, logs, planks, building count, workers/housing capacity,
in-game `HH:MM:SS`, exported unit count and simulation status. Counts include
carried goods and the carpenter's reserved log. Forests are sites rather than
part of the building count. The upper-left menu button and Escape both toggle
the menu. Menu entry pauses the simulation; resume restores the selected speed
and the previous manual pause state.

| Control | Action |
| --- | --- |
| Pause / Space | Toggle pause while input, UI and animation remain live |
| 0.5× / 1× / 2× / 4× | Select speed and resume simulation |
| Tick Step | Advance exactly one fixed tick while paused |
| Menu / Escape | Enter or leave the pause menu |
| Resume Game / Enter in menu | Return to play |
| Save Game / Load Game in menu | Atomic disk save and validated world restore |
| Capture / Restore Snapshot in menu | Exact in-memory simulation plus scalar camera bookmark |
| New Game / Sandbox in menu | Restart normal scenario or start with 2,000 gold |
| Quit Game in menu | Exit and join audio/asset workers |
| Build tool + left click | Place one road/building after a valid press/release |
| Move / Demolish | Edit unused buildings; assigned buildings remain protected |
| WASD / arrows | Pan camera |
| Zoom + / Zoom - | Change camera extent |
| Paths / Chunks | Routes, search/frontier/budget diagnostics, reflection and chunk boundaries |
| ISO / Grid | Project the same integer world as isometric or square cells |

The default window is 1280×800. UI, artwork and button hitboxes scale together
with the physical viewport; smaller windows reduce text size. Map picking uses
the actual camera/viewport and excludes all UI panels. The square view is a
terrain diagnostic with the same isometric building artwork.

## Integrated engine capabilities

| Capability | Concrete game use |
| --- | --- |
| Square/isometric coordinates | Signed cell identities, projection toggle and camera-aware picking |
| Tilemaps, layers, chunks | Sparse ground/road layers with deterministic signed 4×4 chunks |
| Object placement | Exclusive occupancy, validity previews, moves, removal and refunds |
| Pathfinding | Deterministic bounded four-neighbor road search, work trips, recovery and diagnostics |
| Simulation clock | 100 ms ticks; rational half speed, 1×/2×/4×, pause and exact step |
| Headless simulation | Device-free binary executes the identical authoritative schedules |
| Scenarios, snapshots, RNG | harbor/sandbox, pending commands, exact continuation and named market stream |
| World saves | Versioned game codec, work phases/cargo/housing/port settings, validation and rollback |
| Scenario/headless CLI | Project declaration, listing, explicit ticks/seeds and canonical fingerprint |

Milestone 2 integration includes native input, frame/fixed schedules, game-state
and scene transitions, camera/sprites, ordered texture batching, SDK two-frame
animation, bitmap text/buttons, native PCM16 music and delivery effects,
reflection, scalar scene camera bookmarks and background asset reload.
Screen-anchored textured UI uses public sprites composed after world artwork,
followed by screen-space text. Nine-slice frames keep menu/button borders readable
across panel sizes and camera changes. Collision queries gate map interaction.

All three RGBA atlases and `skin.txt` participate in the development asset store
and dependency graph. A 250 ms background scan prepares edits; PollEvents
publishes complete batches. Keep atlas layout/dimensions when replacing images.
Invalid decodes preserve the last good artwork and show an error. Audio reload
remains outside the current SDK subset.

## Save contract and checks

Disk saves use `harbor-save.toml` beside this README. Loading through the menu
restores the exact world tick, queued commands, RNG, workers, reserved/carried
resources, stocks, dispatch cursor, housing, job assignments, port resource and
simulation controls. The menu remains open until explicitly resumed.
Scenario mismatches and invalid worlds leave the live simulation unchanged.

Scenario revision 2 and `harbor-v2` intentionally reject the previous automatic
workshop economy's revision-1 saves. Start a new game for this revision; there
is no silent conversion. Camera bookmarks accompany in-memory snapshots, not
disk world saves. The outer host frame accumulator is not restored.

```console
cargo test -p tycoon_slice
cargo run -p tycoon_slice -- --headless-smoke
cargo run -p tycoon_slice -- --smoke
cargo run -p tycoon_slice -- --silent
```

`--headless-smoke` checks presentation extraction and audio decoding without
window/GPU/audio devices. `--smoke` opens the real window, starts the game and
closes after 60 frames. `--silent` disables native audio; an unavailable device
also emits a diagnostic and permits play.

From the engine repository:

```console
cargo run -p gridthorn_cli -- scenario list ../gridthorn-examples/tycoon_slice
cargo run -p gridthorn_cli -- simulate ../gridthorn-examples/tycoon_slice --scenario harbor --ticks 1000 --seed 42
```

Focused tests cover construction rollback, housing/costs, multi-tick production,
port resource selection, multiple porters, fair dispatch, broken-road recovery,
exact snapshot/save continuation, malformed jobs/cargo/stock, load rollback,
atomic file replacement, menu/escape/speed, menu save/load during work, roof
selection, blocked map input, resized picking and camera scene validation.

All authoritative data belongs to the nested typed scenario root. The outer
runtime owns input, UI, camera, audio and rendering; only scenario schedules
mutate the economy. Host/device/asset worker state never enters the world save.
Supported scale is a finite 12×12 map and single-cell footprints. Arbitrary ECS
capture, replay files, release migrations, large-world benchmarks, cross-platform
native validation and distributable packaging remain deferred.

Original artwork, exact prompts and reproducible audio synthesis are included in
[assets/SOURCES.md](assets/SOURCES.md). Play needs no downloads or generators.
