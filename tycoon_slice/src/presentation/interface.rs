use crate::economy::model::{Kind, Role};
use crate::game::session::{Session, Tool, button_visible, layout_bounds};
use gridthorn::{Color, SimulationControl, TextLabel, UiPrimitive, WindowViewport};

fn text(value: impl Into<String>, position: [f32; 2], scale: f32, color: Color) -> UiPrimitive {
    TextLabel::new(value, position, scale, color)
        .expect("interface text")
        .into()
}
fn cream() -> Color {
    Color::rgb(0.95, 0.91, 0.8)
}
fn gold() -> Color {
    Color::rgb(1.0, 0.75, 0.35)
}
fn muted() -> Color {
    Color::rgb(0.65, 0.79, 0.78)
}

#[allow(clippy::cast_precision_loss)]
pub fn extract(
    s: &Session,
    active: &str,
    control: SimulationControl,
    viewport: WindowViewport,
    reloads: u64,
    error: Option<&str>,
) -> Vec<UiPrimitive> {
    let (scale, width, height) = crate::game::session::ui_layout(viewport);
    let mut ui = header(s, active, control, width);
    if active == "play" {
        selection(s, width, &mut ui);
        ui.push(text(
            "WASD: PAN  RIGHT CLICK: SELECT",
            [16.0, 674.0],
            1.0,
            muted(),
        ));
        ui.push(text("SPACE: PAUSE  ESC: MENU", [16.0, 694.0], 1.0, muted()));
    } else {
        let x = (width - 360.0) * 0.5;
        ui.push(text("HARBOR MENU", [x + 74.0, 160.0], 2.5, gold()));
        ui.push(text(
            "SIMULATION IS PAUSED",
            [x + 86.0, 202.0],
            1.0,
            muted(),
        ));
    }
    for index in 0..35 {
        if !button_visible(index, active, s) {
            continue;
        }
        let (p, _) = layout_bounds(index, viewport);
        let p = p.map(|value| value / scale);
        let label = label(index, s, active);
        ui.push(text(
            label,
            [p[0] + 10.0, p[1] + 10.0],
            1.0,
            if chosen(index, s, control) {
                gold()
            } else {
                cream()
            },
        ));
    }
    ui.push(text(
        s.data.message.chars().take(115).collect::<String>(),
        [18.0, height - 48.0],
        1.0,
        gold(),
    ));
    ui.push(text(
        s.notice.chars().take(115).collect::<String>(),
        [18.0, height - 28.0],
        1.0,
        muted(),
    ));
    if let Some(error) = error {
        ui.push(text(
            format!("ASSET ERROR: {error}"),
            [250.0, 124.0],
            1.0,
            gold(),
        ));
    }
    if s.paths && active == "play" {
        let search = s.diagnostic();
        ui.push(text(
            format!(
                "GRID SEARCH {:?} | RELOADS {reloads}",
                search.as_ref().map(|p| p.status)
            ),
            [250.0, 144.0],
            1.0,
            muted(),
        ));
    }
    ui.into_iter()
        .map(|primitive| match primitive {
            UiPrimitive::Text(label) => text(
                label.text(),
                label.position().map(|v| v * scale),
                label.pixel_scale() * scale,
                label.color(),
            ),
            other @ (UiPrimitive::Rect(_)
            | UiPrimitive::ShapedText(_)
            | UiPrimitive::Clipped { .. }) => other,
        })
        .collect()
}

