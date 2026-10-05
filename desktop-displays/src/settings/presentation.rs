use super::{
    composition,
    model::{Request, Section, Settings},
};
use gridthorn::ui::{
    UiCompositionError, UiControl, UiEffect, UiLayout, UiNodeId, UiRoute, UiRouter, UiTree,
};
use gridthorn::{InputEvent, InputState, RenderFrame, TextSystem, WindowViewport};

pub(super) struct Menu {
    pub model: Settings,
    pub tree: UiTree,
    pub router: UiRouter,
    pub layout: Option<UiLayout>,
    fonts: TextSystem,
    metrics: Option<(WindowViewport, f64)>,
    pub dirty: bool,
    pub layouts: usize,
}

impl Menu {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let model = Settings::default();
        Ok(Self {
            tree: composition::tree(&model)?,
            model,
            router: UiRouter::new(50_000),
            layout: None,
            fonts: composition::fonts()?,
            metrics: None,
            dirty: true,
            layouts: 0,
        })
    }
    pub fn rebuild(&mut self) -> Result<(), UiCompositionError> {
        self.tree = composition::tree(&self.model)?;
        self.dirty = true;
        Ok(())
    }
    #[expect(
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        reason = "physical viewport dimensions enter validated UI metrics"
    )]
    pub fn prepare(
        &mut self,
        viewport: WindowViewport,
        dpi: f64,
    ) -> Result<Option<RenderFrame>, UiCompositionError> {
        if !self.dirty && self.metrics == Some((viewport, dpi)) {
            return Ok(None);
        }
        self.layout = Some(self.router.layout(
            &self.tree,
            [
                viewport.width as f32 / dpi as f32,
                viewport.height as f32 / dpi as f32,
            ],
            dpi as f32,
            Some(&mut self.fonts),
        )?);
        self.metrics = Some((viewport, dpi));
        self.dirty = false;
        self.layouts += 1;
        Ok(Some(
            RenderFrame::default().with_ui(
                self.layout
                    .as_ref()
                    .expect("prepared layout")
                    .primitives()
                    .to_vec(),
            ),
        ))
    }
    pub fn input(&mut self, input: &InputState) -> Result<Vec<Request>, UiCompositionError> {
        let Some(layout) = &self.layout else {
            return Ok(Vec::new());
        };
        let route = self.router.route(&mut self.tree, layout, input)?;
        self.dirty = true;
        self.effects(&route)
    }
    pub fn events(&mut self, events: &[InputEvent]) -> Result<Vec<Request>, UiCompositionError> {
        let Some(layout) = &self.layout else {
            return Ok(Vec::new());
        };
        let route = self.router.route_events(&mut self.tree, layout, events)?;
        self.dirty = true;
        self.effects(&route)
    }
    fn effects(&mut self, route: &UiRoute) -> Result<Vec<Request>, UiCompositionError> {
        let mut requests = Vec::new();
        let mut changed = false;
        for (id, effect) in &route.effects {
            let request = match (*id, *effect) {
                (composition::GRAPHICS, UiEffect::Activated) => {
                    changed = true;
                    self.model.open(Section::Graphics)
                }
                (composition::AUDIO, UiEffect::Activated) => {
                    changed = true;
                    self.model.open(Section::Audio)
                }
                (composition::CONTROLS, UiEffect::Activated) => {
                    changed = true;
                    self.model.open(Section::Controls)
                }
                (composition::REFRESH, UiEffect::Activated) => {
                    changed = true;
                    self.model.refresh()
                }
                (composition::APPLY, UiEffect::Activated) => {
                    changed = true;
                    self.model.apply()
                }
                (composition::RESET, UiEffect::Activated) => {
                    changed = true;
                    self.model.reset();
                    None
                }
                (composition::CLOSE, UiEffect::Activated) => Some(Request::Exit),
                (composition::MONITORS, UiEffect::Changed) => {
                    if let Some(node) = self.tree.node(composition::MONITORS)
                        && let UiControl::List { selected, .. } = node.control
                    {
                        self.model.choose(selected);
                        changed = true;
                    }
                    None
                }
                _ => {
                    changed |= self.graphics_effect(*id, *effect);
                    None
                }
            };
            if let Some(request) = request {
                requests.push(request);
            }
        }
        if changed {
            self.rebuild()?;
        }
        Ok(requests)
    }
    fn graphics_effect(&mut self, id: UiNodeId, effect: UiEffect) -> bool {
        match (id, effect) {
            (composition::MODE, UiEffect::Activated) => {
                let modes = self.model.graphics.modes();
                let index = modes
                    .iter()
                    .position(|mode| *mode == self.model.graphics.mode)
                    .unwrap_or(0);
                self.model.graphics.mode = modes[(index + 1) % modes.len()];
                if self.model.graphics.mode == gridthorn::window::WindowModeKind::Exclusive {
                    let selected = self
                        .model
                        .monitors
                        .iter()
                        .position(|monitor| Some(monitor.id) == self.model.selected);
                    self.model.choose(selected);
                    if let Some(mode) = self.model.graphics.exclusive {
                        self.model.graphics.size = mode.resolution;
                    }
                }
                return true;
            }
            (composition::SIZE, UiEffect::Activated) => {
                let sizes = self.model.graphics.resolutions(self.model.chosen());
                if !sizes.is_empty() {
                    let index = sizes
                        .iter()
                        .position(|size| *size == self.model.graphics.size)
                        .unwrap_or(0);
                    let monitor = self.model.chosen().cloned();
                    self.model
                        .graphics
                        .choose_resolution(sizes[(index + 1) % sizes.len()], monitor.as_ref());
                }
                return true;
            }
            (composition::RATE, UiEffect::Activated) => {
                let modes: Vec<_> = self
                    .model
                    .chosen()
                    .map(|monitor| {
                        monitor
                            .modes
                            .iter()
                            .filter(|mode| mode.resolution == self.model.graphics.size)
                            .copied()
                            .collect()
                    })
                    .unwrap_or_default();
                if !modes.is_empty() {
                    let index = modes
                        .iter()
                        .position(|mode| Some(*mode) == self.model.graphics.exclusive)
                        .unwrap_or(0);
                    self.model.graphics.exclusive = Some(modes[(index + 1) % modes.len()]);
                }
                return true;
            }
            (composition::RESIZABLE, UiEffect::Activated) => {
                self.model.graphics.resizable = !self.model.graphics.resizable;
                return true;
            }
            _ => {}
        }
        false
    }
    pub fn click(
        &mut self,
        id: UiNodeId,
        row: usize,
        dpi: f32,
    ) -> Result<Vec<Request>, UiCompositionError> {
        let bounds = self
            .layout
            .as_ref()
            .and_then(|layout| layout.placement(id))
            .expect("clickable control")
            .content;
        #[expect(
            clippy::cast_precision_loss,
            reason = "smoke row index is bounded by the monitor list"
        )]
        let row_offset = row as f32 * 34.0;
        self.events(&[
            InputEvent::CursorMoved(gridthorn::CursorPosition {
                x: f64::from((bounds.position[0] + 8.0) * dpi),
                y: f64::from((bounds.position[1] + 10.0 + row_offset) * dpi),
            }),
            InputEvent::MouseButton {
                button: gridthorn::MouseButton::Left,
                state: gridthorn::ButtonState::Pressed,
            },
            InputEvent::MouseButton {
                button: gridthorn::MouseButton::Left,
                state: gridthorn::ButtonState::Released,
            },
        ])
    }
}
