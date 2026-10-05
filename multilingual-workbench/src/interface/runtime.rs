use super::composition;
use super::native_performance::{NativePerformance, Phase};
use gridthorn::ui::{
    UiCommand, UiCompositionError, UiEffect, UiLayout, UiNodeId, UiRoute, UiRouter, UiTransition,
    UiTree,
};
use gridthorn::{
    ApplicationRuntime, ExitRequest, FrameTiming, InputState, KeyCode, RenderFrame,
    ScheduleBuilder, ScheduleStage, TextSystem, WindowScaleFactor, WindowViewport,
};

pub(super) struct Interface {
    performance: NativePerformance,
    pub(super) tree: UiTree,
    pub(super) router: UiRouter,
    pub(super) fonts: TextSystem,
    pub(super) layout: Option<UiLayout>,
    frames: u32,
    metrics: ([f32; 2], f32),
    prepared: Option<super::cache::LayoutState>,
    messages: gridthorn::localization::Localization,
    animations: Vec<UiTransition>,
}

impl Interface {
    pub(super) fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let tree = super::windows::tree()?;
        let mut router = UiRouter::new(30_000);
        for id in [200, 300, 400] {
            router.register_layer(&tree, UiNodeId(id))?;
        }
        let mut state = Self {
            performance: NativePerformance::new(),
            tree,
            router,
            fonts: composition::fonts()?,
            layout: None,
            prepared: None,
            frames: 0,
            metrics: ([1000.0, 800.0], 1.0),
            animations: Vec::new(),
            messages: super::messages::load()?,
        };
        super::messages::refresh(&mut state.tree, &mut state.messages)?;
        Ok(state)
    }

    pub(super) fn effect(
        &mut self,
        id: UiNodeId,
        effect: UiEffect,
    ) -> Result<Vec<gridthorn::ui::UiPlatformRequest>, Box<dyn std::error::Error>> {
        let mut requests = Vec::new();
        if effect == UiEffect::Activated {
            let target = match id.0 {
                3 => Some(200),
                201 => Some(300),
                203 => Some(400),
                _ => None,
            };
            if let Some(target) = target {
                self.animations.clear();
                for (panel, offset) in [
                    (200, [80.0, 100.0]),
                    (300, [150.0, 160.0]),
                    (400, [240.0, 200.0]),
                ] {
                    let mut style = self
                        .tree
                        .node(UiNodeId(panel))
                        .expect("layer")
                        .style
                        .clone();
                    style.offset = offset;
                    self.tree.set_style(UiNodeId(panel), style)?;
                }
                let size = self.metrics.0;
                let dpi = self.metrics.1;
                let layout = self.tree.layout(size, dpi, Some(&mut self.fonts))?;
                let opened = self.router.open_layer(
                    &mut self.tree,
                    &layout,
                    UiNodeId(target),
                    gridthorn::ui::UiLayer {
                        modal: target != 400,
                        dismiss_escape: true,
                        dismiss_outside: target == 400,
                    },
                )?;
                requests.extend(opened.platform);
                if matches!(
                    self.tree
                        .node(UiNodeId(4))
                        .expect("animation toggle")
                        .control,
                    gridthorn::ui::UiControl::Toggle { checked: true, .. }
                ) {
                    let end = self
                        .tree
                        .node(UiNodeId(target))
                        .expect("panel")
                        .style
                        .offset;
                    self.animations
                        .retain(|transition| !transition.is_finished());
                    let mut transition = UiTransition::new(
                        UiNodeId(target),
                        gridthorn::ui::UiProperty::Offset([end[0] + 40.0, end[1]]),
                        gridthorn::ui::UiProperty::Offset(end),
                        std::time::Duration::from_millis(350),
                        gridthorn::ui::UiEasing::SmoothStep,
                    )?;
                    transition.advance(&mut self.tree, std::time::Duration::ZERO)?;
                    self.animations.push(transition);
                }
            } else if matches!(id.0, 204 | 301 | 401 | 402) {
                if id.0 == 401 {
                    self.tree.command(
                        UiNodeId(7),
                        UiCommand::SetText("Привет! مرحبًا 日本語 e\u{301}".into()),
                    )?;
                }
                requests.extend(
                    self.router
                        .close_layer(&self.tree, self.layout.as_ref().expect("prepared"))
                        .platform,
                );
            }
        }
        if (effect == UiEffect::Changed && matches!(id.0, 5..=7))
            || (effect == UiEffect::Activated && id.0 == 401)
        {
            super::messages::refresh(&mut self.tree, &mut self.messages)?;
        }
        Ok(requests)
    }
    fn smoke_step(
        &mut self,
    ) -> Result<Vec<gridthorn::ui::UiPlatformRequest>, Box<dyn std::error::Error>> {
        let mut requests = Vec::new();

        if self.frames % 25 == 1 {
            let language = usize::try_from(self.frames / 25).unwrap_or(0) % 4;
            self.tree
                .command(UiNodeId(6), UiCommand::Select(Some(language)))?;
            requests.extend(self.effect(UiNodeId(6), UiEffect::Changed)?);
        }
        let action = match self.frames {
            10 | 100 => Some(3),
            25 => Some(201),
            40 => Some(301),
            55 => Some(203),
            70 => Some(401),
            85 => Some(204),

            _ => None,
        };
        if let Some(action) = action {
            requests.extend(self.effect(UiNodeId(action), UiEffect::Activated)?);
        }

        Ok(requests)
    }

    fn input_frame(
        &mut self,
        viewport: WindowViewport,
        dpi: f64,
        input: &InputState,
        workload: Option<super::SmokeWorkload>,
    ) -> Result<UiRoute, Box<dyn std::error::Error>> {
        let start = self.performance.start();
        self.prepare(viewport, dpi)?;
        self.performance.record(Phase::PrepareBefore, start);
        let start = self.performance.start();
        let mut route = self.input(input)?;
        self.performance.record(Phase::InputRoute, start);
        let start = self.performance.start();
        if workload == Some(super::SmokeWorkload::Mixed) {
            route.platform.extend(self.smoke_step()?);
        } else if workload == Some(super::SmokeWorkload::Editing) {
            let injected = self.editing_step(self.frames)?;
            route.platform.extend(injected.platform);
            route.effects.extend(injected.effects);
        } else if workload == Some(super::SmokeWorkload::Slider) {
            let injected = self.slider_step(self.frames, dpi)?;
            route.platform.extend(injected.platform);
            route.effects.extend(injected.effects);
        } else if workload == Some(super::SmokeWorkload::Locale) {
            let injected = self.locale_step(self.frames)?;
            route.effects.extend(injected.effects);
        } else if workload == Some(super::SmokeWorkload::Scroll) {
            let injected = self.scroll_step(self.frames, dpi)?;
            route.platform.extend(injected.platform);
            route.effects.extend(injected.effects);
        } else if matches!(
            workload,
            Some(super::SmokeWorkload::Windows | super::SmokeWorkload::Animation)
        ) {
            let injected = self.window_step(
                self.frames,
                workload == Some(super::SmokeWorkload::Animation),
            )?;
            route.effects.extend(injected.effects);
        }
        if matches!(
            workload,
            Some(super::SmokeWorkload::Selection | super::SmokeWorkload::Preedit)
        ) {
            let injected =
                self.editor_step(self.frames, workload == Some(super::SmokeWorkload::Preedit))?;
            route.platform.extend(injected.platform);
            route.effects.extend(injected.effects);
        }
        self.performance.record(Phase::Workload, start);
        let start = self.performance.start();
        for (id, effect) in route.effects.clone() {
            route.platform.extend(self.effect(id, effect)?);
        }
        self.performance.record(Phase::Effects, start);
        let start = self.performance.start();
        let changed = self.prepare(viewport, dpi)?;
        self.performance.record(Phase::PrepareAfter, start);
        let start = self.performance.start();
        if changed {
            let layout = self.layout.as_ref().expect("prepared layout");
            let refreshed = self.router.route_events(&mut self.tree, layout, &[])?;
            route.platform.extend(refreshed.platform);
        }
        self.performance.record(Phase::Anchor, start);
        self.performance.finish(self.frames);
        Ok::<_, Box<dyn std::error::Error>>(route)
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
    pub(super) fn prepare(
        &mut self,
        viewport: WindowViewport,
        dpi: f64,
    ) -> Result<bool, UiCompositionError> {
        self.metrics = (
            [
                viewport.width as f32 / dpi as f32,
                viewport.height as f32 / dpi as f32,
            ],
            dpi as f32,
        );
        let start = self.performance.start();
        let inputs = super::cache::LayoutState::capture(&self.tree, &self.router, self.metrics);
        self.performance.record(Phase::Snapshot, start);
        if self.prepared.as_ref() == Some(&inputs) {
            return Ok(false);
        }
        let start = self.performance.start();
        self.layout = Some(self.router.layout(
            &self.tree,
            [
                viewport.width as f32 / dpi as f32,
                viewport.height as f32 / dpi as f32,
            ],
            dpi as f32,
            Some(&mut self.fonts),
        )?);
        self.performance.record(Phase::Layout, start);
        self.prepared = Some(inputs);
        Ok(true)
    }
}

/// Compose and update presentation through public SDK APIs.
pub(crate) fn runtime(
    workload: Option<super::SmokeWorkload>,
    locale: usize,
    frame_limit: u32,
) -> Result<ApplicationRuntime, Box<dyn std::error::Error>> {
    let mut initial = Some(Interface::new()?);
    if let Some(state) = &mut initial {
        state
            .tree
            .command(UiNodeId(6), UiCommand::Select(Some(locale)))?;
        state.effect(UiNodeId(6), UiEffect::Changed)?;
    }
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
            state.performance.begin();
            if state.frames == 1 && workload.is_some() {
                eprintln!("workbench_workload,mode={workload:?},locale_index={locale},physical_width={},physical_height={},scale_factor={dpi},frame_limit={frame_limit}", viewport.width, viewport.height);
            }
            state.input_frame(viewport, dpi, &input, workload)
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
            || (workload.is_some()
                && world
                    .read_resource(|state: &Interface| state.frames >= frame_limit)
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

/// Exercise the same workbench tree and actions without native devices.
pub(crate) fn headless() -> Result<(), Box<dyn std::error::Error>> {
    super::validation::run()
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
        state
            .animations
            .retain(|transition| !transition.is_finished());
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
