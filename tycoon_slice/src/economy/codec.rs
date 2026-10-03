use super::model::{
    Building, Command, DOCK, Harbor, Kind, MAX, MIN, ROADS, Resource, Role, Worker,
};
use gridthorn::grid::{GridCell, GridFootprint, GridObjectId, PlacementMap};
use gridthorn::{GameCommandQueue, WorldSaveCodec};
use std::fmt::Write;

pub struct HarborCodec;

impl WorldSaveCodec<Harbor, Command> for HarborCodec {
    fn encode(
        &self,
        data: &Harbor,
        commands: &GameCommandQueue<Command>,
    ) -> Result<String, String> {
        let mut result = format!(
            "harbor-v2 {} {} {} {} {}\n",
            data.coins, data.timber, data.shipped, data.next_id, data.next_worker
        );
        for (id, b) in &data.buildings {
            writeln!(
                result,
                "b {} {} {} {} {} {} {} {}",
                id.0,
                b.kind.sprite(),
                b.cell.column,
                b.cell.row,
                b.raw,
                b.planks,
                b.export.code(),
                b.last_porter
            )
            .map_err(|e| e.to_string())?;
        }
        for cell in &data.roads {
            writeln!(result, "r {} {}", cell.column, cell.row).map_err(|e| e.to_string())?;
        }
        for (id, w) in &data.workers {
            writeln!(
                result,
                "w {} {} {} {} {} {} {} {} {} {}",
                id,
                w.role.code(),
                w.home.0,
                w.source.0,
                w.destination.0,
                w.cell.column,
                w.cell.row,
                w.cargo.map_or(2, Resource::code),
                w.remaining,
                u8::from(w.outbound)
            )
            .map_err(|e| e.to_string())?;
        }
        let mut queue = commands.clone();
        for command in queue.drain() {
            writeln!(result, "q {}", encode_command(&command)).map_err(|e| e.to_string())?;
        }
        Ok(result)
    }

    fn decode(&self, payload: &str) -> Result<(Harbor, GameCommandQueue<Command>), String> {
        let (mut data, lines) = decode_header(payload)?;
        let mut queue = GameCommandQueue::new();
        for line in lines {
            let fields: Vec<_> = line.split_whitespace().collect();
            match fields.first().copied() {
                Some("b") if fields.len() == 9 => {
                    let id = GridObjectId(number(fields[1])?);
                    let kind = kind(fields[2])?;
                    let cell = cell(fields[3], fields[4])?;
                    if id.0 == 0
                        || id.0 >= data.next_id
                        || data.buildings.contains_key(&id)
                        || !(data.land(cell) || kind == Kind::Port && cell == DOCK)
                    {
                        return Err("invalid building identity or terrain".into());
                    }
                    data.occupancy
                        .place(id, cell, GridFootprint::single_cell())
                        .map_err(|e| e.to_string())?;
                    data.buildings.insert(
                        id,
                        Building {
                            kind,
                            cell,
                            raw: number(fields[5])?,
                            planks: number(fields[6])?,
                            export: resource(fields[7])?,
                            last_porter: number(fields[8])?,
                        },
                    );
                }
                Some("r") if fields.len() == 3 => {
                    let cell = cell(fields[1], fields[2])?;
                    if (!data.land(cell) && cell != DOCK) || data.roads.contains(&cell) {
                        return Err("invalid or duplicate road".into());
                    }
                    data.add_road(cell);
                }
                Some("w") if fields.len() == 11 => {
                    let id = number(fields[1])?;
                    let worker = Worker {
                        role: role(fields[2])?,
                        home: GridObjectId(number(fields[3])?),
                        source: GridObjectId(number(fields[4])?),
                        destination: GridObjectId(number(fields[5])?),
                        cell: cell(fields[6], fields[7])?,
                        cargo: if fields[8] == "2" {
                            None
                        } else {
                            Some(resource(fields[8])?)
                        },
                        remaining: fields[9].parse().map_err(|_| "invalid work timer")?,
                        outbound: flag(fields[10])?,
                    };
                    if id == 0
                        || id >= data.next_worker
                        || data.workers.len() >= 64
                        || data.workers.insert(id, worker).is_some()
                    {
                        return Err("invalid worker identity".into());
                    }
                }
                Some("q") if queue.len() < 256 => queue.push(decode_command(&fields[1..])?),
                _ => return Err(format!("unknown or malformed harbor record: {line}")),
            }
        }
        validate_workers(&data)?;
        if data
            .roads
            .iter()
            .any(|cell| data.occupancy.object_at(*cell).is_some())
        {
            return Err("inconsistent harbor world".into());
        }
        data.message = "WORLD RESTORED - EXACT ECONOMY CONTINUATION".into();
        Ok((data, queue))
    }
}

