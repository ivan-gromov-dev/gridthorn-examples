use super::{composition, model::Request, presentation::Menu};
use gridthorn::{
    WindowViewport,
    ui::UiCompositionError,
    window::{WindowModeKind, WindowOperation},
};
use std::time::{Duration, Instant};

pub(super) struct Smoke {
    phase: u8,
    started: Instant,
    revision: Option<u64>,
    desktop_rate: Option<u32>,
    target_rate: Option<u32>,
    desktop_resolution: Option<gridthorn::display::DisplayResolution>,
    target_resolution: Option<gridthorn::display::DisplayResolution>,
}
impl Default for Smoke {
    fn default() -> Self {
        Self {
            phase: 0,
            started: Instant::now(),
            revision: None,
            desktop_rate: None,
            target_rate: None,
            desktop_resolution: None,
            target_resolution: None,
        }
    }
}
impl Smoke {
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
            self.started.elapsed() < Duration::from_secs(45),
            "settings smoke timed out"
        );
        if let Some(WindowOperation::Failed { error, .. }) = &menu.model.graphics.feedback {
            panic!("settings smoke failed: {error}");
        }
        if menu.model.busy() {
            return Ok(Vec::new());
        }
        let requests = match self.phase {
            0 if menu.model.revision > 0 => self.start(menu, viewport, dpi)?,
            1 | 2 => {
                assert!(matches!(
                    menu.model.graphics.feedback,
                    Some(WindowOperation::Applied { .. })
                ));
                menu.click(composition::MODE, 0, dpi as f32)?;
                menu.prepare(viewport, dpi)?;
                self.phase += 1;
                menu.click(composition::APPLY, 0, dpi as f32)?
            }
            3 => self.change_rate(menu, viewport, dpi)?,
            4 => {
                let mode = menu.model.graphics.actual.unwrap().display_mode.unwrap();
                assert_eq!(Some(mode.resolution), self.target_resolution);
                self.phase = 5;
                menu.model.refresh().into_iter().collect()
            }
            5 => {
                assert_eq!(
                    menu.model.chosen().unwrap().refresh_rate_millihertz,
                    menu.model
                        .graphics
                        .actual
                        .unwrap()
                        .display_mode
                        .map(|mode| mode.refresh_rate_millihertz)
                );
                assert_eq!(
                    Some(menu.model.chosen().unwrap().resolution),
                    self.target_resolution
                );
                println!(
                    "settings_smoke: exclusive mode confirmed {:?} / {:?} -> {:?} / requested {:?}, reported {:?}",
                    self.desktop_resolution,
                    self.desktop_rate,
                    self.target_resolution,
                    self.target_rate,
                    menu.model.chosen().unwrap().refresh_rate_millihertz
                );
                self.restore(menu, viewport, dpi)?
            }
            6 => {
                assert_eq!(
                    menu.model.graphics.actual.unwrap().mode,
                    WindowModeKind::Windowed
                );
                assert!(matches!(
                    menu.model.graphics.feedback,
                    Some(WindowOperation::Applied { .. })
                ));
                self.phase = 7;
                menu.model.refresh().into_iter().collect()
            }
            7 => {
                assert_eq!(
                    menu.model.chosen().unwrap().refresh_rate_millihertz,
                    self.desktop_rate
                );
                assert_eq!(
                    Some(menu.model.chosen().unwrap().resolution),
                    self.desktop_resolution
                );
                self.revision = Some(menu.model.revision);
                self.started = Instant::now();
                self.phase = 8;
                Vec::new()
            }
            8 if self.started.elapsed() >= Duration::from_secs(2) => {
                assert_eq!(
                    Some(menu.model.revision),
                    self.revision,
                    "idle menu must not enumerate again"
                );
                println!(
                    "settings_smoke: supported fullscreen/refresh/desktop restore confirmed, query={}, layouts={}",
                    menu.model.revision, menu.layouts
                );
                self.phase = 9;
                menu.click(composition::CLOSE, 0, dpi as f32)?
            }
            _ => Vec::new(),
        };
        Ok(requests)
    }
    #[expect(
        clippy::cast_possible_truncation,
        reason = "native DPI is bounded by UI validation"
    )]
    fn start(
        &mut self,
        menu: &mut Menu,
        viewport: WindowViewport,
        dpi: f64,
    ) -> Result<Vec<Request>, UiCompositionError> {
        let row = menu
            .model
            .monitors
            .iter()
            .position(|monitor| Some(monitor.id) == menu.model.active)
            .unwrap_or(0);
        menu.click(composition::MONITORS, row, dpi as f32)?;
        self.desktop_rate = menu.model.chosen().unwrap().refresh_rate_millihertz;
        self.desktop_resolution = Some(menu.model.chosen().unwrap().resolution);
        menu.prepare(viewport, dpi)?;
        menu.click(composition::RESIZABLE, 0, dpi as f32)?;
        menu.prepare(viewport, dpi)?;
        self.phase = 1;
        menu.click(composition::APPLY, 0, dpi as f32)
    }
    #[expect(
        clippy::cast_possible_truncation,
        reason = "native DPI is bounded by UI validation"
    )]
    fn change_rate(
        &mut self,
        menu: &mut Menu,
        viewport: WindowViewport,
        dpi: f64,
    ) -> Result<Vec<Request>, UiCompositionError> {
        if menu.model.graphics.mode != WindowModeKind::Exclusive {
            return self.restore(menu, viewport, dpi);
        }
        let current = menu.model.graphics.exclusive.unwrap();
        let modes = &menu.model.chosen().unwrap().modes;
        let target = modes
            .iter()
            .find(|mode| {
                mode.resolution.width == 1600
                    && mode.resolution.height == 900
                    && mode.bit_depth == current.bit_depth
                    && mode.refresh_rate_millihertz == 60_000
            })
            .or_else(|| {
                modes.iter().find(|mode| {
                    mode.resolution.width == 1280
                        && mode.resolution.height == 720
                        && mode.bit_depth == current.bit_depth
                        && mode.refresh_rate_millihertz == 60_000
                })
            })
            .or_else(|| {
                modes.iter().find(|mode| {
                    mode.resolution == current.resolution
                        && mode.bit_depth == current.bit_depth
                        && mode.refresh_rate_millihertz == 60_000
                        && mode.refresh_rate_millihertz != current.refresh_rate_millihertz
                })
            })
            .or_else(|| {
                modes.iter().find(|mode| {
                    mode.resolution == current.resolution
                        && mode.bit_depth == current.bit_depth
                        && mode.refresh_rate_millihertz > 0
                        && mode.refresh_rate_millihertz != current.refresh_rate_millihertz
                })
            })
            .copied();
        let Some(target) = target else {
            return self.restore(menu, viewport, dpi);
        };
        let count = modes.len();
        let sizes = menu.model.graphics.resolutions(menu.model.chosen());
        for _ in 0..sizes.len() {
            if menu.model.graphics.size == target.resolution {
                break;
            }
            menu.prepare(viewport, dpi)?;
            menu.click(composition::SIZE, 0, dpi as f32)?;
        }
        assert_eq!(menu.model.graphics.size, target.resolution);
        for _ in 0..count {
            if menu.model.graphics.exclusive == Some(target) {
                break;
            }
            menu.prepare(viewport, dpi)?;
            menu.click(composition::RATE, 0, dpi as f32)?;
        }
        assert_eq!(menu.model.graphics.exclusive, Some(target));
        self.target_rate = Some(target.refresh_rate_millihertz);
        self.target_resolution = Some(target.resolution);
        self.phase = 4;
        menu.prepare(viewport, dpi)?;
        menu.click(composition::APPLY, 0, dpi as f32)
    }
    #[expect(
        clippy::cast_possible_truncation,
        reason = "native DPI is bounded by UI validation"
    )]
    fn restore(
        &mut self,
        menu: &mut Menu,
        viewport: WindowViewport,
        dpi: f64,
    ) -> Result<Vec<Request>, UiCompositionError> {
        menu.model.graphics.mode = WindowModeKind::Windowed;
        menu.model.graphics.size = gridthorn::display::DisplayResolution {
            width: 1100,
            height: 860,
        };
        menu.model.graphics.resizable = true;
        menu.rebuild()?;
        menu.prepare(viewport, dpi)?;
        self.phase = 6;
        menu.click(composition::APPLY, 0, dpi as f32)
    }
}
