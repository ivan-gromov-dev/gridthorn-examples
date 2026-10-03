use super::session::{Session, Tool, button_visible, scene, state};
use crate::economy::model::Command;
use gridthorn::grid::{GridPoint, GridView};
use gridthorn::prelude::*;
use gridthorn::{SimulationControl, WindowViewport};

pub fn register(schedules: &mut ScheduleBuilder, smoke: bool) {
    schedules.add_system(ScheduleStage::Input, move |world| {
        let input = world
            .read_resource(Clone::clone)
            .unwrap_or_else(InputState::default);
        let viewport = world
            .read_resource(|v: &WindowViewport| *v)
            .expect("viewport");
        let active = world
            .read_resource(|s: &GameStateStack| s.current().as_str().to_owned())
            .expect("state");
        let mut control = world
            .read_resource(|c: &SimulationControl| *c)
            .unwrap_or_default();
        let mut next = None;
        let mut exit = false;
        world.update_resource(|session: &mut Session| {
            session.layout(viewport, active != "play");
            let mut clicked = None;
            let visible: Vec<_> = (0..35)
                .map(|index| button_visible(index, &active, session))
                .collect();
            for (index, button) in session.buttons.iter_mut().enumerate() {
                if !visible[index] {
                    session.hovered[index] = false;
                    continue;
                }
                let interaction = button.update(&input);
                session.hovered[index] = interaction.hovered();
                if interaction.activated() {
                    clicked = Some(index);
                }
            }
            if (input.key_just_pressed(KeyCode::Enter) && active != "play")
                || input.key_just_pressed(KeyCode::Escape)
                || (smoke && session.frames == 0)
            {
                clicked = Some(20);
            }
            if input.key_just_pressed(KeyCode::Space) && active == "play" {
                clicked = Some(6);
            }
            if active == "play" {
                session.runner.world().insert_resource(control);
            }
            if let Some(index) = clicked {
                if index == 21 {
                    exit = true;
                }
                let result = super::controls::activate(session, index, &active, &mut control);
                if let Ok(change) = &result {
                    next = *change;
                }
                if let Err(error) = result {
                    session.notice = format!("ERROR: {error}");
                    eprintln!("{}", session.notice);
                }
            }
            if active == "play" {
                camera_and_hover(session, &input, viewport);
            } else {
                session.hover = None;
            }
            map_commands(session, &input, &active, clicked.is_some());
            if active == "play" && next != Some("menu") {
                session.runner.world().insert_resource(control);
            }
        });
        world.insert_resource(control);
        if let Some(next) = next {
            world.update_resource(|s: &mut GameStateStack| s.request_set(state(next)));
            world.update_resource(|s: &mut SceneController| s.request_switch(scene(next)));
        }
        if exit {
            world.update_resource(|e: &mut ExitRequest| e.request());
        }
    });
}

fn camera_and_hover(session: &mut Session, input: &InputState, viewport: WindowViewport) {
    let movement = [
        i16::from(input.key_down(KeyCode::KeyD) || input.key_down(KeyCode::ArrowRight))
            - i16::from(input.key_down(KeyCode::KeyA) || input.key_down(KeyCode::ArrowLeft)),
        i16::from(input.key_down(KeyCode::KeyS) || input.key_down(KeyCode::ArrowDown))
            - i16::from(input.key_down(KeyCode::KeyW) || input.key_down(KeyCode::ArrowUp)),
    ];
    for (axis, movement) in movement.into_iter().enumerate() {
        session.camera[axis] =
            (session.camera[axis] + f32::from(movement) * 5.0).clamp(-600.0, 600.0);
    }
    session.hover = input.cursor_position().and_then(|cursor| {
        if !canvas_contains(cursor, viewport) {
            return None;
        }
        let view = GridView::new(
            GridPoint::new(f64::from(session.camera[0]), f64::from(session.camera[1])),
            f64::from(session.height),
            GridPoint::new(f64::from(viewport.width), f64::from(viewport.height)),
        )
        .ok()?;
        if matches!(session.tool, Tool::Select | Tool::Move) {
            let world_point = view
                .screen_to_world(GridPoint::new(cursor.x, cursor.y))
                .ok()??;
            if let Some(cell) = building_hit(session, world_point) {
                return Some(cell);
            }
        }
        session
            .data
            .terrain
            .pick_screen(
                session.projection(),
                view,
                GridPoint::new(cursor.x, cursor.y),
            )
            .ok()?
            .map(|hit| hit.cell)
    });
}

