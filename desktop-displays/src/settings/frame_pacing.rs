use super::model::Request;
use gridthorn::presentation::{
    FrameRateLimit, PresentMode, PresentationConfig, PresentationOperation, PresentationSettings,
    PresentationState,
};

/// Game-owned staged choices and independently observed surface feedback.
pub(super) struct FramePacing {
    pub config: PresentationConfig,
    pub state: PresentationState,
    pub feedback: Option<PresentationOperation>,
    pub applying: bool,
    pub status: String,
    initialized: bool,
}
impl Default for FramePacing {
    fn default() -> Self {
        Self {
            config: PresentationConfig::default(),
            state: PresentationState::default(),
            feedback: None,
            applying: false,
            initialized: false,
            status: "Показ кадров недоступен без renderer.".into(),
        }
    }
}
impl FramePacing {
    pub fn available(&self) -> bool {
        !self.state.supported_modes.is_empty()
    }
    pub fn needs_sync(&self, settings: &PresentationSettings) -> bool {
        self.state != *settings.state() || self.feedback.as_ref() != settings.feedback()
    }
    pub fn sync(&mut self, settings: &PresentationSettings) {
        self.observe(settings.state().clone(), settings.feedback().cloned());
    }
    pub fn observe(&mut self, state: PresentationState, feedback: Option<PresentationOperation>) {
        self.state = state;
        self.feedback = feedback;
        if !self.initialized && self.available() {
            self.reset();
            self.initialized = true;
        }
        self.applying = matches!(self.feedback, Some(PresentationOperation::Pending { .. }));
        self.status = match &self.feedback {
            Some(PresentationOperation::Pending { .. }) => {
                "Ожидаем применения режима показа кадров…".into()
            }
            Some(PresentationOperation::Applied { .. }) => {
                "Режим показа и лимит FPS применены.".into()
            }
            Some(PresentationOperation::Failed { error, .. }) => {
                format!("Не удалось применить показ кадров: {error}")
            }
            None if self.available() => {
                "Выберите VSync и лимит FPS, затем примените параметры.".into()
            }
            None => "Показ кадров недоступен без renderer.".into(),
        };
    }
    pub fn reset(&mut self) {
        if let Some(mode) = self.state.applied_mode {
            self.config.present_mode = mode;
        }
        self.config.frame_rate_limit = self.state.frame_rate_limit;
    }
    pub fn toggle_target(&self) -> PresentMode {
        if self.config.present_mode == PresentMode::Immediate {
            PresentMode::Fifo
        } else {
            PresentMode::Immediate
        }
    }
    pub fn can_toggle(&self) -> bool {
        self.state.supported_modes.contains(&self.toggle_target())
    }
    pub fn toggle_vsync(&mut self) -> bool {
        if !self.can_toggle() {
            return false;
        }
        self.config.present_mode = self.toggle_target();
        true
    }
    pub fn cycle_mode(&mut self) -> bool {
        let modes = &self.state.supported_modes;
        if modes.len() < 2 {
            return false;
        }
        let index = modes
            .iter()
            .position(|mode| *mode == self.config.present_mode)
            .unwrap_or(0);
        self.config.present_mode = modes[(index + 1) % modes.len()];
        true
    }
    pub fn cycle_cap(&mut self) {
        let caps = [None, Some(30), Some(60), Some(90), Some(144), Some(1)];
        let index = caps
            .iter()
            .position(|cap| *cap == self.config.frame_rate_limit.map(FrameRateLimit::fps))
            .unwrap_or(0);
        self.config.frame_rate_limit =
            caps[(index + 1) % caps.len()].map(|fps| FrameRateLimit::new(fps).unwrap());
    }
    pub fn request(&mut self) -> Option<Request> {
        if !self.available() || self.applying {
            return None;
        }
        self.applying = true;
        self.status = "Применяем VSync и лимит FPS…".into();
        Some(Request::Present(self.config))
    }
}

pub(super) fn mode_caption(mode: PresentMode) -> &'static str {
    match mode {
        PresentMode::Fifo => "FIFO — VSync включён",
        PresentMode::FifoRelaxed => "Adaptive FIFO — поздние кадры могут разрываться",
        PresentMode::Immediate => "Immediate — VSync выключен",
        PresentMode::Mailbox => "Mailbox — последний кадр на вертикальном обновлении",
    }
}
pub(super) fn cap_caption(cap: Option<FrameRateLimit>) -> String {
    cap.map_or_else(
        || "без ограничения".into(),
        |cap| format!("{} FPS", cap.fps()),
    )
}
