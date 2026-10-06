use super::{composition, model::Request, presentation::Menu};
use gridthorn::{
    GraphicsAdapterKey, GraphicsSelection, WindowViewport,
    ui::{UiCommand, UiCompositionError, UiNodeId},
};
use std::time::{Duration, Instant};

pub(super) struct AdapterSmoke {
    phase: u8,
    started: Instant,
    chosen: GraphicsSelection,
    effective: Option<GraphicsAdapterKey>,
    preferences: crate::adapters::Preferences,
}
impl AdapterSmoke {
    pub fn new(preferences: crate::adapters::Preferences) -> Self {
        Self {
            phase: 0,
            started: Instant::now(),
            chosen: GraphicsSelection::default(),
            effective: None,
            preferences,
        }
    }
    #[expect(
        clippy::cast_possible_truncation,
        reason = "native DPI is bounded by UI validation"
    )]
    pub fn advance(
        &mut self,
        menu: &mut Menu,
        viewport: WindowViewport,
        dpi: f64,
    ) -> Result<Vec<Request>, UiCompositionError> {
        assert!(
            self.started.elapsed() < Duration::from_secs(15),
            "adapter UI smoke timed out"
        );
        let requests = match self.phase {
            0 => {
                self.effective = Some(
                    menu.model
                        .adapters
                        .as_ref()
                        .expect("GPU inventory")
                        .selected
                        .clone(),
                );
                menu.tree
                    .command(UiNodeId(5), UiCommand::ScrollTo([0.0, 5000.0]))?;
                menu.dirty = true;
                Vec::new()
            }
            1 => {
                let devices = menu.model.devices();
                let index = devices
                    .iter()
                    .position(|device| {
                        device
                            .apis
                            .iter()
                            .any(|api| Some(&api.key) == self.effective.as_ref())
                    })
                    .unwrap();
                let requests = menu.click(composition::ADAPTERS, index + 1, dpi as f32)?;
                assert_eq!(
                    menu.model.graphics_selection.device.as_ref(),
                    Some(&devices[index].key)
                );
                requests
            }
            2 => {
                menu.prepare(viewport, dpi)?;
                let apis = menu.model.rendering_apis();
                let index = apis
                    .iter()
                    .position(|api| *api != self.effective.as_ref().unwrap().backend)
                    .unwrap_or(0);
                let requests = menu.click(composition::RENDER_API, index + 1, dpi as f32)?;
                assert_eq!(menu.model.graphics_selection.api, Some(apis[index]));
                self.chosen.clone_from(&menu.model.graphics_selection);
                requests
            }
            3 => {
                menu.prepare(viewport, dpi)?;
                let requests = menu.click(composition::SAVE_ADAPTER, 0, dpi as f32)?;
                assert_eq!(requests, [Request::SaveAdapter]);
                requests
            }
            4 => {
                assert_eq!(self.preferences.load().unwrap(), self.chosen);
                assert_eq!(
                    Some(&menu.model.adapters.as_ref().unwrap().selected),
                    self.effective.as_ref()
                );
                assert!(menu.model.adapter_status.contains("сохранён"));
                println!(
                    "adapter_ui_smoke: saved {:?}; effective GPU remains {:?}",
                    self.chosen, self.effective
                );
                vec![Request::Exit]
            }
            _ => Vec::new(),
        };
        self.phase += 1;
        Ok(requests)
    }
}
