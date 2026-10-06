use super::model::Settings;
use gridthorn::{GraphicsAdapterCompatibility, GraphicsBackend, GraphicsDevice, graphics_devices};

impl Settings {
    pub fn devices(&self) -> Vec<GraphicsDevice> {
        self.adapters
            .as_ref()
            .map_or_else(Vec::new, |inventory| graphics_devices(&inventory.adapters))
    }
    pub fn rendering_apis(&self) -> Vec<GraphicsBackend> {
        let mut apis = Vec::new();
        for device in self.devices().iter().filter(|device| {
            self.graphics_selection
                .device
                .as_ref()
                .is_none_or(|key| &device.key == key)
        }) {
            for adapter in &device.apis {
                if adapter.compatibility == GraphicsAdapterCompatibility::Compatible
                    && !apis.contains(&adapter.key.backend)
                {
                    apis.push(adapter.key.backend);
                }
            }
        }
        apis
    }
    pub fn choose_adapter(&mut self, row: Option<usize>) {
        let Some(row) = row else {
            return;
        };
        if row == 0 {
            self.graphics_selection.device = None;
        } else {
            let devices = self.devices();
            let Some(device) = devices.get(row - 1) else {
                return;
            };
            if !device
                .apis
                .iter()
                .any(|adapter| adapter.compatibility == GraphicsAdapterCompatibility::Compatible)
            {
                self.adapter_status = "Эта видеокарта несовместима с текущим окном.".into();
                return;
            }
            self.graphics_selection.device = Some(device.key.clone());
        }
        if self
            .graphics_selection
            .api
            .is_some_and(|api| !self.rendering_apis().contains(&api))
        {
            self.graphics_selection.api = None;
        }
        self.adapter_status = "Выбор ещё не сохранён. Текущий GPU продолжает работать.".into();
    }
    pub fn choose_api(&mut self, row: Option<usize>) {
        let Some(row) = row else {
            return;
        };
        let api = if row == 0 {
            None
        } else {
            self.rendering_apis().get(row - 1).copied()
        };
        if row != 0 && api.is_none() {
            return;
        }
        if let Some(key) = &self.graphics_selection.device {
            let devices = self.devices();
            let Some(device) = devices.iter().find(|device| &device.key == key) else {
                return;
            };
            if device
                .apis
                .iter()
                .filter(|adapter| api.is_none_or(|api| adapter.key.backend == api))
                .any(|adapter| {
                    device
                        .apis
                        .iter()
                        .filter(|candidate| candidate.key.backend == adapter.key.backend)
                        .count()
                        > 1
                })
            {
                self.adapter_status =
                    "Несколько одинаковых карт нельзя однозначно различить для этого API.".into();
                return;
            }
        }
        self.graphics_selection.api = api;
        self.adapter_status = "Выбор API ещё не сохранён. Изменение требует перезапуска.".into();
    }
    pub fn adapter_row(&self) -> Option<usize> {
        self.graphics_selection
            .device
            .as_ref()
            .map_or(Some(0), |key| {
                self.devices()
                    .iter()
                    .position(|device| &device.key == key)
                    .map(|index| index + 1)
            })
    }
    pub fn api_row(&self) -> Option<usize> {
        self.graphics_selection.api.map_or(Some(0), |api| {
            self.rendering_apis()
                .iter()
                .position(|candidate| *candidate == api)
                .map(|index| index + 1)
        })
    }
}
