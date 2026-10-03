use crate::economy::{
    Root, Runner,
    codec::HarborCodec,
    model::{Command, Harbor, Kind},
    runner,
};
use gridthorn::grid::{GridCell, GridObjectId, GridPoint, GridProjection};
use gridthorn::{GameStateId, SceneId, SimulationSnapshot, UiButton};
use std::path::PathBuf;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Tool {
    Select,
    Build(Kind),
    Road,
    Remove,
    Move,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum UiMode {
    Menu,
    Play,
}

pub(crate) struct Session {
    pub runner: Runner,
    pub data: Harbor,
    pub tool: Tool,
    pub selected: Option<GridObjectId>,
    pub employee: Option<u64>,
    pub assigning: Option<(u64, Option<GridObjectId>)>,
    pub resume_control: gridthorn::SimulationControl,
    ui_mode: UiMode,
    pub viewport: gridthorn::WindowViewport,
    pub moving: Option<GridObjectId>,
    pub hover: Option<GridCell>,
    pub map_press: Option<GridCell>,
    pub camera: [f32; 2],
    pub height: f32,
    pub square: bool,
    pub paths: bool,
    pub chunks: bool,
    pub frames: u64,
    pub buttons: Vec<UiButton>,
    pub hovered: Vec<bool>,
    pub snapshot: Option<SimulationSnapshot<Harbor, Command>>,
    pub view_snapshot: Option<String>,
    pub notice: String,
    pub save_path: PathBuf,
    pub ticks: u64,
}

impl Session {
    pub fn new(name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut runner = runner(name, 42)?;
        let data = runner.snapshot()?.state().data.clone();
        Ok(Self {
            runner,
            data,
            tool: Tool::Select,
            selected: None,
            employee: None,
            assigning: None,
            resume_control: gridthorn::SimulationControl::default(),
            ui_mode: UiMode::Menu,
            viewport: gridthorn::WindowViewport {
                width: 1280,
                height: 800,
            },
            moving: None,
            hover: None,
            map_press: None,
            camera: [25.0, -20.0],
            height: 720.0,
            square: false,
            paths: false,
            chunks: false,
            frames: 0,
            buttons: Vec::new(),
            hovered: vec![false; 35],
            snapshot: None,
            view_snapshot: None,
            notice: "SELECT A BUILDING TO INSPECT STAFF AND STOCK".into(),
            save_path: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("harbor-save.toml"),
            ticks: 0,
        })
    }

    pub fn projection(&self) -> GridProjection {
        if self.square {
            GridProjection::square(46.0, GridPoint::new(-46.0, -46.0))
        } else {
            GridProjection::isometric(64.0, 32.0, GridPoint::new(0.0, -32.0))
        }
        .expect("projection")
    }

    pub fn refresh(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let snapshot = self.runner.snapshot()?;
        self.ticks = snapshot.completed_ticks();
        self.data = snapshot.state().data.clone();
        self.selected = self
            .selected
            .filter(|id| self.data.buildings.contains_key(id));
        self.employee = self
            .employee
            .filter(|id| self.data.workers.contains_key(id));
        Ok(())
    }

    pub fn diagnostic(&self) -> Option<gridthorn::grid::PathSearch> {
        let start = self
            .hover
            .and_then(|cell| {
                if self.data.roads.contains(&cell) {
                    Some(cell)
                } else {
                    self.data
                        .building_route(cell)
                        .and_then(|path| path.first().copied())
                }
            })
            .unwrap_or(GridCell::new(-4, 0));
        self.data
            .diagnostic(start, if self.chunks { 8 } else { 144 })
    }

    pub fn advance(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.runner.run_ticks(1)?;
        self.refresh()
    }

    pub fn queue(&mut self, command: Command) {
        self.runner.world().update_resource(|root: &mut Root| {
            if root.commands.len() < 256 {
                root.commands.push(command);
            }
        });
    }

    pub fn save(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.runner.save_file(&self.save_path, &HarborCodec)?;
        self.notice = "WORLD SAVED TO HARBOR-SAVE.TOML".into();
        Ok(())
    }
    pub fn load(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let before = self.runner.snapshot()?;
        self.runner.load_file(&self.save_path, &HarborCodec)?;
        let restored = self.runner.snapshot()?;
        if !restored
            .state()
            .random
            .states()
            .any(|(name, _)| name == "market")
        {
            self.runner.restore(&before)?;
            return Err("save is missing the required market RNG stream".into());
        }
        self.refresh()?;
        self.notice = "SAVE LOADED - RNG AND CARGO RESTORED".into();
        Ok(())
    }
}

#[cfg(test)]
pub fn button_bounds(index: usize) -> ([f32; 2], [f32; 2]) {
    layout_bounds(
        index,
        gridthorn::WindowViewport {
            width: 1280,
            height: 800,
        },
    )
}

#[allow(clippy::cast_precision_loss)]
pub fn layout_bounds(index: usize, viewport: gridthorn::WindowViewport) -> ([f32; 2], [f32; 2]) {
    let (scale, width, _) = ui_layout(viewport);
    let menu_x = (width - 360.0) * 0.5;
    let side_x = width - 292.0;
    let row = |value: u16| 160.0 + f32::from(value) * 38.0;
    let (position, size) = match index {
        26 => ([menu_x + 24.0, 236.0], [312.0, 34.0]),
        20 => ([16.0, 16.0], [94.0, 34.0]),
        6 | 7 | 27..=29 => {
            let slot: u16 = match index {
                6 => 0,
                7 => 1,
                27 => 2,
                28 => 3,
                _ => 4,
            };
            ([16.0 + f32::from(slot) * 62.0, 62.0], [56.0, 28.0])
        }
        0..=5 => (
            [16.0, row(u16::try_from(index + 1).expect("tool"))],
            [204.0, 32.0],
        ),
        22 => ([16.0, row(0)], [204.0, 32.0]),
        23..=25 => (
            [16.0, row(u16::try_from(index - 16).expect("tool"))],
            [204.0, 32.0],
        ),
        8 | 9 | 14..=17 => {
            let slot: u16 = match index {
                8 => 0,
                9 => 1,
                14 => 2,
                15 => 3,
                16 => 4,
                _ => 5,
            };
            (
                [
                    16.0 + f32::from(slot % 2) * 106.0,
                    552.0 + f32::from(slot / 2) * 34.0,
                ],
                [98.0, 28.0],
            )
        }
        10..=13 | 18..=19 | 21 => {
            let slot: u16 = match index {
                10 => 0,
                11 => 1,
                12 => 2,
                13 => 3,
                18 => 4,
                19 => 5,
                _ => 6,
            };
            (
                [menu_x + 24.0, 278.0 + f32::from(slot) * 42.0],
                [312.0, 34.0],
            )
        }
        30..=34 => (
            [
                side_x + 16.0,
                548.0 + f32::from(u16::try_from(index - 30).expect("worker button")) * 36.0,
            ],
            [256.0, 30.0],
        ),
        _ => ([-1000.0, -1000.0], [1.0, 1.0]),
    };
    (
        position.map(|value| value * scale),
        size.map(|value| value * scale),
    )
}

pub fn button_visible(index: usize, active: &str, s: &Session) -> bool {
    if index == 20 {
        return true;
    }
    if active != "play" {
        return matches!(index, 10..=13 | 18..=19 | 21 | 26);
    }
    match index {
        10..=13 | 18..=19 | 21 | 26 => false,
        30 => s
            .selected
            .and_then(|id| s.data.buildings.get(&id))
            .is_some_and(|b| {
                matches!(
                    b.kind,
                    Kind::Forest | Kind::Sawmill | Kind::RawStore | Kind::PlankStore
                )
            }),
        31 => s
            .selected
            .and_then(|id| s.data.buildings.get(&id))
            .is_some_and(|b| b.kind == Kind::Port),
        32..=34 => !s.staff().is_empty(),
        _ => true,
    }
}

impl Session {
    pub fn layout(&mut self, viewport: gridthorn::WindowViewport, menu: bool) {
        let mode = if menu { UiMode::Menu } else { UiMode::Play };
        if self.buttons.is_empty() || self.viewport != viewport || self.ui_mode != mode {
            self.ui_mode = mode;
            self.viewport = viewport;
            self.buttons = (0..35)
                .map(|index| {
                    let (p, size) = layout_bounds(index, viewport);
                    UiButton::new(p, size).expect("button")
                })
                .collect();
            self.hovered.fill(false);
        }
    }
    pub fn staff(&self) -> Vec<u64> {
        self.data
            .workers
            .iter()
            .filter(|(_, w)| {
                self.selected
                    .is_some_and(|id| [w.home, w.source, w.destination].contains(&id))
            })
            .map(|(id, _)| *id)
            .collect()
    }
}

pub fn state(name: &str) -> GameStateId {
    GameStateId::new(name).expect("game state")
}
pub fn scene(name: &str) -> SceneId {
    SceneId::new(name).expect("game scene")
}

#[allow(clippy::cast_precision_loss)]
pub fn ui_layout(viewport: gridthorn::WindowViewport) -> (f32, f32, f32) {
    let width = viewport.width.max(1) as f32;
    let height = viewport.height.max(1) as f32;
    let scale = (width / 1280.0).min(height / 800.0);
    (scale, width / scale, height / scale)
}
