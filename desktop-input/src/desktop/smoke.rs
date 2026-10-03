use gridthorn::{
    ExitRequest, InputEvent, InputState, PointerCapture, PointerCaptureMode, ScheduleBuilder,
    ScheduleStage,
};

#[derive(Default)]
struct SmokeState {
    step: usize,
    pending: bool,
}

/// Exercise each native capture request and its asynchronous feedback before shutdown.
pub(super) fn register(schedules: &mut ScheduleBuilder) {
    schedules.add_system(ScheduleStage::Startup, |world| {
        world.insert_resource(SmokeState::default());
    });
    schedules.add_system(ScheduleStage::Input, |world| {
        let input = world
            .read_resource(Clone::clone)
            .unwrap_or_else(InputState::default);
        let modes = [
            PointerCaptureMode::Confined,
            PointerCaptureMode::Locked,
            PointerCaptureMode::None,
        ];
        let request = world
            .update_resource_with(|state: &mut SmokeState| {
                if state.pending
                    && input.events().iter().any(|event| {
                        matches!(event,
                InputEvent::PointerCaptureChanged(status) if status.requested == modes[state.step])
                    })
                {
                    state.step += 1;
                    state.pending = false;
                }
                if state.step < modes.len() && !state.pending && input.focused() {
                    state.pending = true;
                    Some(modes[state.step])
                } else {
                    None
                }
            })
            .flatten();
        if let Some(mode) = request {
            world.update_resource(|capture: &mut PointerCapture| capture.request(mode));
        }
        if world
            .read_resource(|state: &SmokeState| state.step == modes.len())
            .unwrap_or(false)
        {
            world.update_resource(|exit: &mut ExitRequest| exit.request());
        }
    });
}