#[allow(clippy::cast_precision_loss)]
fn selection(session: &Session, width: f32, ui: &mut Vec<UiPrimitive>) {
    let left = width - 276.0;
    let Some(id) = session
        .selected
        .filter(|id| session.data.buildings.contains_key(id))
    else {
        ui.push(text("SETTLEMENT", [left, 140.0], 1.5, gold()));
        for (i, line) in [
            "SELECT A BUILDING TO SEE STAFF",
            "LOGS: FOREST -> LOG WAREHOUSE",
            "PORTERS: WAREHOUSE -> MILL/PORT",
            "PLANKS: MILL -> PLANK WAREHOUSE",
            "EACH PORT SELLS ONE RESOURCE",
            "BUILD A PLANK PORT TO EXPAND",
        ]
        .iter()
        .enumerate()
        {
            ui.push(text(
                *line,
                [
                    left,
                    186.0 + f32::from(u16::try_from(i).expect("line")) * 28.0,
                ],
                1.0,
                muted(),
            ));
        }
        return;
    };
    let building = &session.data.buildings[&id];
    ui.push(text(
        format!("{} #{}", building.kind.name(), id.0),
        [left, 140.0],
        1.5,
        gold(),
    ));
    ui.push(text(
        format!("STOCK: {} LOGS / {} PLANKS", building.raw, building.planks),
        [left, 174.0],
        1.0,
        cream(),
    ));
    ui.push(text(
        if building.kind == Kind::Home {
            format!("RESIDENTS {}/4", session.staff().len())
        } else if building.kind == Kind::Port {
            format!("SELLS ONLY {}", building.export.name())
        } else {
            format!("ASSIGNED WORKERS {}", session.staff().len())
        },
        [left, 198.0],
        1.0,
        muted(),
    ));
    ui.push(text("WORKERS / HOME / ROUTE", [left, 232.0], 1.0, gold()));
    staff_rows(session, left, ui);
}

pub fn chosen(index: usize, s: &Session, control: SimulationControl) -> bool {
    match index {
        0 => s.tool == Tool::Road,
        1 => s.tool == Tool::Build(Kind::Sawmill),
        2 => s.tool == Tool::Build(Kind::RawStore),
        3 => s.tool == Tool::Build(Kind::Home),
        4 => s.tool == Tool::Move,
        5 => s.tool == Tool::Remove,
        22 => s.tool == Tool::Select,
        23 => s.tool == Tool::Build(Kind::Forest),
        24 => s.tool == Tool::Build(Kind::PlankStore),
        25 => s.tool == Tool::Build(Kind::Port),
        6 => control.is_paused(),
        7 => !control.is_paused() && control.speed().denominator() == 2,
        27..=29 => {
            !control.is_paused()
                && control.speed().numerator() == 1 << (index - 27)
                && control.speed().denominator() == 1
        }
        9 => s.paths,
        14 => s.chunks,
        34 => s.assigning.is_some(),
        _ => false,
    }
}

fn label(index: usize, s: &Session, active: &str) -> String {
    match index {
        0 => "ROAD              3 GOLD",
        1 => "SAWMILL         100 GOLD",
        2 => "LOG WAREHOUSE    60 GOLD",
        3 => "HOUSE / 4 BEDS   40 GOLD",
        4 => "MOVE              5 GOLD",
        5 => "DEMOLISH / 50% REFUND",
        6 => "PAUSE",
        7 => "0.5X",
        27 => "1X",
        28 => "2X",
        29 => "4X",
        8 => "TICK STEP",
        9 => "PATHS",
        10 => "SAVE GAME",
        11 => "LOAD GAME",
        12 => "CAPTURE SNAPSHOT",
        13 => "RESTORE SNAPSHOT",
        14 => "CHUNKS",
        15 => "ISO / GRID",
        16 => "ZOOM +",
        17 => "ZOOM -",
        18 => "NEW GAME",
        19 => "SANDBOX",
        20 => {
            if active == "play" {
                "MENU"
            } else {
                "RESUME"
            }
        }
        21 => "QUIT GAME",
        22 => "SELECT / INSPECT",
        23 => "FOREST           60 GOLD",
        24 => "PLANK WAREHOUSE  60 GOLD",
        25 => "PORT / COAST    180 GOLD",
        26 => "RESUME GAME",
        30 => {
            let role = s.selected.and_then(|id| s.data.buildings.get(&id)).map_or(
                Role::Porter,
                |b| match b.kind {
                    Kind::Forest => Role::Lumberjack,
                    Kind::Sawmill => Role::Carpenter,
                    _ => Role::Porter,
                },
            );
            return format!("HIRE {} / {} GOLD", role.name(), role.cost());
        }
        31 => {
            return if s
                .selected
                .and_then(|id| s.data.buildings.get(&id))
                .is_some_and(|b| b.export == crate::economy::model::Resource::Raw)
            {
                "CHANGE TO SELL PLANKS".into()
            } else {
                "CHANGE TO SELL LOGS".into()
            };
        }
        32 => "PREVIOUS WORKER",
        33 => "NEXT WORKER",
        34 => "ASSIGN ACTIVE WORKER",
        _ => "",
    }
    .into()
}

