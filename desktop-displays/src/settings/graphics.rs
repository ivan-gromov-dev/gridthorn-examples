use gridthorn::display::{DisplayMode, DisplayResolution, MonitorInfo};
use gridthorn::window::{
    WindowCapabilities, WindowMode, WindowModeKind, WindowOperation, WindowPlacement,
    WindowRequest, WindowResizePolicy, WindowSettings, WindowState,
};

pub(super) struct GraphicsSettings {
    pub mode: WindowModeKind,
    pub size: DisplayResolution,
    pub resizable: bool,
    pub exclusive: Option<DisplayMode>,
    pub capabilities: WindowCapabilities,
    pub actual: Option<WindowState>,
    pub feedback: Option<WindowOperation>,
}
impl Default for GraphicsSettings {
    fn default() -> Self {
        Self {
            mode: WindowModeKind::Windowed,
            size: DisplayResolution {
                width: 1100,
                height: 860,
            },
            resizable: true,
            exclusive: None,
            capabilities: WindowCapabilities::default(),
            actual: None,
            feedback: None,
        }
    }
}
impl GraphicsSettings {
    pub fn sync(&mut self, settings: &WindowSettings) {
        let initial = self.actual.is_none();
        self.capabilities = settings.capabilities();
        self.actual = settings.state();
        self.feedback = settings.feedback().cloned();
        if initial {
            self.reset();
        }
    }
    pub fn needs_sync(&self, settings: &WindowSettings) -> bool {
        self.actual != settings.state()
            || self.feedback.as_ref() != settings.feedback()
            || self.capabilities != settings.capabilities()
    }
    pub fn reset(&mut self) {
        if let Some(actual) = self.actual {
            self.mode = actual.mode;
            self.size = actual.size;
            self.resizable = !matches!(actual.resize_policy, WindowResizePolicy::Fixed);
            self.exclusive = actual.display_mode;
        }
    }
    pub fn modes(&self) -> Vec<WindowModeKind> {
        let mut modes = vec![WindowModeKind::Windowed];
        if self.capabilities.borderless {
            modes.push(WindowModeKind::Borderless);
        }
        if self.capabilities.exclusive {
            modes.push(WindowModeKind::Exclusive);
        }
        modes
    }
    pub fn resolutions(&self, monitor: Option<&MonitorInfo>) -> Vec<DisplayResolution> {
        let mut sizes: Vec<DisplayResolution> = if self.mode == WindowModeKind::Exclusive {
            monitor
                .map(|monitor| monitor.modes.iter().map(|mode| mode.resolution).collect())
                .unwrap_or_default()
        } else {
            [
                (960, 540),
                (1100, 860),
                (1280, 720),
                (1600, 900),
                (1920, 1080),
            ]
            .into_iter()
            .map(|(width, height)| DisplayResolution { width, height })
            .collect()
        };
        if self.mode == WindowModeKind::Windowed {
            sizes.push(self.size);
        }
        sizes.sort_unstable();
        sizes.dedup();
        sizes
    }
    pub fn choose_resolution(&mut self, size: DisplayResolution, monitor: Option<&MonitorInfo>) {
        self.size = size;
        if self.mode == WindowModeKind::Exclusive {
            self.exclusive = monitor.and_then(|monitor| {
                monitor
                    .modes
                    .iter()
                    .filter(|mode| mode.resolution == size)
                    .max_by_key(|mode| mode.refresh_rate_millihertz)
                    .copied()
            });
        }
    }
    pub fn request(&self, monitor: &MonitorInfo) -> WindowRequest {
        match self.mode {
            WindowModeKind::Windowed => WindowRequest {
                mode: Some(WindowMode::Windowed),
                size: Some(self.size),
                resize_policy: Some(if self.resizable {
                    WindowResizePolicy::Resizable {
                        min: Some(DisplayResolution {
                            width: 640,
                            height: 480,
                        }),
                        max: None,
                    }
                } else {
                    WindowResizePolicy::Fixed
                }),
                placement: self
                    .capabilities
                    .placement
                    .then_some(WindowPlacement::Centered {
                        monitor: monitor.id,
                    }),
            },
            WindowModeKind::Borderless => WindowRequest {
                mode: Some(WindowMode::Borderless {
                    monitor: monitor.id,
                }),
                ..WindowRequest::default()
            },
            WindowModeKind::Exclusive => WindowRequest {
                mode: self.exclusive.map(|mode| WindowMode::Exclusive {
                    monitor: monitor.id,
                    mode,
                }),
                ..WindowRequest::default()
            },
        }
    }
}
