# Crystal Trail

A small playable Milestone 2 integration example built entirely through public
Gridthorn APIs. Collect four gold crystals while navigating around two walls.

From the examples repository:

```powershell
cargo run -p classic_2d
cargo run -p classic_2d -- --headless-smoke
cargo run -p classic_2d -- --smoke
cargo test -p classic_2d
```

Enter or the mouse button starts/restarts the game. WASD or arrows move;
Space pauses/resumes, Enter or the button resumes from pause, Escape quits.
World Y increases down the screen: W/Up decreases Y and S/Down increases Y.
The game uses integer fixed-tick positions, AABB overlap queries, a scene-owned
arena entity, deferred scene replacement, and a game-state stack for pause.
Presentation animation uses host-frame time and stops while paused. The ordered
player, crystal, and wall sprites share one cloned atlas for the batching path.
Bitmap labels, a mouse button, and a timing overlay exercise runtime UI.

The original atlas and PCM16 WAV assets are included locally and may be reused
under this repository's license. An `assets` directory beside the executable
takes priority over the source package's assets. To relocate a build, copy the
executable and the complete `assets` directory together. Without an adjacent
directory, development runs use the source package's assets. A present but
incomplete adjacent directory produces a load error instead of falling back.
Asset resolution is independent of the working directory. This does not bundle
platform runtimes or establish clean-machine deployment support. Native audio runs on an
example-owned worker using the public `gridthorn_audio` native-output service.
Pause suspends voices; resume restores them; Shutdown stops voices and joins
the worker. Missing audio devices produce a diagnostic and allow silent play.
Headless smoke decodes the same assets without opening a device or window.

The worker queue holds 32 requests. Pickup submission is nonblocking and effects
can be discarded when it is full. Pause/resume publishes the latest state in a
separate atomic mailbox before requesting a wake. A full queue already contains
requests that will trigger consumption; stale wakes do not restore old states.
Intermediate pause transitions may coalesce. Shutdown waits for delivery and
joins the worker. Application latency still depends on worker progress.
An ignored release probe measures headless submission/shutdown and a separately
gated transport fixture; it does not measure device latency:

```console
cargo test -p classic_2d --release --locked measure_audio_worker_handoff -- --ignored --nocapture
```

Tests cover movement bounds, blocking walls, one-shot pickup, pause, victory,
scene cleanup, restart, and headless render extraction. `--smoke` opens a real
window, starts a round, and exits after 30 frames. It does not verify sound
quality or manual mouse interaction. Native platform suspend/resume is not
integrated by this example; the demonstrated suspension is game pause.

Reflection, scene persistence, asset dependencies/hot reload, and CLI watching
have dedicated sibling examples or engine tests; this game does not exercise
those services. Milestone 2 was closed on 2026-10-02 after maintainer gameplay
confirmation. The engine completion review records supported subsets and
remaining platform, audio-quality, and performance measurement deferrals.
