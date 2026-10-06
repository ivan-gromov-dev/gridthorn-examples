use super::super::presentation::Menu;
use gridthorn::{InputState, WindowScaleFactor, WindowViewport, WorldAccess};
use gridthorn::{display::Displays, presentation::PresentationSettings, window::WindowSettings};
use std::time::Duration;

pub(super) struct Snapshot {
    pub viewport: WindowViewport,
    pub dpi: f64,
    pub inventory: Option<Displays>,
    pub input: Option<InputState>,
    pub window: Option<WindowSettings>,
    pub pacing: Option<PresentationSettings>,
    pub pacing_timing: Option<(u64, Duration)>,
}

pub(super) fn read(world: &WorldAccess<'_>) -> Snapshot {
    let viewport = world
        .read_resource(|value: &WindowViewport| *value)
        .unwrap_or_default();
    let dpi = world
        .read_resource(|value: &WindowScaleFactor| value.0)
        .unwrap_or(1.0);
    let inventory = world
        .read_resource(|menu: &Menu| {
            world.read_resource(|displays: &Displays| {
                menu.model.needs_sync(displays).then(|| displays.clone())
            })
        })
        .flatten()
        .flatten();
    let input = world
        .read_resource(|input: &InputState| (!input.events().is_empty()).then(|| input.clone()))
        .flatten();
    let window = world
        .read_resource(|menu: &Menu| {
            world.read_resource(|window: &WindowSettings| {
                menu.model
                    .graphics
                    .needs_sync(window)
                    .then(|| window.clone())
            })
        })
        .flatten()
        .flatten();
    let pacing = world
        .read_resource(|menu: &Menu| {
            world.read_resource(|settings: &PresentationSettings| {
                menu.model
                    .pacing
                    .needs_sync(settings)
                    .then(|| settings.clone())
            })
        })
        .flatten()
        .flatten();
    let pacing_timing = world
        .read_resource(|timing: &gridthorn::FrameTiming| {
            world.read_resource(|fixed: &gridthorn::FixedTime| {
                (timing.completed_ticks(), fixed.fixed_step())
            })
        })
        .flatten();
    Snapshot {
        viewport,
        dpi,
        inventory,
        input,
        window,
        pacing,
        pacing_timing,
    }
}
