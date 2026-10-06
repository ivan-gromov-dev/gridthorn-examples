use super::errors::PreferenceError;
use gridthorn::{GraphicsBackend, GraphicsDeviceKey, GraphicsSelection};
use std::{fs, io, path::PathBuf};

#[derive(Clone)]
pub(crate) struct Preferences {
    pub path: PathBuf,
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            path: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("adapter-preference.txt"),
        }
    }
}

fn api(value: &str) -> Result<Option<GraphicsBackend>, PreferenceError> {
    Ok(match value {
        "auto" => None,
        "Vulkan" => Some(GraphicsBackend::Vulkan),
        "Direct3D12" => Some(GraphicsBackend::Direct3D12),
        "Metal" => Some(GraphicsBackend::Metal),
        "OpenGl" => Some(GraphicsBackend::OpenGl),
        "Other" => Some(GraphicsBackend::Other),
        _ => return Err(PreferenceError::Invalid),
    })
}

impl Preferences {
    pub fn load(&self) -> Result<GraphicsSelection, PreferenceError> {
        let text = match fs::read_to_string(&self.path) {
            Ok(text) => text,
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                return Ok(GraphicsSelection::default());
            }
            Err(error) => return Err(error.into()),
        };
        let versioned = text.starts_with("v2\n");
        let mut fields = text.strip_prefix("v2\n").unwrap_or(&text).splitn(4, '\n');
        let api = api(fields.next().ok_or(PreferenceError::Invalid)?)?;
        let vendor = fields.next().ok_or(PreferenceError::Invalid)?;
        if versioned && vendor == "auto" {
            return Ok(GraphicsSelection { device: None, api });
        }
        let vendor = vendor.parse().map_err(|_| PreferenceError::Invalid)?;
        let device = fields
            .next()
            .and_then(|value| value.parse().ok())
            .ok_or(PreferenceError::Invalid)?;
        let name = fields.next().ok_or(PreferenceError::Invalid)?.to_owned();
        let name = if api == Some(GraphicsBackend::OpenGl) {
            name.strip_suffix("/PCIe/SSE2").unwrap_or(&name).into()
        } else {
            name
        };
        Ok(GraphicsSelection {
            device: Some(GraphicsDeviceKey {
                vendor,
                device,
                name,
            }),
            api,
        })
    }

    pub fn save(&self, selection: &GraphicsSelection) -> Result<(), PreferenceError> {
        if selection == &GraphicsSelection::default() {
            return match fs::remove_file(&self.path) {
                Ok(()) => Ok(()),
                Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
                Err(error) => Err(error.into()),
            };
        }
        let api = selection
            .api
            .map_or_else(|| "auto".into(), |api| format!("{api:?}"));
        let device = selection.device.as_ref().map_or_else(
            || "auto".into(),
            |device| format!("{}\n{}\n{}", device.vendor, device.device, device.name),
        );
        let temporary = self.path.with_extension("tmp");
        fs::write(&temporary, format!("v2\n{api}\n{device}"))?;
        fs::rename(&temporary, &self.path)?;
        Ok(())
    }
}
