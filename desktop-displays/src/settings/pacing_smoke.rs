use super::{composition, model::Request, presentation::Menu};
use gridthorn::{
    WindowViewport,
    presentation::{FrameRateLimit, PresentMode, PresentationConfig, PresentationOperation},
    ui::{UiCommand, UiCompositionError, UiNodeId},
    window::WindowState,
};
use std::time::{Duration, Instant};

/// Finite routed acceptance of presentation settings without display mode changes.
pub(super) struct PacingSmoke {
    phase: u8,
    started: Instant,
    measured_at: Instant,
    first_tick: u64,
    revision: u64,
    window: Option<WindowState>,
    original: PresentationConfig,
    configs: Vec<PresentationConfig>,
    next: usize,
    applied_at: Option<Instant>,
}
impl Default for PacingSmoke {
    fn default() -> Self {
        Self {
            phase: 0,
            started: Instant::now(),
            measured_at: Instant::now(),
            first_tick: 0,
            revision: 0,
            window: None,
            original: PresentationConfig::default(),
            configs: Vec::new(),
            next: 0,
            applied_at: None,
        }
    }
}
impl PacingSmoke {
    pub fn advance(
        &mut self,
        menu: &mut Menu,
        viewport: WindowViewport,
        dpi: f64,
        timing: Option<(u64, Duration)>,
    ) -> Result<Vec<Request>, UiCompositionError> {
        if self.phase == 5 {
            return Ok(Vec::new());
        }
        assert!(
            self.started.elapsed() < Duration::from_secs(30),
            "presentation UI smoke timed out: {:?}",
            menu.model.pacing.feedback
        );
        if let Some(PresentationOperation::Failed { error, .. }) = &menu.model.pacing.feedback {
            panic!("presentation UI smoke failed: {error}");
        }
        if menu.model.busy() {
            return Ok(Vec::new());
        }
        let Some((ticks, fixed_step)) = timing else {
            return Ok(Vec::new());
        };
        match self.phase {
            0 => self.initialize(menu, viewport, dpi, ticks),
            1 => self.enable_vsync(menu, viewport, dpi),
            2 => self.apply_next(menu, viewport, dpi),
            3 => self.reset_choices(menu, viewport, dpi),
            4 => self.finish(menu, viewport, dpi, ticks, fixed_step),
            _ => unreachable!(),
        }
    }
    fn initialize(
        &mut self,
        menu: &mut Menu,
        viewport: WindowViewport,
        dpi: f64,
        ticks: u64,
    ) -> Result<Vec<Request>, UiCompositionError> {
        let pacing = &menu.model.pacing;
        assert!(pacing.available(), "presentation capabilities unavailable");
        self.window = menu.model.graphics.actual;
        self.revision = menu.model.revision;
        self.first_tick = ticks;
        self.measured_at = Instant::now();
        self.original = PresentationConfig {
            present_mode: pacing.state.applied_mode.unwrap_or_default(),
            frame_rate_limit: pacing.state.frame_rate_limit,
        };
        self.configs = pacing
            .state
            .supported_modes
            .iter()
            .flat_map(|mode| {
                [Some(1), Some(30), Some(90), None].map(|fps| PresentationConfig {
                    present_mode: *mode,
                    frame_rate_limit: fps.map(|fps| FrameRateLimit::new(fps).unwrap()),
                })
            })
            .collect();
        if pacing
            .state
            .supported_modes
            .contains(&PresentMode::Immediate)
        {
            self.phase = 1;
            assert_eq!(Self::click(menu, composition::VSYNC, viewport, dpi)?, []);
            assert_eq!(
                menu.model.pacing.config.present_mode,
                PresentMode::Immediate
            );
            Self::click(menu, composition::APPLY_PRESENTATION, viewport, dpi)
        } else {
            assert_eq!(Self::click(menu, composition::VSYNC, viewport, dpi)?, []);
            assert_eq!(menu.model.pacing.config, self.original);
            self.phase = 2;
            Ok(Vec::new())
        }
    }
    fn enable_vsync(
        &mut self,
        menu: &mut Menu,
        viewport: WindowViewport,
        dpi: f64,
    ) -> Result<Vec<Request>, UiCompositionError> {
        assert_eq!(
            menu.model.pacing.state.applied_mode,
            Some(PresentMode::Immediate)
        );
        self.phase = 2;
        assert_eq!(Self::click(menu, composition::VSYNC, viewport, dpi)?, []);
        assert_eq!(menu.model.pacing.config.present_mode, PresentMode::Fifo);
        Self::click(menu, composition::APPLY_PRESENTATION, viewport, dpi)
    }
    fn apply_next(
        &mut self,
        menu: &mut Menu,
        viewport: WindowViewport,
        dpi: f64,
    ) -> Result<Vec<Request>, UiCompositionError> {
        if self.next > 0 {
            let expected = self.configs[self.next - 1];
            assert!(
                matches!(menu.model.pacing.feedback, Some(PresentationOperation::Applied { config, .. }) if config == expected)
            );
            let applied = self.applied_at.get_or_insert_with(Instant::now);
            if applied.elapsed() < Duration::from_millis(300) {
                return Ok(Vec::new());
            }
            println!("presentation_ui_applied: {expected:?}");
            self.applied_at = None;
        }
        if let Some(config) = self.configs.get(self.next).copied() {
            self.next += 1;
            Self::stage(menu, config, viewport, dpi)?;
            Self::click(menu, composition::APPLY_PRESENTATION, viewport, dpi)
        } else {
            self.phase = 3;
            Self::stage(menu, self.original, viewport, dpi)?;
            Self::click(menu, composition::APPLY_PRESENTATION, viewport, dpi)
        }
    }
    fn reset_choices(
        &mut self,
        menu: &mut Menu,
        viewport: WindowViewport,
        dpi: f64,
    ) -> Result<Vec<Request>, UiCompositionError> {
        assert_eq!(
            menu.model.pacing.state.applied_mode,
            Some(self.original.present_mode)
        );
        assert_eq!(
            menu.model.pacing.state.frame_rate_limit,
            self.original.frame_rate_limit
        );
        assert_eq!(Self::click(menu, composition::FPS_CAP, viewport, dpi)?, []);
        assert_ne!(menu.model.pacing.config, self.original);
        assert_eq!(Self::click(menu, composition::RESET, viewport, dpi)?, []);
        assert_eq!(menu.model.pacing.config, self.original);
        self.phase = 4;
        Ok(Vec::new())
    }
    fn finish(
        &mut self,
        menu: &mut Menu,
        viewport: WindowViewport,
        dpi: f64,
        ticks: u64,
        fixed_step: Duration,
    ) -> Result<Vec<Request>, UiCompositionError> {
        assert_eq!(
            menu.model.graphics.actual, self.window,
            "presentation controls changed window/display state"
        );
        assert_eq!(
            menu.model.revision, self.revision,
            "presentation controls queried monitors again"
        );
        let expected = self.measured_at.elapsed().as_nanos() / fixed_step.as_nanos();
        let completed = ticks - self.first_tick;
        assert!(
            u128::from(completed) + 20 >= expected,
            "FPS cap stalled simulation: {completed} ticks, expected approximately {expected}"
        );
        println!(
            "presentation_ui_smoke: VSync, supported policies, 1/30/90 FPS, uncapped and reset passed; fixed_ticks={completed}, step={fixed_step:?}"
        );
        self.phase = 5;
        menu.prepare(viewport, dpi)?;
        menu.click(composition::CLOSE, 0, Self::dpi(dpi))
    }
    fn stage(
        menu: &mut Menu,
        config: PresentationConfig,
        viewport: WindowViewport,
        dpi: f64,
    ) -> Result<(), UiCompositionError> {
        let observed = menu.model.pacing.state.clone();
        for _ in 0..menu.model.pacing.state.supported_modes.len() {
            if menu.model.pacing.config.present_mode == config.present_mode {
                break;
            }
            assert_eq!(
                Self::click(menu, composition::PRESENT_MODE, viewport, dpi)?,
                []
            );
        }
        for _ in 0..6 {
            if menu.model.pacing.config.frame_rate_limit == config.frame_rate_limit {
                break;
            }
            assert_eq!(Self::click(menu, composition::FPS_CAP, viewport, dpi)?, []);
        }
        assert_eq!(menu.model.pacing.config, config);
        assert_eq!(
            menu.model.pacing.state, observed,
            "staging must not change observed native state"
        );
        Ok(())
    }
    fn click(
        menu: &mut Menu,
        id: UiNodeId,
        viewport: WindowViewport,
        dpi: f64,
    ) -> Result<Vec<Request>, UiCompositionError> {
        menu.prepare(viewport, dpi)?;
        let layout = menu.layout.as_ref().unwrap();
        let target = layout.placement(id).unwrap().content.position[1];
        let top = layout.placement(UiNodeId(5)).unwrap().content.position[1];
        let offset = menu.tree.node(UiNodeId(5)).unwrap().scroll_offset[1];
        menu.tree.command(
            UiNodeId(5),
            UiCommand::ScrollTo([0.0, (offset + target - top - 8.0).max(0.0)]),
        )?;
        menu.dirty = true;
        menu.prepare(viewport, dpi)?;
        menu.click(id, 0, Self::dpi(dpi))
    }
    #[expect(
        clippy::cast_possible_truncation,
        reason = "native DPI is bounded by UI validation"
    )]
    fn dpi(dpi: f64) -> f32 {
        dpi as f32
    }
}
