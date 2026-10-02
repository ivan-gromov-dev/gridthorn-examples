use gridthorn::{
    FixedStepConfig, FixedTime, GameCommandQueue, HeadlessSimulation, ScheduleBuilder,
    ScheduleStage,
};
use std::time::Duration;

/// Run identical authoritative commands through differently partitioned requests.
pub fn validate() -> Result<(), Box<dyn std::error::Error>> {
    let mut first = build()?;
    let mut second = build()?;
    let progress = first.run_ticks(100)?;
    second.run_ticks(40)?;
    second.run_ticks(60)?;
    let state = first.world().read_resource(|value: &u64| *value).unwrap();
    assert_eq!(
        Some(state),
        second.world().read_resource(|value: &u64| *value)
    );
    assert_eq!(state, 4957);
    assert_eq!(progress.completed_ticks, 100);
    first.shutdown();
    second.shutdown();
    println!("headless simulation: 100 contiguous ticks, state={state}, repeated result matches");
    Ok(())
}

fn build() -> Result<HeadlessSimulation, Box<dyn std::error::Error>> {
    let mut schedules = ScheduleBuilder::new();
    schedules.add_system(ScheduleStage::Startup, |world| {
        world.insert_resource(0_u64);
        let mut commands = GameCommandQueue::default();
        commands.push(7_u64);
        world.insert_resource(commands);
    });
    schedules.add_system(ScheduleStage::FixedUpdate, |world| {
        let tick = world
            .read_resource(|time: &FixedTime| time.tick_index())
            .unwrap();
        let commands = world
            .update_resource_with(|commands: &mut GameCommandQueue<u64>| {
                commands.drain().collect::<Vec<_>>()
            })
            .unwrap();
        world.update_resource(|value: &mut u64| *value += tick + commands.iter().sum::<u64>());
    });
    for stage in [
        ScheduleStage::PollEvents,
        ScheduleStage::Update,
        ScheduleStage::PostUpdate,
        ScheduleStage::Render,
    ] {
        schedules.add_system(stage, |_| panic!("headless runner invoked presentation"));
    }
    Ok(HeadlessSimulation::new(
        schedules.build(),
        FixedStepConfig::new(Duration::from_millis(10), 1)?,
    ))
}
