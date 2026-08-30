use gridthorn_app::{ApplicationError, WindowControl, WindowLifecycle};
use gridthorn_world::{ScheduleBuilder, ScheduleRuntime, ScheduleStage};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct ExecutionCounts {
    startup: u32,
    fixed_update: u32,
    update: u32,
}

/// Window lifecycle that drives explicit ECS schedules.
pub(crate) struct ScheduleLifecycle {
    runtime: ScheduleRuntime,
    smoke_enabled: bool,
}

impl ScheduleLifecycle {
    /// Create the schedule example lifecycle.
    pub(crate) fn new(smoke_enabled: bool) -> Self {
        let mut builder = ScheduleBuilder::new();
        builder
            .add_system(ScheduleStage::Startup, |world| {
                world.insert_resource(ExecutionCounts {
                    startup: 1,
                    ..ExecutionCounts::default()
                });
            })
            .add_system(ScheduleStage::FixedUpdate, |world| {
                world.update_resource(|counts: &mut ExecutionCounts| counts.fixed_update += 1);
            })
            .add_system(ScheduleStage::Update, |world| {
                world.update_resource(|counts: &mut ExecutionCounts| counts.update += 1);
            });
        Self {
            runtime: builder.build(),
            smoke_enabled,
        }
    }

    fn counts(&mut self) -> Option<ExecutionCounts> {
        self.runtime
            .world()
            .read_resource(|counts: &ExecutionCounts| *counts)
    }
}

impl WindowLifecycle for ScheduleLifecycle {
    fn started(&mut self, _control: &mut WindowControl) -> Result<(), ApplicationError> {
        self.runtime.run_startup();
        Ok(())
    }

    fn idle(&mut self, control: &mut WindowControl) -> Result<(), ApplicationError> {
        self.runtime.run_fixed_update();
        self.runtime.run_update();
        if self.smoke_enabled && self.counts().is_some_and(|counts| counts.update >= 3) {
            control.exit();
        }
        Ok(())
    }
}

#[cfg(test)]
mod test;
