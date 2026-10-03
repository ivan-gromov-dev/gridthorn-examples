use super::session::{Session, Tool};
use crate::economy::model::{Command, Kind, Resource, Role};
use gridthorn::{SimulationControl, SimulationSpeed};

pub fn activate(
    session: &mut Session,
    index: usize,
    active: &str,
    control: &mut SimulationControl,
) -> Result<Option<&'static str>, Box<dyn std::error::Error>> {
    match index {
        0..=5 => construction(session, index),
        22 => {
            session.tool = Tool::Select;
            session.assigning = None;
        }
        23 => session.tool = Tool::Build(Kind::Forest),
        24 => session.tool = Tool::Build(Kind::PlankStore),
        25 => session.tool = Tool::Build(Kind::Port),
        27..=29 => {
            control.set_speed(SimulationSpeed::new(1 << (index - 27), 1)?);
            control.resume();
        }
        30..=34 => employment(session, index),
        6..=9 => simulation(session, index, active, control)?,
        10..=13 if active != "play" => return persistence(session, index, control),
        14 => session.chunks = !session.chunks,
        15 => {
            session.square = !session.square;
            session.notice = "SAME INTEGER WORLD - DIFFERENT PROJECTION".into();
        }
        16 => session.height = (session.height - 80.0).max(400.0),
        17 => session.height = (session.height + 80.0).min(1200.0),
        18 | 19 => {
            *session = Session::new(if index == 19 { "sandbox" } else { "harbor" })?;
            *control = SimulationControl::default();
            return Ok(Some("play"));
        }
        20 | 26 => {
            if active == "play" {
                session.resume_control = *control;
                session.runner.world().insert_resource(*control);
                control.pause();
                return Ok(Some("menu"));
            }
            *control = session.resume_control;
            return Ok(Some("play"));
        }
        _ => {}
    }
    Ok(None)
}

fn construction(session: &mut Session, index: usize) {
    session.tool = match index {
        1 => Tool::Build(Kind::Sawmill),
        2 => Tool::Build(Kind::RawStore),
        3 => Tool::Build(Kind::Home),
        4 => {
            session.moving = None;
            session.notice = "SELECT A BUILDING THEN A FREE CELL - MOVE COSTS 5".into();
            Tool::Move
        }
        5 => Tool::Remove,
        _ => Tool::Road,
    };
}

fn simulation(
    session: &mut Session,
    index: usize,
    active: &str,
    control: &mut SimulationControl,
) -> Result<(), Box<dyn std::error::Error>> {
    match index {
        6 if active == "play" => {
            if control.is_paused() {
                control.resume();
            } else {
                control.pause();
            }
        }
        7 => {
            control.set_speed(SimulationSpeed::new(1, 2)?);
            control.resume();
        }
        8 if active == "play" && control.is_paused() => {
            session.advance()?;
        }
        9 => session.paths = !session.paths,
        _ => {}
    }
    Ok(())
}

fn persistence(
    session: &mut Session,
    index: usize,
    control: &mut SimulationControl,
) -> Result<Option<&'static str>, Box<dyn std::error::Error>> {
    match index {
        10 => {
            session
                .runner
                .world()
                .insert_resource(session.resume_control);
            session.save()?;
        }
        11 => {
            session.load()?;
            session.resume_control = saved_control(session);
            control.pause();
        }
        12 => {
            session.view_snapshot = Some(super::view_bookmark::capture(session)?);
            session.snapshot = Some(session.runner.snapshot()?);
            session.notice = "SNAPSHOT AND VERSIONED CAMERA SCENE CAPTURED".into();
        }
        13 => {
            if let Some(snapshot) = &session.snapshot {
                session.runner.restore(snapshot)?;
                session.refresh()?;
                session.resume_control = saved_control(session);
                control.pause();
                if let Some(source) = session.view_snapshot.clone() {
                    super::view_bookmark::restore(session, &source)?;
                }
                session.notice = "SNAPSHOT RESTORED".into();
            } else {
                session.notice = "CAPTURE A SNAPSHOT FIRST".into();
            }
        }
        _ => {}
    }
    Ok(None)
}

fn saved_control(session: &mut Session) -> SimulationControl {
    session
        .runner
        .world()
        .read_resource(|c: &SimulationControl| *c)
        .unwrap_or_default()
}

fn employment(session: &mut Session, index: usize) {
    let Some(id) = session.selected else {
        return;
    };
    match index {
        30 => {
            let role = match session.data.buildings[&id].kind {
                Kind::Forest => Role::Lumberjack,
                Kind::Sawmill => Role::Carpenter,
                Kind::RawStore | Kind::PlankStore => Role::Porter,
                _ => {
                    session.notice = "HIRE AT A FOREST, MILL OR WAREHOUSE".into();
                    return;
                }
            };
            session.queue(Command::Hire(role, id));
        }
        31 => {
            let resource = if session.data.buildings[&id].export == Resource::Raw {
                Resource::Planks
            } else {
                Resource::Raw
            };
            session.queue(Command::Export(id, resource));
        }
        32 | 33 => {
            let staff = session.staff();
            let current = staff
                .iter()
                .position(|id| Some(*id) == session.employee)
                .unwrap_or(0);
            if !staff.is_empty() {
                let next = if index == 33 {
                    (current + 1) % staff.len()
                } else {
                    (current + staff.len() - 1) % staff.len()
                };
                session.employee = Some(staff[next]);
            }
        }
        34 => {
            if let Some(worker) = session
                .employee
                .or_else(|| session.staff().first().copied())
            {
                session.assigning = Some((worker, None));
                session.tool = Tool::Select;
                session.notice = "ASSIGN: CLICK SOURCE THEN DESTINATION BUILDING".into();
            }
        }
        _ => {}
    }
}
