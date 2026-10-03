use super::composition;
use gridthorn::ui::{
    UiCommand, UiCompositionError, UiEffect, UiLayout, UiNodeId, UiRoute, UiRouter, UiTransition,
    UiTree, UiVisualState,
};
use gridthorn::{
    ApplicationRuntime, ExitRequest, FrameTiming, InputState, KeyCode, RenderFrame,
    ScheduleBuilder, ScheduleStage, TextSystem, WindowScaleFactor, WindowViewport,
};

struct Interface {
    tree: UiTree,
    router: UiRouter,
    fonts: TextSystem,
    layout: Option<UiLayout>,
    frames: u32,
    animations: Vec<UiTransition>,
}

impl Interface {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let layers = std::env::args().any(|arg| arg == "--layers");
        let mut state = Self {
            tree: if layers {
                super::layers::tree()?
            } else {
                composition::tree()?
            },
            router: UiRouter::new(10_000),
            fonts: composition::fonts()?,
            layout: None,
            frames: 0,
            animations: Vec::new(),
        };
        if layers {
            let layout = state
                .tree
                .layout([900.0, 700.0], 1.0, Some(&mut state.fonts))?;
            super::layers::open(&mut state.tree, &mut state.router, &layout)?;
        }
        if std::env::args().any(|arg| arg == "--animations") {
            state.animations = super::animation::transitions(&state.tree, layers)?;
        }
        Ok(state)
    }

    fn input(&mut self, input: &InputState) -> Result<UiRoute, UiCompositionError> {
        let Some(layout) = &self.layout else {
            return Ok(UiRoute::default());
        };
        self.router.route(&mut self.tree, layout, input)
    }
    #[expect(
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        reason = "window dimensions and DPI enter validated presentation metrics"
    )]
    fn prepare(&mut self, viewport: WindowViewport, dpi: f64) -> Result<(), UiCompositionError> {
        self.layout = Some(self.router.layout(
            &self.tree,
            [
                viewport.width as f32 / dpi as f32,
                viewport.height as f32 / dpi as f32,
            ],
            dpi as f32,
            Some(&mut self.fonts),
        )?);
        Ok(())
    }
}

/// Compose and update presentation through public SDK APIs.
pub(crate) fn runtime(smoke: bool) -> Result<ApplicationRuntime, Box<dyn std::error::Error>> {
    let mut initial = Some(Interface::new()?);
    let mut schedules = ScheduleBuilder::new();
    schedules.add_system(ScheduleStage::Startup, move |world| {
        world.insert_resource(initial.take().expect("startup once"));
    });
    schedules.add_system(ScheduleStage::Input, move |world| {
        let input = world
            .read_resource(Clone::clone)
            .unwrap_or_else(InputState::default);
        let viewport = world
            .read_resource(|value: &WindowViewport| *value)
            .unwrap_or_default();
        let dpi = world
            .read_resource(|value: &WindowScaleFactor| value.0)
            .unwrap_or(1.0);
        let result = world.update_resource_with(|state: &mut Interface| {
            state.frames += 1;
            state.prepare(viewport, dpi)?;
            let mut route = state.input(&input)?;
            state.prepare(viewport, dpi)?;
            if let Some(layout) = &state.layout {
                let refreshed = state.router.route_events(&mut state.tree, layout, &[])?;
                route.platform.extend(refreshed.platform);
            }
            Ok::<_, UiCompositionError>(route)
        });
        let mut world_escape = false;
        match result {
            Some(Ok(route)) => {
                world_escape = route.world_events.iter().any(|event| {
                    matches!(
                        event,
                        gridthorn::InputEvent::Keyboard {
                            key: KeyCode::Escape,
                            state: gridthorn::ButtonState::Pressed
                        } | gridthorn::InputEvent::Key(gridthorn::KeyboardEvent {
                            logical_key: gridthorn::LogicalKey::Named(gridthorn::NamedKey::Escape),
                            state: gridthorn::ButtonState::Pressed,
                            ..
                        })
                    )
                });
                for effect in &route.effects {
                    println!("UI: {effect:?}");
                }
                super::interaction::apply_platform(world, route.platform);
            }
            Some(Err(error)) => {
                eprintln!("UI preparation failed: {error}");
                world.update_resource(|exit: &mut ExitRequest| exit.request());
            }
            None => {}
        }
        if world_escape
            || (smoke
                && world
                    .read_resource(|state: &Interface| state.frames >= 120)
                    .unwrap_or(false))
        {
            world.update_resource(|exit: &mut ExitRequest| exit.request());
        }
    });
    schedules.add_system(ScheduleStage::Update, animate);
    schedules.add_system(ScheduleStage::Render, |world| {
        let primitives = world
            .read_resource(|state: &Interface| {
                state
                    .layout
                    .as_ref()
                    .map(|layout| layout.primitives().to_vec())
                    .unwrap_or_default()
            })
            .unwrap_or_default();
        world.insert_resource(RenderFrame::default().with_ui(primitives));
    });
    Ok(ApplicationRuntime::new(schedules.build()))
}