#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    reason = "Physical pixels are converted to bounded presentation collider coordinates"
)]
fn canvas_contains(cursor: gridthorn::CursorPosition, viewport: WindowViewport) -> bool {
    if viewport.width == 0 || viewport.height == 0 {
        return false;
    }
    let (scale, _, _) = super::session::ui_layout(viewport);
    let bounds = Aabb2d::new(
        Vec2::new(
            (viewport.width as f32 - 292.0 * scale).midpoint(236.0 * scale),
            (viewport.height as f32 - 62.0 * scale).midpoint(112.0 * scale),
        ),
        Vec2::new(
            (viewport.width as f32 - 530.0 * scale) * 0.5,
            (viewport.height as f32 - 176.0 * scale) * 0.5,
        ),
    )
    .expect("canvas bounds");
    Circle2d::new(Vec2::new(cursor.x as f32, cursor.y as f32), 0.0)
        .is_ok_and(|point| gridthorn::overlaps(bounds.into(), point.into()))
}

fn map_commands(session: &mut Session, input: &InputState, active: &str, button_clicked: bool) {
    if input.mouse_button_just_pressed(MouseButton::Right) {
        session.tool = Tool::Select;
        session.assigning = None;
        session.moving = None;
    }
    if input.mouse_button_just_pressed(MouseButton::Left) {
        session.map_press = if active == "play" && !button_clicked {
            session.hover
        } else {
            None
        };
    }
    let released = input.mouse_button_just_released(MouseButton::Left);
    let valid_press = session.map_press.is_some() && session.map_press == session.hover;
    if active == "play"
        && !button_clicked
        && released
        && valid_press
        && let Some(cell) = session.hover
    {
        let command = match session.tool {
            Tool::Select => {
                if let Some(id) = session.data.occupancy.object_at(cell) {
                    if let Some((worker, source)) = session.assigning {
                        if let Some(source) = source {
                            session.assigning = None;
                            session.queue(Command::Assign(worker, source, id));
                            session.notice = "ASSIGNMENT QUEUED".into();
                        } else {
                            session.assigning = Some((worker, Some(id)));
                            session.notice = "NOW CLICK DESTINATION".into();
                        }
                    } else {
                        session.selected = Some(id);
                        session.employee = session.staff().first().copied();
                    }
                } else {
                    session.selected = None;
                    session.employee = None;
                }
                None
            }
            Tool::Build(kind) => Some(Command::Build(kind, cell)),
            Tool::Road => Some(Command::Road(cell)),
            Tool::Remove => Some(Command::Remove(cell)),
            Tool::Move => {
                if let Some(id) = session.moving.take() {
                    Some(Command::Move(id, cell))
                } else {
                    session.moving = session.data.occupancy.object_at(cell);
                    None
                }
            }
        };
        if let Some(command) = command {
            session.queue(command);
            session.notice = "COMMAND QUEUED FOR THE NEXT TICK".into();
        }
    }
    if released || session.hover.is_none() {
        session.map_press = None;
    }
}

fn building_hit(session: &Session, point: GridPoint) -> Option<gridthorn::grid::GridCell> {
    session
        .data
        .buildings
        .values()
        .filter(|b| {
            let center = session.projection().cell_center(b.cell).expect("cell");
            (point.x - center.x).abs() <= 33.5 && (point.y - (center.y - 19.0)).abs() <= 36.0
        })
        .max_by_key(|b| (b.cell.column + b.cell.row, b.cell.column))
        .map(|b| b.cell)
}