fn encode_command(command: &Command) -> String {
    match command {
        Command::Build(k, c) => format!("build {} {} {}", k.sprite(), c.column, c.row),
        Command::Road(c) => format!("road {} {}", c.column, c.row),
        Command::Remove(c) => format!("remove {} {}", c.column, c.row),
        Command::Move(id, c) => format!("move {} {} {}", id.0, c.column, c.row),
        Command::Hire(role, id) => format!("hire {} {}", role.code(), id.0),
        Command::Assign(id, source, destination) => {
            format!("assign {} {} {}", id, source.0, destination.0)
        }
        Command::Export(id, resource) => format!("export {} {}", id.0, resource.code()),
    }
}

fn decode_command(f: &[&str]) -> Result<Command, String> {
    match f {
        ["build", k, x, y] => Ok(Command::Build(kind(k)?, cell(x, y)?)),
        ["road", x, y] => Ok(Command::Road(cell(x, y)?)),
        ["remove", x, y] => Ok(Command::Remove(cell(x, y)?)),
        ["move", id, x, y] => Ok(Command::Move(GridObjectId(number(id)?), cell(x, y)?)),
        ["hire", r, id] => Ok(Command::Hire(role(r)?, GridObjectId(number(id)?))),
        ["assign", id, source, destination] => Ok(Command::Assign(
            number(id)?,
            GridObjectId(number(source)?),
            GridObjectId(number(destination)?),
        )),
        ["export", id, r] => Ok(Command::Export(GridObjectId(number(id)?), resource(r)?)),
        _ => Err("unknown game command".into()),
    }
}

fn number(value: &str) -> Result<u64, String> {
    value
        .parse()
        .map_err(|_| format!("invalid integer: {value}"))
}
fn flag(value: &str) -> Result<bool, String> {
    match value {
        "0" => Ok(false),
        "1" => Ok(true),
        _ => Err("invalid outcome flag".into()),
    }
}
fn kind(value: &str) -> Result<Kind, String> {
    match value {
        "4" => Ok(Kind::Sawmill),
        "5" => Ok(Kind::RawStore),
        "6" => Ok(Kind::Home),
        "8" => Ok(Kind::Forest),
        "16" => Ok(Kind::PlankStore),
        "7" => Ok(Kind::Port),
        _ => Err("unknown building kind".into()),
    }
}
fn cell(x: &str, y: &str) -> Result<GridCell, String> {
    let x = x.parse().map_err(|_| "invalid column")?;
    let y = y.parse().map_err(|_| "invalid row")?;
    if !(MIN..=MAX).contains(&x) || !(MIN..=MAX).contains(&y) {
        return Err("cell outside harbor".into());
    }
    Ok(GridCell::new(x, y))
}