/// Check all six controls, commands, DPI, resize, clipping and scroll without a GPU.
pub(crate) fn headless() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::args().any(|arg| arg == "--animations") {
        return super::animation::headless();
    }
    if std::env::args().any(|arg| arg == "--layers") {
        return super::layers::headless();
    }
    let mut state = Interface::new()?;
    assert_eq!(
        state.tree.command(UiNodeId(3), UiCommand::Activate)?,
        UiEffect::Activated
    );
    state.tree.command(UiNodeId(4), UiCommand::Activate)?;
    state
        .tree
        .command(UiNodeId(5), UiCommand::SetValue(200.0))?;
    state
        .tree
        .command(UiNodeId(6), UiCommand::Select(Some(3)))?;
    state
        .tree
        .command(UiNodeId(6), UiCommand::ScrollTo([0.0, 999.0]))?;
    state.tree.command(
        UiNodeId(7),
        UiCommand::SetText("Привет 日本語 العربية".into()),
    )?;
    state
        .tree
        .command(UiNodeId(3), UiCommand::Visual(UiVisualState::Hovered))?;
    for width in [320_u16, 900] {
        let mut baseline = None;
        for dpi in [1.0_f32, 1.25, 2.0] {
            let layout =
                state
                    .tree
                    .layout([f32::from(width), 650.0], dpi, Some(&mut state.fonts))?;
            let placements = layout.placements().to_vec();
            if let Some(previous) = &baseline {
                assert_eq!(previous, &placements);
            }
            baseline = Some(placements);
            assert_eq!(
                layout.placement(UiNodeId(6)).expect("list").scroll_offset,
                [0.0, 144.0]
            );
            assert_ne!(layout.primitives(), []);
            println!(
                "UI width {width}, DPI {dpi}: {} nodes, {} clipped paint groups",
                layout.placements().len(),
                layout.primitives().len()
            );
        }
    }
    super::interaction::headless()
}

/// Advance presentation and refresh geometry-dependent native text anchors.
fn animate(world: &mut gridthorn::WorldAccess<'_>) {
    let delta = world
        .read_resource(|timing: &FrameTiming| timing.frame_elapsed())
        .unwrap_or_default();
    let viewport = world
        .read_resource(|value: &WindowViewport| *value)
        .unwrap_or_default();
    let dpi = world
        .read_resource(|value: &WindowScaleFactor| value.0)
        .unwrap_or(1.0);
    let result = world.update_resource_with(|state: &mut Interface| {
        if state.animations.is_empty() {
            return Ok(Vec::new());
        }
        for transition in &mut state.animations {
            transition.advance(&mut state.tree, delta)?;
        }
        state.prepare(viewport, dpi)?;
        let route = state.router.route_events(
            &mut state.tree,
            state.layout.as_ref().expect("prepared layout"),
            &[],
        )?;
        Ok::<_, Box<dyn std::error::Error>>(route.platform)
    });
    match result {
        Some(Ok(requests)) => super::interaction::apply_platform(world, requests),
        Some(Err(error)) => {
            eprintln!("UI animation failed: {error}");
            world.update_resource(|exit: &mut ExitRequest| exit.request());
        }
        None => {}
    }
}
