use crate::economy::{
    Root,
    codec::HarborCodec,
    model::{Command, Kind},
    runner,
};
use gridthorn::WorldSaveCodec;
use gridthorn::grid::GridCell;

#[test]
fn partitioning_snapshots_rng_and_pending_commands_continue_exactly() {
    let mut original = runner("harbor", u64::MAX).unwrap();
    original.run_ticks(23).unwrap();
    original.world().update_resource(|root: &mut Root| {
        root.commands
            .push(Command::Build(Kind::Home, GridCell::new(-2, 1)));
    });
    let snapshot = original.snapshot().unwrap();
    let document = original.save_document(&HarborCodec).unwrap();
    let mut restored = runner("harbor", 1).unwrap();
    restored.restore(&snapshot).unwrap();
    let mut loaded = runner("harbor", 2).unwrap();
    loaded.load_document(&document, &HarborCodec).unwrap();
    original.run_ticks(377).unwrap();
    for ticks in [0, 7, 130, 240] {
        restored.run_ticks(ticks).unwrap();
    }
    loaded.run_ticks(377).unwrap();
    let expected = original.save_document(&HarborCodec).unwrap();
    assert_eq!(restored.save_document(&HarborCodec).unwrap(), expected);
    assert_eq!(loaded.save_document(&HarborCodec).unwrap(), expected);
    assert_eq!(loaded.snapshot().unwrap().state().data.homes(), 2);
}

#[test]
fn invalid_world_and_incompatible_scenario_preserve_the_live_world() {
    let mut runtime = runner("harbor", 42).unwrap();
    runtime.run_ticks(20).unwrap();
    let document = runtime.save_document(&HarborCodec).unwrap();
    assert!(
        runtime
            .load_document(&document.replace("harbor-v2", "harbor-v999"), &HarborCodec)
            .is_err()
    );
    assert_eq!(runtime.save_document(&HarborCodec).unwrap(), document);
    let snapshot = runtime.snapshot().unwrap();
    let payload = HarborCodec
        .encode(&snapshot.state().data, &snapshot.state().commands)
        .unwrap();
    for extra in [
        "b 50 4 6 6\n",
        "r 0 0\n",
        "q build 99 1 1\n",
        "c 0 1 1 5 0\n",
        "unknown 4\n",
    ] {
        assert!(
            HarborCodec.decode(&(payload.clone() + extra)).is_err(),
            "{extra}"
        );
    }
    let mut sandbox = runner("sandbox", 42).unwrap();
    assert!(sandbox.load_document(&document, &HarborCodec).is_err());
}

#[test]
fn real_save_file_replacement_reconstructs_transient_maps() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../target");
    std::fs::create_dir_all(&root).unwrap();
    let path = root.join(format!("harbor-save-test-{}.toml", std::process::id()));
    let mut runtime = runner("harbor", 42).unwrap();
    runtime.save_file(&path, &HarborCodec).unwrap();
    runtime.run_ticks(31).unwrap();
    runtime.save_file(&path, &HarborCodec).unwrap();
    let mut loaded = runner("harbor", 99).unwrap();
    loaded.load_file(&path, &HarborCodec).unwrap();
    assert_eq!(
        loaded.save_document(&HarborCodec).unwrap(),
        runtime.save_document(&HarborCodec).unwrap()
    );
    let state = loaded.snapshot().unwrap();
    assert_eq!(state.state().data.occupancy.objects().len(), 6);
    assert!(
        state
            .state()
            .data
            .terrain
            .layers()
            .all(|(_, layer)| layer.chunks().count() > 0)
    );
    std::fs::remove_file(path).unwrap();
}

#[test]
fn inconsistent_jobs_cargo_stock_and_overcrowded_homes_are_rejected() {
    let snapshot = runner("harbor", 42).unwrap().snapshot().unwrap();
    let original = &snapshot.state().data;
    for change in 0..5 {
        let mut data = original.clone();
        match change {
            0 => data.workers.get_mut(&1).unwrap().home = gridthorn::grid::GridObjectId(3),
            1 => data.workers.get_mut(&2).unwrap().remaining = 5,
            2 => {
                data.buildings
                    .get_mut(&gridthorn::grid::GridObjectId(2))
                    .unwrap()
                    .raw += 1;
            }
            3 => data.workers.get_mut(&1).unwrap().source = gridthorn::grid::GridObjectId(6),
            _ => {
                data.next_worker = 6;
                data.workers.insert(5, data.workers[&4].clone());
            }
        }
        let payload = HarborCodec
            .encode(&data, &snapshot.state().commands)
            .unwrap();
        assert!(
            HarborCodec.decode(&payload).is_err(),
            "invalid mutation {change}"
        );
    }
}
