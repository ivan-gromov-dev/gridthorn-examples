use gridthorn_world::{ScheduleBuilder, ScheduleStage};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct SimulationState {
    startup_runs: u32,
    fixed_steps: u32,
    value: u64,
}

/// Observable result of a completed headless run.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SimulationSnapshot {
    pub(crate) startup_runs: u32,
    pub(crate) fixed_steps: u32,
    pub(crate) value: u64,
}

/// Deterministic fixed-update scenario that requires no application services.
pub(crate) struct HeadlessScenario {
    runtime: gridthorn_world::ScheduleRuntime,
}

impl HeadlessScenario {
    /// Build the same startup and fixed-update schedule boundary used by windowed applications.
    pub(crate) fn new() -> Self {
        let mut builder = ScheduleBuilder::new();
        builder
            .add_system(ScheduleStage::Startup, |world| {
                world.insert_resource(SimulationState {
                    startup_runs: 1,
                    ..SimulationState::default()
                });
            })
            .add_system(ScheduleStage::FixedUpdate, |world| {
                world.update_resource(|state: &mut SimulationState| {
                    state.fixed_steps += 1;
                    state.value = state.value.wrapping_mul(3).wrapping_add(7);
                });
            });
        Self {
            runtime: builder.build(),
        }
    }

    /// Execute startup once and the requested number of fixed steps.
    pub(crate) fn run(mut self, fixed_steps: u32) -> SimulationSnapshot {
        self.runtime.run_startup();
        for _ in 0..fixed_steps {
            self.runtime.run_fixed_update();
        }
        self.runtime
            .world()
            .read_resource(|state: &SimulationState| SimulationSnapshot {
                startup_runs: state.startup_runs,
                fixed_steps: state.fixed_steps,
                value: state.value,
            })
            .expect("startup schedule must initialize simulation state")
    }
}

#[cfg(test)]
mod test;
