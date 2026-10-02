use super::{runtime, scenario};
use gridthorn::StateFingerprint;

/// Run a named scenario through the dedicated CLI headless binary.
///
/// # Errors
/// Returns an error for unknown scenarios or failed SDK initialization/execution.
pub fn run(name: &str, ticks: u64, seed: u64) -> Result<(), Box<dyn std::error::Error>> {
    if name != "economy" {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("unknown scenario `{name}`"),
        )
        .into());
    }
    let mut runner = runtime(scenario(seed)?)?;
    runner.run_ticks(ticks)?;
    let snapshot = runner.snapshot()?;
    let mut fingerprint = StateFingerprint::new();
    fingerprint.write_u64(snapshot.completed_ticks());
    fingerprint.write_u64(seed);
    for value in &snapshot.state().data {
        fingerprint.write_u64(*value);
    }
    for (name, state) in snapshot.state().random.states() {
        fingerprint.write_bytes(name.as_bytes());
        fingerprint.write_u64(state);
    }
    println!(
        "scenario={name} completed_ticks={} seed={seed} fingerprint={:016x}",
        snapshot.completed_ticks(),
        fingerprint.finish()
    );
    runner.shutdown();
    Ok(())
}
