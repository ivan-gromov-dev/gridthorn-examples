use super::{Root, codec::HarborCodec, runner};
use gridthorn::{StateFingerprint, WorldSaveCodec};

/// Execute the CLI's explicit scenario/ticks/seed contract without assets or devices.
///
/// # Errors
/// Rejects malformed arguments, unknown scenarios, and runtime or save-codec failures.
pub fn run(arguments: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let [scenario_flag, name, ticks_flag, ticks, seed_flag, seed] = arguments else {
        return Err("expected --scenario harbor|sandbox --ticks N --seed N".into());
    };
    if scenario_flag != "--scenario" || ticks_flag != "--ticks" || seed_flag != "--seed" {
        return Err("invalid headless flags".into());
    }
    let seed: u64 = seed.parse()?;
    let mut runtime = runner(name, seed)?;
    runtime.run_ticks(ticks.parse()?)?;
    let snapshot = runtime.snapshot()?;
    let root: &Root = snapshot.state();
    let mut fingerprint = StateFingerprint::new();
    fingerprint.write_u64(snapshot.completed_ticks());
    fingerprint.write_bytes(HarborCodec.encode(&root.data, &root.commands)?.as_bytes());
    for (name, state) in root.random.states() {
        fingerprint.write_bytes(name.as_bytes());
        fingerprint.write_u64(state);
    }
    println!(
        "scenario={name} seed={seed} completed_ticks={} coins={} shipped={} fingerprint={:016x}",
        snapshot.completed_ticks(),
        root.data.coins,
        root.data.shipped,
        fingerprint.finish()
    );
    runtime.shutdown();
    Ok(())
}