fn role(value: &str) -> Result<Role, String> {
    match value {
        "0" => Ok(Role::Lumberjack),
        "1" => Ok(Role::Carpenter),
        "2" => Ok(Role::Porter),
        _ => Err("invalid role".into()),
    }
}
fn resource(value: &str) -> Result<Resource, String> {
    match value {
        "0" => Ok(Resource::Raw),
        "1" => Ok(Resource::Planks),
        _ => Err("invalid resource".into()),
    }
}
fn validate_workers(data: &Harbor) -> Result<(), String> {
    for (id, w) in &data.workers {
        if !data.roads.contains(&w.cell)
            || !worker_phase_valid(w)
            || !worker_binding_valid(data, w)
            || !data
                .buildings
                .get(&w.home)
                .is_some_and(|b| b.kind == Kind::Home)
            || !data.buildings.contains_key(&w.source)
            || !data.buildings.contains_key(&w.destination)
            || (w.role == Role::Lumberjack
                && data.workers.iter().any(|(other, worker)| {
                    other != id && worker.role == Role::Lumberjack && worker.source == w.source
                }))
            || data
                .workers
                .values()
                .filter(|worker| worker.home == w.home)
                .count()
                > 4
        {
            return Err("inconsistent worker assignment or housing".into());
        }
    }
    if data.timber != data.raw_total() {
        return Err("inconsistent resource totals".into());
    }
    if data.buildings.values().any(|b| {
        b.last_porter >= data.next_worker
            || b.raw > 9999
            || b.planks > 9999
            || (b.raw > 0 && !matches!(b.kind, Kind::RawStore | Kind::Sawmill))
            || (b.planks > 0 && b.kind != Kind::PlankStore)
            || (b.kind == Kind::Port && b.cell != DOCK && b.cell.column != 4)
    }) {
        return Err("stock out of range".into());
    }
    Ok(())
}

fn decode_header(payload: &str) -> Result<(Harbor, std::str::Lines<'_>), String> {
    let mut lines = payload.lines();
    let header: Vec<_> = lines
        .next()
        .ok_or("missing harbor header")?
        .split_whitespace()
        .collect();
    if header.len() != 6 || header[0] != "harbor-v2" {
        return Err("unsupported harbor payload".into());
    }
    let mut data = Harbor::new();
    data.coins = number(header[1])?;
    data.timber = number(header[2])?;
    data.shipped = number(header[3])?;
    data.next_id = number(header[4])?;
    data.next_worker = number(header[5])?;
    data.workers.clear();
    if data.coins > 999_999
        || data.timber > 1_000_000
        || data.shipped > 1_000_000
        || data.next_id == 0
        || data.next_id > 1_000_000
        || data.next_worker == 0
        || data.next_worker > 1_000_000
    {
        return Err("harbor counters out of range".into());
    }
    data.buildings.clear();
    data.occupancy = PlacementMap::new();
    for cell in std::mem::take(&mut data.roads) {
        data.terrain
            .set_tile(ROADS, cell, None)
            .map_err(|e| e.to_string())?;
    }
    Ok((data, lines))
}

fn worker_binding_valid(data: &Harbor, worker: &Worker) -> bool {
    let (Some(source), Some(destination)) = (
        data.buildings.get(&worker.source),
        data.buildings.get(&worker.destination),
    ) else {
        return false;
    };
    if destination.kind == Kind::Port && worker.outbound && worker.cargo != Some(destination.export)
    {
        return false;
    }
    match worker.role {
        Role::Lumberjack => source.kind == Kind::Forest && destination.kind == Kind::RawStore,
        Role::Carpenter => source.kind == Kind::Sawmill && destination.kind == Kind::PlankStore,
        Role::Porter => match source.kind {
            Kind::RawStore => {
                matches!(destination.kind, Kind::Sawmill | Kind::Port)
                    && worker.cargo.is_none_or(|r| r == Resource::Raw)
            }
            Kind::PlankStore => {
                destination.kind == Kind::Port && worker.cargo.is_none_or(|r| r == Resource::Planks)
            }
            _ => false,
        },
    }
}
fn worker_phase_valid(worker: &Worker) -> bool {
    if worker.remaining > 0 {
        !worker.outbound
            && match worker.role {
                Role::Lumberjack => worker.remaining <= 30 && worker.cargo.is_none(),
                Role::Carpenter => worker.remaining <= 40 && worker.cargo == Some(Resource::Raw),
                Role::Porter => false,
            }
    } else {
        worker.outbound == worker.cargo.is_some()
            && match worker.role {
                Role::Lumberjack => worker.cargo.is_none_or(|r| r == Resource::Raw),
                Role::Carpenter => worker.cargo.is_none_or(|r| r == Resource::Planks),
                Role::Porter => true,
            }
    }
}