fn header(s: &Session, active: &str, control: SimulationControl, width: f32) -> Vec<UiPrimitive> {
    let planks: u64 = s.data.buildings.values().map(|b| b.planks).sum::<u64>()
        + u64::try_from(
            s.data
                .workers
                .values()
                .filter(|w| w.cargo == Some(crate::economy::model::Resource::Planks))
                .count(),
        )
        .expect("bounded workers");
    let seconds = s.ticks / 10;
    vec![
        text(format!("GOLD {}", s.data.coins), [154.0, 28.0], 1.5, gold()),
        text(
            format!("LOGS {}", s.data.timber),
            [326.0, 28.0],
            1.5,
            cream(),
        ),
        text(format!("PLANKS {planks}"), [490.0, 28.0], 1.5, cream()),
        text(
            format!(
                "BUILDINGS {}",
                s.data
                    .buildings
                    .values()
                    .filter(|b| b.kind != Kind::Forest)
                    .count()
            ),
            [665.0, 28.0],
            1.5,
            cream(),
        ),
        text(
            format!("STAFF {}/{}", s.data.workers.len(), s.data.homes() * 4),
            [840.0, 28.0],
            1.5,
            cream(),
        ),
        text(
            format!(
                "{:02}:{:02}:{:02}",
                seconds / 3600,
                seconds / 60 % 60,
                seconds % 60
            ),
            [width - 170.0, 28.0],
            1.5,
            gold(),
        ),
        text("TIMBER HARBOR", [360.0, 68.0], 2.0, gold()),
        text(
            format!(
                "SOLD {}   {}",
                s.data.shipped,
                if active == "play" && !control.is_paused() {
                    "RUNNING"
                } else {
                    "PAUSED"
                }
            ),
            [width - 310.0, 72.0],
            1.0,
            muted(),
        ),
        text("BUILD & EXPAND", [20.0, 130.0], 1.5, gold()),
    ]
}

fn staff_rows(session: &Session, left: f32, ui: &mut Vec<UiPrimitive>) {
    let staff = session.staff();
    let current = session.employee.or_else(|| staff.first().copied());
    let first = staff
        .iter()
        .position(|id| Some(*id) == current)
        .unwrap_or(0)
        .saturating_sub(3);
    for (i, id) in staff.iter().skip(first).take(6).enumerate() {
        let worker = &session.data.workers[id];
        let top = 262.0 + f32::from(u16::try_from(i).expect("worker row")) * 42.0;
        ui.push(text(
            format!(
                "{} #{} {}",
                if Some(*id) == current { ">" } else { " " },
                id,
                worker.role.name()
            ),
            [left, top],
            1.0,
            if Some(*id) == current {
                gold()
            } else {
                cream()
            },
        ));
        ui.push(text(
            format!(
                "HOME #{}  #{} -> #{}  {}",
                worker.home.0,
                worker.source.0,
                worker.destination.0,
                worker.status()
            ),
            [left, top + 16.0],
            0.85,
            muted(),
        ));
    }
    if staff.is_empty() {
        ui.push(text("NO WORKER ASSIGNED", [left, 262.0], 1.0, muted()));
    }
    if let Some(worker) = current.and_then(|id| session.data.workers.get(&id)) {
        ui.push(text(
            format!(
                "ACTIVE #{} {} | WORK {}",
                current.expect("active worker"),
                worker.role.name(),
                worker.remaining
            ),
            [left, 522.0],
            1.0,
            gold(),
        ));
    }
}
