use super::Preferences;
use gridthorn::{
    GraphicsBackend, GraphicsSelection, enumerate_graphics_adapters, graphics_devices,
};

pub(crate) struct LaunchSelection {
    pub list_only: bool,
    pub graphics: GraphicsSelection,
    pub preferences: Preferences,
}

pub(crate) fn selection() -> Result<LaunchSelection, Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    let list_only = args.iter().any(|arg| arg == "--list-adapters");
    let index = args.iter().position(|arg| arg == "--adapter");
    let api_index = args.iter().position(|arg| arg == "--render-api");
    let mut preferences = Preferences::default();
    if let Some(index) = args.iter().position(|arg| arg == "--adapter-preference") {
        preferences.path = args
            .get(index + 1)
            .ok_or("--adapter-preference requires a file path")?
            .into();
    }
    let automatic = list_only
        || args
            .iter()
            .any(|arg| arg == "--auto-adapter" || arg == "--headless");
    let mut graphics = if automatic {
        GraphicsSelection::default()
    } else {
        preferences.load()?
    };
    if list_only
        || index.is_some()
        || graphics
            .device
            .as_ref()
            .is_some_and(|device| device.device == 0)
    {
        let devices = graphics_devices(&enumerate_graphics_adapters());
        for (index, device) in devices.iter().enumerate() {
            let apis: Vec<_> = device
                .apis
                .iter()
                .map(|adapter| adapter.key.backend)
                .collect();
            println!("{index}: {} · APIs={apis:?}", device.key.name);
        }
        if let Some(index) = index {
            let index: usize = args
                .get(index + 1)
                .ok_or("--adapter requires a device index")?
                .parse()?;
            graphics.device = Some(
                devices
                    .get(index)
                    .ok_or("device index is outside the current inventory")?
                    .key
                    .clone(),
            );
        } else if let Some(legacy) = &graphics.device {
            let matches: Vec<_> = devices
                .iter()
                .filter(|device| {
                    device.key.vendor == legacy.vendor && device.key.name == legacy.name
                })
                .collect();
            if matches.len() == 1 {
                graphics.device = Some(matches[0].key.clone());
            }
        }
    }
    if let Some(index) = api_index {
        graphics.api = match args
            .get(index + 1)
            .ok_or("--render-api requires an API name")?
            .as_str()
        {
            "auto" => None,
            "vulkan" => Some(GraphicsBackend::Vulkan),
            "dx12" => Some(GraphicsBackend::Direct3D12),
            "metal" => Some(GraphicsBackend::Metal),
            "opengl" => Some(GraphicsBackend::OpenGl),
            _ => return Err("unknown rendering API".into()),
        };
    }
    Ok(LaunchSelection {
        list_only,
        graphics,
        preferences,
    })
}
