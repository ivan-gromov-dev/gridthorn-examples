use crate::economy::{
    Root,
    model::{Command, Harbor, Kind, Resource, Role},
    runner,
};
use gridthorn::grid::{GridCell, GridObjectId};

#[test]
fn starting_settlement_has_four_housed_workers_and_raw_only_port() {
    let data = Harbor::new();
    assert_eq!(data.coins, 120);
    assert_eq!(data.workers.len(), 4);
    assert_eq!(data.homes(), 1);
    assert_eq!(data.buildings[&GridObjectId(6)].export, Resource::Raw);
    assert_eq!(
        data.workers
            .values()
            .filter(|w| w.role == Role::Porter)
            .count(),
        2
    );
    assert!(data.workers.values().all(|w| w.home == GridObjectId(5)));
}

#[test]
fn placement_and_occupied_building_rejections_preserve_funds() {
    let mut data = Harbor::new();
    data.apply(Command::Build(Kind::Home, GridCell::new(0, 0)));
    data.apply(Command::Move(GridObjectId(1), GridCell::new(-3, 1)));
    data.apply(Command::Remove(GridCell::new(-4, -1)));
    assert_eq!(data.coins, 120);
    assert_eq!(data.buildings.len(), 6);
    data.apply(Command::Build(Kind::Home, GridCell::new(1, 1)));
    assert_eq!(data.coins, 80);
    data.apply(Command::Move(GridObjectId(7), GridCell::new(2, 1)));
    assert_eq!(data.coins, 75);
    data.apply(Command::Remove(GridCell::new(2, 1)));
    assert_eq!(data.coins, 95);
}

#[test]
fn broken_road_stalls_delivery_and_repair_resumes_it() {
    let mut runtime = runner("harbor", 42).unwrap();
    runtime.world().update_resource(|root: &mut Root| {
        root.commands.push(Command::Remove(GridCell::new(3, 0)));
    });
    runtime.run_ticks(100).unwrap();
    assert_eq!(runtime.snapshot().unwrap().state().data.shipped, 0);
    runtime
        .world()
        .update_resource(|root: &mut Root| root.commands.push(Command::Road(GridCell::new(3, 0))));
    runtime.run_ticks(60).unwrap();
    assert!(runtime.snapshot().unwrap().state().data.shipped > 0);
}

#[test]
fn processing_takes_ticks_and_planks_wait_without_a_matching_port() {
    let mut runtime = runner("harbor", 42).unwrap();
    runtime.run_ticks(20).unwrap();
    let state = runtime.snapshot().unwrap();
    assert!(state.state().data.workers[&2].remaining > 0);
    assert_eq!(state.state().data.buildings[&GridObjectId(4)].planks, 0);
    runtime.run_ticks(380).unwrap();
    let state = runtime.snapshot().unwrap();
    assert!(state.state().data.buildings[&GridObjectId(4)].planks > 0);
    assert!(state.state().data.shipped > 0);
    assert!(state.state().data.coins > 120);
}

#[test]
fn hiring_requires_gold_housing_and_unique_lumberjack_workplace() {
    let mut data = Harbor::new();
    data.apply(Command::Hire(Role::Porter, GridObjectId(2)));
    assert_eq!(data.workers.len(), 4);
    assert_eq!(data.coins, 120);
    data.apply(Command::Build(Kind::Home, GridCell::new(1, 1)));
    data.apply(Command::Hire(Role::Lumberjack, GridObjectId(1)));
    assert_eq!(data.workers.len(), 4);
    data.apply(Command::Hire(Role::Porter, GridObjectId(2)));
    assert_eq!(data.workers.len(), 5);
    assert_eq!(data.coins, 45);
    assert_eq!(data.workers[&5].home, GridObjectId(7));
}

#[test]
fn port_resource_selection_controls_compatible_assignments_and_sales() {
    let mut data = Harbor::new();
    data.apply(Command::Assign(4, GridObjectId(4), GridObjectId(6)));
    assert_eq!(data.workers[&4].source, GridObjectId(2));
    data.apply(Command::Export(GridObjectId(6), Resource::Planks));
    data.apply(Command::Assign(4, GridObjectId(4), GridObjectId(6)));
    assert_eq!(data.workers[&4].source, GridObjectId(4));
    assert!(!data.assignment_valid(Role::Porter, GridObjectId(2), GridObjectId(6)));
    let mut runtime = runner("harbor", 42).unwrap();
    runtime
        .world()
        .update_resource(|root: &mut Root| root.data = data);
    runtime.run_ticks(300).unwrap();
    assert!(runtime.snapshot().unwrap().state().data.shipped > 0);
}

#[test]
fn warehouse_dispatch_does_not_starve_the_export_porter() {
    let mut app = runner("harbor", 42).unwrap();
    app.run_ticks(1000).unwrap();
    let state = app.snapshot().unwrap();
    assert!(state.state().data.shipped >= 10);
    assert!(state.state().data.coins >= 180);
    assert!(state.state().data.buildings[&GridObjectId(4)].planks >= 5);
}

#[test]
fn multiple_porters_share_one_port_and_second_port_sells_planks() {
    let mut app = runner("sandbox", 42).unwrap();
    app.world().update_resource(|root: &mut Root| {
        for command in [
            Command::Build(Kind::Port, GridCell::new(4, 1)),
            Command::Export(GridObjectId(7), Resource::Planks),
            Command::Build(Kind::Home, GridCell::new(-2, 1)),
            Command::Hire(Role::Porter, GridObjectId(4)),
            Command::Hire(Role::Porter, GridObjectId(4)),
        ] {
            root.commands.push(command);
        }
    });
    app.run_ticks(600).unwrap();
    let snapshot = app.snapshot().unwrap();
    let data = &snapshot.state().data;
    assert_eq!(data.workers.len(), 6);
    assert_eq!(data.workers[&5].destination, GridObjectId(7));
    assert_eq!(data.workers[&6].destination, GridObjectId(7));
    assert_eq!(data.buildings[&GridObjectId(6)].export, Resource::Raw);
    assert_eq!(data.buildings[&GridObjectId(7)].export, Resource::Planks);
    assert!(data.shipped > 10);
    assert!(data.coins > 1800);
}
