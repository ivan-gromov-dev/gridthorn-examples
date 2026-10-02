# Simulation clock
Run `cargo run -p gridthorn_example_simulation_clock` from the examples workspace.
This provisional public SDK smoke checks normal speed, pause with queued command
retention, resume at 2x, slow playback at 1/2x, and explicit stepping while paused.
Five consecutive ticks consume the command once while all five frames update
presentation. No window or GPU is needed; this does not implement the planned
headless simulation service.
