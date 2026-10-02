use gridthorn::{
    ApplicationRuntime, FixedStepConfig, FixedTime, GameCommandQueue, ScheduleBuilder,
    ScheduleStage, SimulationControl, SimulationSpeed,
};
use std::time::Duration;

#[derive(Default)]
struct Progress {
    ticks: Vec<u64>,
    commands: Vec<u32>,
    frames: u32,
}

/// Demonstrate timed playback and explicit stepping through the public SDK.
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut schedules = ScheduleBuilder::new();
    schedules.add_system(ScheduleStage::Startup, |world| {
        world.insert_resource(Progress::default());
        world.insert_resource(GameCommandQueue::<u32>::default());
    });
    schedules.add_system(ScheduleStage::FixedUpdate, |world| {
        let tick = world
            .read_resource(|time: &FixedTime| time.tick_index())
            .unwrap();
        let commands = world
            .update_resource_with(|queue: &mut GameCommandQueue<u32>| {
                queue.drain().collect::<Vec<_>>()
            })
            .unwrap();
        world.update_resource(|progress: &mut Progress| {
            progress.ticks.push(tick);
            progress.commands.extend(commands);
        });
    });
    schedules.add_system(ScheduleStage::Update, |world| {
        world.update_resource(|progress: &mut Progress| progress.frames += 1);
    });
    let config = FixedStepConfig::new(Duration::from_millis(10), 4)?;
    let mut app = ApplicationRuntime::with_fixed_step(schedules.build(), config);
    app.run_timed_frame(Duration::from_millis(15))?;
    app.world().update_resource(SimulationControl::pause);
    app.world()
        .update_resource(|queue: &mut GameCommandQueue<u32>| queue.push(7));
    let paused = app.run_timed_frame(Duration::from_secs(10))?;
    assert_eq!(paused.fixed_steps(), 0);
    assert_eq!(
        app.world().read_resource(|p: &Progress| p.commands.len()),
        Some(0)
    );
    app.world()
        .update_resource(|control: &mut SimulationControl| {
            control.resume();
            control.set_speed(SimulationSpeed::new(2, 1).unwrap());
        });
    app.run_timed_frame(Duration::from_millis(10))?;
    app.world()
        .update_resource(|control: &mut SimulationControl| {
            control.set_speed(SimulationSpeed::new(1, 2).unwrap());
        });
    app.run_timed_frame(Duration::from_millis(10))?;
    app.world().update_resource(SimulationControl::pause);
    app.run_frame(1)?;
    app.world().read_resource(|p: &Progress| {
        assert_eq!(p.ticks, vec![0, 1, 2, 3, 4]);
        assert_eq!(p.commands, vec![7]);
        assert_eq!(p.frames, 5);
        println!(
            "Simulation clock: ticks={:?}, commands={:?}, presentation_frames={}",
            p.ticks, p.commands, p.frames
        );
    });
    app.shutdown();
    Ok(())
}
