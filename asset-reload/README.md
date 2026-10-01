# Asset reload

Run `cargo run -p gridthorn_example_asset_reload` from the examples repository.
Edit `assets/sprite.ppm` (plain-text RGB values) while the window is open. The
worker polls contents every 250 ms; a later frame publishes the decoded texture. Invalid or
partially saved images retain the last good texture and emit a diagnostic;
repairing the image resumes reload automatically.

The raw `scene.txt` source declares a dependency on the texture, so changes to
the texture report both IDs in dependency-first order. The example declares
this edge in code; it does not parse scene files or implement serialization.

`cargo run -p gridthorn_example_asset_reload -- --smoke` performs file edits in
an automatically cleaned temporary directory and validates headless runtime
frames, dependency propagation, immutable old snapshots, failed-decode rollback,
recovery, and worker shutdown. It uses a shorter request interval for testing
and does not edit the checked-in assets.

This provisional development path uses `AssetReloader`: file reads and decoding
run on a background thread, while `PollEvents` only publishes a ready snapshot
and requests work at the configured cadence. Only one request/result can be
outstanding. Existing texture clones stay unchanged; each render frame resolves
the current value by ID. `Shutdown` joins the worker and can wait for active
disk I/O. Initial loading is synchronous, and no native file watcher is used.
