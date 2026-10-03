use gridthorn::grid::{GridCell, GridObjectId, PlacementMap, TileLayerId, TileMap};
use std::collections::{BTreeMap, BTreeSet};

pub const GROUND: TileLayerId = TileLayerId(0);
pub const ROADS: TileLayerId = TileLayerId(1);
pub const DOCK: GridCell = GridCell::new(5, 0);
pub const MIN: i32 = -5;
pub const MAX: i32 = 6;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Sawmill,
    RawStore,
    Home,
    Forest,
    PlankStore,
    Port,
}

impl Kind {
    pub fn cost(self) -> u64 {
        match self {
            Self::Sawmill => 100,
            Self::RawStore | Self::Forest | Self::PlankStore => 60,
            Self::Home => 40,
            Self::Port => 180,
        }
    }
    pub fn sprite(self) -> u8 {
        match self {
            Self::Sawmill => 4,
            Self::RawStore => 5,
            Self::Home => 6,
            Self::Forest => 8,
            Self::PlankStore => 16,
            Self::Port => 7,
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            Self::Sawmill => "SAWMILL",
            Self::RawStore => "LOG WAREHOUSE",
            Self::Home => "HOUSE",
            Self::Forest => "FOREST",
            Self::PlankStore => "PLANK WAREHOUSE",
            Self::Port => "PORT",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Command {
    Build(Kind, GridCell),
    Road(GridCell),
    Remove(GridCell),
    Move(GridObjectId, GridCell),
    Hire(Role, GridObjectId),
    Assign(u64, GridObjectId, GridObjectId),
    Export(GridObjectId, Resource),
}

#[derive(Clone, Debug)]
pub struct Building {
    pub kind: Kind,
    pub cell: GridCell,
    pub raw: u64,
    pub planks: u64,
    pub export: Resource,
    pub last_porter: u64,
}

#[derive(Clone, Debug)]
pub struct Harbor {
    pub coins: u64,
    pub timber: u64,
    pub shipped: u64,
    pub next_id: u64,
    pub buildings: BTreeMap<GridObjectId, Building>,
    pub roads: BTreeSet<GridCell>,
    pub workers: BTreeMap<u64, Worker>,
    pub next_worker: u64,
    pub terrain: TileMap<u8>,
    pub occupancy: PlacementMap,
    pub message: String,
}

impl Default for Harbor {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resource {
    Raw,
    Planks,
}
impl Resource {
    pub fn name(self) -> &'static str {
        match self {
            Self::Raw => "LOGS",
            Self::Planks => "PLANKS",
        }
    }
    pub fn code(self) -> u8 {
        u8::from(self == Self::Planks)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    Lumberjack,
    Carpenter,
    Porter,
}
impl Role {
    pub fn name(self) -> &'static str {
        match self {
            Self::Lumberjack => "LUMBERJACK",
            Self::Carpenter => "CARPENTER",
            Self::Porter => "PORTER",
        }
    }
    pub fn cost(self) -> u64 {
        match self {
            Self::Lumberjack => 50,
            Self::Carpenter => 75,
            Self::Porter => 35,
        }
    }
    pub fn code(self) -> u8 {
        match self {
            Self::Lumberjack => 0,
            Self::Carpenter => 1,
            Self::Porter => 2,
        }
    }
}
#[derive(Clone, Debug)]
pub struct Worker {
    pub role: Role,
    pub home: GridObjectId,
    pub source: GridObjectId,
    pub destination: GridObjectId,
    pub cell: GridCell,
    pub cargo: Option<Resource>,
    pub remaining: u16,
    pub outbound: bool,
}
impl Worker {
    pub fn status(&self) -> &'static str {
        if self.remaining > 0 {
            "WORKING"
        } else if self.outbound {
            "DELIVERING"
        } else {
            "RETURNING / WAITING"
        }
    }
}
