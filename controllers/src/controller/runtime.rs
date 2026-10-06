use super::{composition, indicators, model::Devices};
use gridthorn::controller::{
    ControllerAxis, ControllerButton, ControllerEvent, ControllerFeedback, ControllerId,
    ControllerInfo, ControllerPolling, RumbleRequest,
};
use gridthorn::ui::{UiCompositionError, UiControl, UiEffect, UiNodeId, UiRouter, UiTree};
use gridthorn::{
    ApplicationRuntime, ButtonState, ExitRequest, InputBuffer, InputEvent, InputState, KeyCode,
    MouseButton, RenderFrame, ScheduleBuilder, ScheduleStage, TextSystem, WindowScaleFactor,
    WindowViewport,
};

pub(super) struct Monitor {
    pub(super) devices: Devices,
    pub(super) tree: UiTree,
    router: UiRouter,
    fonts: TextSystem,
    request: u64,
    frames: u32,
    refresh_requested: bool,
}
impl Monitor {
    pub(super) fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let devices = Devices::default();
        Ok(Self {
            tree: composition::tree(&devices)?,
            devices,
            router: UiRouter::new(0),
            fonts: composition::fonts()?,
            request: 0,
            frames: 0,
            refresh_requested: false,
        })
    }
    /// Refresh inventory, route the selected device and draw its live state.
    pub(super) fn frame(
        &mut self,
        input: &InputState,
        size: [f32; 2],
        dpi: f32,
    ) -> Result<(RenderFrame, Option<RumbleRequest>), UiCompositionError> {
        let previous = self.devices.selected;
        if self.devices.sync(input) {
            self.tree
                .replace(composition::tree(&self.devices)?.root().clone())?;
        }
        if previous != self.devices.selected {
            self.router
                .set_controller_navigation(self.devices.selected.is_some());
        }
        let layout = self
            .router
            .layout(&self.tree, size, dpi, Some(&mut self.fonts))?;
        let events: Vec<_> = input
            .events()
            .iter()
            .filter(|event| match event {
                InputEvent::Controller(
                    ControllerEvent::Button { id, .. } | ControllerEvent::Axis { id, .. },
                ) => Some(*id) == self.devices.selected,
                _ => true,
            })
            .cloned()
            .collect();
        let mut request = None;
        for event in &events {
            let route =
                self.router
                    .route_events(&mut self.tree, &layout, std::slice::from_ref(event))?;
            for (node, effect) in route.effects {
                if node == UiNodeId(6) && effect == UiEffect::Activated {
                    self.refresh_requested = true;
                }
                let pointer_selection = node == UiNodeId(2) && effect == UiEffect::Changed && {
                    matches!(
                        event,
                        InputEvent::MouseButton {
                            button: MouseButton::Left,
                            state: ButtonState::Pressed
                        }
                    )
                };
                if pointer_selection || (node == UiNodeId(7) && effect == UiEffect::Activated) {
                    if let Some(UiControl::List {
                        selected: Some(row),
                        ..
                    }) = self.tree.node(UiNodeId(2)).map(|node| &node.control)
                    {
                        self.devices.select(*row);
                    }
                    self.router
                        .set_controller_navigation(self.devices.selected.is_some());
                    self.tree
                        .replace(composition::tree(&self.devices)?.root().clone())?;
                }
                if node == UiNodeId(3)
                    && effect == UiEffect::Activated
                    && let Some(id) = self.devices.selected
                {
                    self.request += 1;
                    request = Some(RumbleRequest {
                        request: self.request,
                        id,
                        strong: 0.4,
                        weak: 0.2,
                        duration_ms: 200,
                    });
                }
            }
        }
        for event in input.events() {
            match event {
                InputEvent::Controller(ControllerEvent::Feedback {
                    request, result, ..
                }) => {
                    self.devices.feedback = match result {
                        Ok(()) => format!("Вибрация #{request}: запрос отправлен"),
                        Err(error) => format!("Вибрация #{request}: {error}"),
                    }
                }
                InputEvent::Controller(ControllerEvent::Unavailable(error)) => {
                    self.devices.feedback = format!("Контроллеры недоступны: {error}");
                }
                _ => {}
            }
        }
        let mut root = self.tree.root().clone();
        if let Some(label) = root.children.iter_mut().find(|node| node.id == UiNodeId(4)) {
            label.control = UiControl::Label(self.devices.feedback.clone());
        }
        self.tree.replace(root)?;
        let layout = self
            .router
            .layout(&self.tree, size, dpi, Some(&mut self.fonts))?;
        let mut primitives = layout.primitives().to_vec();
        primitives.extend(indicators::paint(&layout, input, &self.devices, dpi));
        self.frames += 1;
        Ok((RenderFrame::default().with_ui(primitives), request))
    }
}
/// Native visual monitor with game-owned device selection.
pub(crate) fn runtime(smoke: bool) -> Result<ApplicationRuntime, Box<dyn std::error::Error>> {
    let mut monitor = Monitor::new()?;
    let mut schedules = ScheduleBuilder::new();
    schedules.add_system(ScheduleStage::Startup, |world| {
        world.update_resource(ControllerPolling::request_poll);
    });
    let mut last_poll = std::time::Instant::now();
    schedules.add_system(ScheduleStage::Input, move |world| {
        let input = world
            .read_resource(Clone::clone)
            .unwrap_or_else(InputState::default);
        let viewport = world
            .read_resource(|viewport: &WindowViewport| *viewport)
            .unwrap_or_default();
        let dpi = world
            .read_resource(|dpi: &WindowScaleFactor| dpi.0)
            .unwrap_or(1.0);
        let (size, dpi) = metrics(viewport, dpi);
        match monitor.frame(&input, size, dpi) {
            Ok((frame, request)) => {
                world.insert_resource(frame);
                if let Some(request) = request {
                    world.update_resource(|feedback: &mut ControllerFeedback| {
                        feedback.rumble(request);
                    });
                }
            }
            Err(error) => {
                eprintln!("Controller UI: {error}");
                world.update_resource(|exit: &mut ExitRequest| exit.request());
            }
        }
        if std::mem::take(&mut monitor.refresh_requested)
            || (monitor.devices.selected.is_some()
                && last_poll.elapsed() >= std::time::Duration::from_millis(50))
        {
            world.update_resource(ControllerPolling::request_poll);
            last_poll = std::time::Instant::now();
        }
        if input.key_just_pressed(KeyCode::Escape) || (smoke && monitor.frames >= 6) {
            if smoke {
                assert_eq!(input.controller_availability(), Some(&Ok(())));
                println!(
                    "Controller native visual smoke passed: {} connected",
                    input.controllers().count()
                );
            }
            world.update_resource(|exit: &mut ExitRequest| exit.request());
        }
    });
    Ok(ApplicationRuntime::new(schedules.build()))
}
#[expect(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    reason = "native dimensions and DPI become presentation coordinates"
)]
fn metrics(viewport: WindowViewport, dpi: f64) -> ([f32; 2], f32) {
    (
        [
            viewport.width as f32 / dpi as f32,
            viewport.height as f32 / dpi as f32,
        ],
        dpi as f32,
    )
}
/// Inject device discovery and controls, validate navigation, indicators and disconnect.
pub(crate) fn headless() -> Result<(), Box<dyn std::error::Error>> {
    let id = ControllerId(1);
    let mut input = InputBuffer::new();
    input.push(InputEvent::Controller(ControllerEvent::Ready));
    input.push(InputEvent::Controller(ControllerEvent::Connected(
        ControllerInfo {
            id,
            name: "Injected controller".into(),
            model_uuid: [0; 16],
            vendor_id: None,
            product_id: None,
            buttons: vec![ControllerButton::South],
            axes: vec![ControllerAxis::LeftStickX],
            rumble_supported: false,
        },
    )));
    let mut monitor = Monitor::new()?;
    monitor.frame(&input.snapshot(), [1000.0, 800.0], 1.0)?;
    monitor.devices.select(1);
    monitor.router.set_controller_navigation(true);
    monitor
        .tree
        .replace(composition::tree(&monitor.devices)?.root().clone())?;
    for button in [
        ControllerButton::RightShoulder,
        ControllerButton::RightShoulder,
        ControllerButton::South,
    ] {
        for state in [ButtonState::Pressed, ButtonState::Released] {
            input.push(InputEvent::Controller(ControllerEvent::Button {
                id,
                button,
                state,
            }));
        }
    }
    input.push(InputEvent::Controller(ControllerEvent::Axis {
        id,
        axis: ControllerAxis::LeftStickX,
        value: 0.8,
    }));
    let (frame, request) = monitor.frame(&input.snapshot(), [1000.0, 800.0], 1.0)?;
    assert_ne!(frame.ui(), []);
    assert_eq!(request.unwrap().id, id);
    input.push(InputEvent::Controller(ControllerEvent::Disconnected(id)));
    monitor.frame(&input.snapshot(), [1000.0, 800.0], 1.0)?;
    assert_eq!(monitor.devices.selected, None);
    assert_eq!(monitor.devices.items(), ["Клавиатура и мышь"]);
    println!("Controller visual headless smoke passed");
    Ok(())
}
