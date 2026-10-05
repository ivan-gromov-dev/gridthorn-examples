use super::graphics::GraphicsSettings;
use gridthorn::display::{Displays, MonitorId, MonitorInfo};
use gridthorn::window::{WindowOperation, WindowRequest, WindowSettings};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Section {
    Graphics,
    Audio,
    Controls,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Request {
    Refresh,
    Configure(WindowRequest),
    Exit,
}

pub(super) struct Settings {
    pub section: Section,
    pub monitors: Vec<MonitorInfo>,
    pub selected: Option<MonitorId>,
    pub active: Option<MonitorId>,
    pub revision: u64,
    pub waiting: bool,
    pub applying: bool,
    pub graphics: GraphicsSettings,
    pub status: String,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            section: Section::Graphics,
            monitors: Vec::new(),
            selected: None,
            active: None,
            revision: 0,
            waiting: true,
            applying: false,
            graphics: GraphicsSettings::default(),
            status: "Получаем список мониторов…".into(),
        }
    }
}
impl Settings {
    pub fn needs_sync(&self, displays: &Displays) -> bool {
        self.revision != displays.revision()
    }
    pub fn sync(&mut self, displays: &Displays) {
        self.revision = displays.revision();
        self.monitors = displays.monitors().to_vec();
        self.active = displays.active();
        self.waiting = false;
        if !self
            .monitors
            .iter()
            .any(|monitor| Some(monitor.id) == self.selected)
        {
            self.selected = self
                .active
                .or_else(|| self.monitors.first().map(|monitor| monitor.id));
        }
        if !self.applying {
            self.status = if self.monitors.is_empty() {
                "ОС не сообщила о доступных мониторах.".into()
            } else {
                "Выберите параметры и нажмите «Применить».".into()
            };
        }
    }
    pub fn sync_window(&mut self, window: &WindowSettings) {
        let changed = self.graphics.feedback.as_ref() != window.feedback();
        self.graphics.sync(window);
        if let Some(actual) = self.graphics.actual {
            self.active = actual.monitor;
        }
        if changed {
            match &self.graphics.feedback {
                Some(WindowOperation::Pending { .. }) => {
                    self.applying = true;
                    self.status = "Применяем параметры окна…".into();
                }
                Some(WindowOperation::Applied { .. }) => {
                    self.applying = false;
                    self.status =
                        "Параметры окна применены. Состояние подтверждено платформой.".into();
                }
                Some(WindowOperation::Failed { error, .. }) => {
                    self.applying = false;
                    self.status = format!("Не удалось применить параметры: {error}");
                }
                None => {}
            }
        }
    }
    pub fn busy(&self) -> bool {
        self.waiting || self.applying
    }
    pub fn chosen(&self) -> Option<&MonitorInfo> {
        self.monitors
            .iter()
            .find(|monitor| Some(monitor.id) == self.selected)
    }
    pub fn choose(&mut self, index: Option<usize>) {
        if !self.busy() {
            self.selected =
                index.and_then(|index| self.monitors.get(index).map(|monitor| monitor.id));
            if let Some(monitor) = self.chosen() {
                self.graphics.exclusive = monitor
                    .modes
                    .iter()
                    .find(|mode| {
                        mode.resolution == monitor.resolution
                            && Some(mode.refresh_rate_millihertz) == monitor.refresh_rate_millihertz
                    })
                    .or_else(|| monitor.modes.first())
                    .copied();
                if self.graphics.mode == gridthorn::window::WindowModeKind::Exclusive
                    && let Some(mode) = self.graphics.exclusive
                {
                    self.graphics.size = mode.resolution;
                }
            }
        }
    }
    pub fn refresh(&mut self) -> Option<Request> {
        if self.busy() {
            return None;
        }
        self.waiting = true;
        self.status = "Обновляем список мониторов…".into();
        Some(Request::Refresh)
    }
    pub fn open(&mut self, section: Section) -> Option<Request> {
        let changed = self.section != section;
        self.section = section;
        if changed && section == Section::Graphics {
            self.refresh()
        } else {
            None
        }
    }
    pub fn apply(&mut self) -> Option<Request> {
        if self.busy() {
            return None;
        }
        let request = self.graphics.request(self.chosen()?);
        if request == WindowRequest::default() {
            self.status = "Для этого монитора нет выбранного эксклюзивного режима.".into();
            return None;
        }
        self.applying = true;
        self.status = "Применяем параметры окна…".into();
        Some(Request::Configure(request))
    }
    pub fn reset(&mut self) {
        if !self.busy() {
            self.selected = self.active;
            self.graphics.reset();
        }
    }
}
