//! Integer economy, deterministic routing, scenarios, and durable world data.
pub mod codec;
mod construction;
mod employment;
pub mod launch;
pub mod model;
mod navigation;
mod settlement;
mod transport;

use gridthorn::{
    FixedStepConfig, FixedTime, GameCommandQueue, RandomStreams, Scenario, ScenarioRuntime,
    ScenarioState, ScheduleBuilder, ScheduleStage,
};
use model::{Command, Harbor};
use std::time::Duration;

pub type Root = ScenarioState<Harbor, Command>;
pub type Runner = ScenarioRuntime<Harbor, Command>;

/// Construct the same authoritative schedules used by the interactive game.
pub fn runner(name: &str, seed: u64) -> Result<Runner, Box<dyn std::error::Error>> {
    if !["harbor", "sandbox"].contains(&name) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("unknown scenario: {name}"),
        )
        .into());
    }
    let mut random = RandomStreams::new(seed);
    random.register("market")?;
    let mut data = Harbor::new();
    if name == "sandbox" {
        data.coins = 2000;
    }
    let initial = Root {
        data,
        commands: GameCommandQueue::new(),
        random,
    };
    let mut schedules = ScheduleBuilder::new();
    schedules.add_system(ScheduleStage::FixedUpdate, |world| {
        let tick = world
            .read_resource(|t: &FixedTime| t.tick_index())
            .expect("fixed time");
        world.update_resource(|root: &mut Root| {
            let commands: Vec<_> = root.commands.drain().collect();
            for command in commands {
                root.data.apply(command);
            }
            root.data.step(tick, &mut root.random);
        });
    });
    Ok(Runner::new(
        schedules.build(),
        config(),
        Scenario::new(name, 2, initial)?,
    )?)
}

pub fn config() -> FixedStepConfig {
    FixedStepConfig::new(Duration::from_millis(100), 8).expect("fixed configuration")
}

#[cfg(test)]
mod test;
