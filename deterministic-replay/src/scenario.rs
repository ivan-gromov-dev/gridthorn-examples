use gridthorn_simulation::{DeterministicRng, StateFingerprint};
use gridthorn_world::{ScheduleBuilder, ScheduleStage};

/// One authoritative command applied at a fixed tick.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SimulationCommand {
    pub(crate) tick: u64,
    pub(crate) delta: i64,
}

/// Complete deterministic input for one scenario run.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ScenarioInput {
    pub(crate) seed: u64,
    pub(crate) fixed_steps: u64,
    pub(crate) commands: Vec<SimulationCommand>,
}

struct SimulationState {
    tick: u64,
    value: i64,
    next_command: usize,
    commands: Vec<SimulationCommand>,
    random: DeterministicRng,
}

/// Runs fixed updates and fingerprints the resulting authoritative state.
pub(crate) fn run_scenario(input: ScenarioInput) -> u64 {
    let fixed_steps = input.fixed_steps;
    let mut builder = ScheduleBuilder::new();
    builder
        .add_system(ScheduleStage::Startup, move |world| {
            world.insert_resource(SimulationState {
                tick: 0,
                value: 0,
                next_command: 0,
                commands: input.commands.clone(),
                random: DeterministicRng::new(input.seed),
            });
        })
        .add_system(ScheduleStage::FixedUpdate, advance_simulation);

    let mut runtime = builder.build();
    runtime.run_startup();
    for _ in 0..fixed_steps {
        runtime.run_fixed_update();
    }

    runtime
        .world()
        .read_resource(fingerprint_state)
        .expect("startup schedule must initialize simulation state")
}

fn advance_simulation(world: &mut gridthorn_world::WorldAccess<'_>) {
    world.update_resource(|state: &mut SimulationState| {
        while let Some(command) = state.commands.get(state.next_command) {
            if command.tick != state.tick {
                break;
            }
            state.value = state.value.wrapping_mul(3).wrapping_add(command.delta);
            state.next_command += 1;
        }
        let random_delta = i64::from(state.random.next_u64().to_le_bytes()[0]);
        state.value = state.value.wrapping_mul(5).wrapping_add(random_delta);
        state.tick += 1;
    });
}

fn fingerprint_state(state: &SimulationState) -> u64 {
    let mut fingerprint = StateFingerprint::new();
    fingerprint.write_u64(state.tick);
    fingerprint.write_i64(state.value);
    fingerprint.write_u64(state.random.state());
    fingerprint.write_u64(
        u64::try_from(state.next_command)
            .expect("command cursor must fit into fingerprint encoding"),
    );
    fingerprint.finish()
}

#[cfg(test)]
mod test;
