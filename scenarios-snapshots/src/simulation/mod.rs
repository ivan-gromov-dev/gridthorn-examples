mod launch;

pub use launch::run;

use gridthorn::{
    FixedStepConfig, FixedTime, GameCommandQueue, RandomStreams, Scenario, ScenarioRuntime,
    ScenarioState, ScheduleBuilder, ScheduleStage, StateFingerprint,
};

type Economy = ScenarioState<Vec<u64>, u64>;

/// Compare independent execution and snapshot continuation through public SDK contracts.
///
/// # Errors
/// Returns SDK initialization, execution, or snapshot errors.
///
/// # Panics
/// Panics when the demonstrated repeatability or continuation contract fails.
pub fn validate() -> Result<(), Box<dyn std::error::Error>> {
    let scenario = scenario(42)?;
    let mut first = runtime(scenario.clone())?;
    let mut second = runtime(scenario.clone())?;
    let initial = first.snapshot()?;
    first.run_ticks(25)?;
    first
        .world()
        .update_resource(|state: &mut Economy| state.commands.push(100));
    let checkpoint = first.snapshot()?;
    first.run_ticks(75)?;
    let expected = first.snapshot()?;
    second.restore(&checkpoint)?;
    second.run_ticks(75)?;
    let restored = second.snapshot()?;
    assert_eq!(expected.state(), restored.state());
    assert_eq!(restored.completed_ticks(), 100);
    first.restore(&initial)?;
    first.run_ticks(25)?;
    first
        .world()
        .update_resource(|state: &mut Economy| state.commands.push(100));
    first.run_ticks(75)?;
    assert_eq!(expected.state(), first.snapshot()?.state());
    let mut fingerprint = StateFingerprint::new();
    fingerprint.write_u64(expected.completed_ticks());
    fingerprint.write_u64(expected.state().random.seed());
    fingerprint.write_u64(expected.state().data.len() as u64);
    for value in &expected.state().data {
        fingerprint.write_u64(*value);
    }
    for (name, state) in expected.state().random.states() {
        fingerprint.write_u64(name.len() as u64);
        fingerprint.write_bytes(name.as_bytes());
        fingerprint.write_u64(state);
    }
    assert!(expected.state().commands.is_empty());
    fingerprint.write_u64(0);
    println!(
        "scenario economy.v1: replay and restored continuation match at tick 100, fingerprint={:016x}",
        fingerprint.finish()
    );
    first.shutdown();
    second.shutdown();
    Ok(())
}

fn scenario(seed: u64) -> Result<Scenario<Vec<u64>, u64>, Box<dyn std::error::Error>> {
    let mut random = RandomStreams::new(seed);
    random.register("economy")?;
    random.register("weather")?;
    Ok(Scenario::new(
        "economy",
        1,
        Economy {
            data: Vec::new(),
            commands: GameCommandQueue::new(),
            random,
        },
    )?)
}

fn runtime(
    scenario: Scenario<Vec<u64>, u64>,
) -> Result<ScenarioRuntime<Vec<u64>, u64>, Box<dyn std::error::Error>> {
    let mut schedules = ScheduleBuilder::new();
    schedules.add_system(ScheduleStage::FixedUpdate, |world| {
        let tick = world
            .read_resource(|time: &FixedTime| time.tick_index())
            .unwrap();
        world.update_resource(|state: &mut Economy| {
            let command_sum: u64 = state.commands.drain().sum();
            let economy = state.random.next_u64("economy").unwrap();
            let weather = state.random.next_u64("weather").unwrap();
            state.data.push(economy ^ weather ^ tick ^ command_sum);
        });
    });
    Ok(ScenarioRuntime::new(
        schedules.build(),
        FixedStepConfig::default(),
        scenario,
    )?)
}
