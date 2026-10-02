mod codec;

use codec::EconomyCodec;
use gridthorn::{
    FixedStepConfig, FixedTime, GameCommandQueue, RandomStreams, Scenario, ScenarioRuntime,
    ScenarioState, ScheduleBuilder, ScheduleStage,
};

type Economy = ScenarioState<Vec<u64>, u64>;

/// Validate file replacement, fresh-runner continuation, and rejected-load rollback.
pub fn validate() -> Result<(), Box<dyn std::error::Error>> {
    let directory =
        std::env::temp_dir().join(format!("gridthorn-world-saving-{}", std::process::id()));
    std::fs::create_dir(&directory)?;
    let result = validate_in(&directory);
    std::fs::remove_dir_all(&directory)?;
    result
}

fn validate_in(directory: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let path = directory.join("economy.toml");
    let mut original = runtime()?;
    original.save_file(&path, &EconomyCodec)?;
    original.run_ticks(25)?;
    original
        .world()
        .update_resource(|state: &mut Economy| state.commands.push(100));
    original.save_file(&path, &EconomyCodec)?;
    let mut fresh = runtime()?;
    fresh.load_file(&path, &EconomyCodec)?;
    assert_eq!(fresh.snapshot()?.completed_ticks(), 25);
    assert_eq!(fresh.snapshot()?.state().commands.len(), 1);
    original.run_ticks(75)?;
    fresh.run_ticks(75)?;
    let expected = original.save_document(&EconomyCodec)?;
    assert_eq!(fresh.save_document(&EconomyCodec)?, expected);
    assert!(
        fresh
            .load_document(&expected.replace("schema = 1", "schema = 2"), &EconomyCodec)
            .is_err()
    );
    assert_eq!(fresh.save_document(&EconomyCodec)?, expected);
    original.shutdown();
    fresh.shutdown();
    println!(
        "world save: atomic replacement, fresh-runner continuation at tick 100, and rollback verified"
    );
    Ok(())
}

fn runtime() -> Result<ScenarioRuntime<Vec<u64>, u64>, Box<dyn std::error::Error>> {
    let mut random = RandomStreams::new(42);
    random.register("economy")?;
    let scenario = Scenario::new(
        "economy",
        1,
        Economy {
            data: Vec::new(),
            commands: GameCommandQueue::new(),
            random,
        },
    )?;
    let mut schedules = ScheduleBuilder::new();
    schedules.add_system(ScheduleStage::FixedUpdate, |world| {
        let tick = world
            .read_resource(|time: &FixedTime| time.tick_index())
            .unwrap();
        world.update_resource(|state: &mut Economy| {
            let sum: u64 = state.commands.drain().sum();
            state
                .data
                .push(state.random.next_u64("economy").unwrap() ^ tick ^ sum);
        });
    });
    Ok(ScenarioRuntime::new(
        schedules.build(),
        FixedStepConfig::default(),
        scenario,
    )?)
}
