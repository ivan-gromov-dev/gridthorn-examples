use super::input::{click, key};
use crate::game::{
    runtime,
    session::{Session, button_bounds},
};
use gridthorn::{GameStateStack, KeyCode, SimulationControl, WindowViewport};
use std::time::Duration;

fn button(runtime: &mut gridthorn::ApplicationRuntime, index: usize) {
    let (p, _) = button_bounds(index);
    click(runtime, [f64::from(p[0] + 12.0), f64::from(p[1] + 12.0)]);
}
fn tick(runtime: &mut gridthorn::ApplicationRuntime) -> u64 {
    runtime
        .world()
        .read_resource(|s: &Session| s.ticks)
        .unwrap()
}

#[test]
fn escape_and_menu_button_pause_and_restore_half_speed_without_exiting() {
    let mut app = runtime(false, false).unwrap();
    key(&mut app, KeyCode::Enter);
    button(&mut app, 7);
    key(&mut app, KeyCode::Escape);
    assert_eq!(
        app.world()
            .read_resource(|s: &GameStateStack| s.current().as_str().to_owned())
            .unwrap(),
        "menu"
    );
    let before = tick(&mut app);
    app.run_timed_frame(Duration::from_secs(1)).unwrap();
    assert_eq!(tick(&mut app), before);
    button(&mut app, 20);
    assert_eq!(
        app.world()
            .read_resource(|c: &SimulationControl| c.speed().denominator())
            .unwrap(),
        2
    );
    let before = tick(&mut app);
    app.run_timed_frame(Duration::from_secs(1)).unwrap();
    assert_eq!(tick(&mut app), before + 5);
    button(&mut app, 20);
    key(&mut app, KeyCode::Escape);
    assert_eq!(
        app.world()
            .read_resource(|s: &GameStateStack| s.current().as_str().to_owned())
            .unwrap(),
        "play"
    );
    app.shutdown();
}

#[test]
fn menu_save_load_restores_work_clock_and_keeps_menu_open() {
    let mut app = runtime(false, false).unwrap();
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(format!("../target/menu-save-{}.toml", std::process::id()));
    app.world()
        .update_resource(|s: &mut Session| s.save_path = path.clone());
    key(&mut app, KeyCode::Enter);
    for _ in 0..25 {
        app.run_timed_frame(Duration::from_millis(100)).unwrap();
    }
    key(&mut app, KeyCode::Escape);
    let saved_tick = tick(&mut app);
    button(&mut app, 10);
    assert!(path.exists());
    let before = app
        .world()
        .update_resource_with(|s: &mut Session| {
            s.runner
                .save_document(&crate::economy::codec::HarborCodec)
                .unwrap()
        })
        .unwrap();
    key(&mut app, KeyCode::Escape);
    for _ in 0..30 {
        app.run_timed_frame(Duration::from_millis(100)).unwrap();
    }
    key(&mut app, KeyCode::Escape);
    button(&mut app, 11);
    assert_eq!(tick(&mut app), saved_tick);
    assert_eq!(
        app.world()
            .read_resource(|s: &GameStateStack| s.current().as_str().to_owned())
            .unwrap(),
        "menu"
    );
    let after = app
        .world()
        .update_resource_with(|s: &mut Session| {
            s.runner
                .save_document(&crate::economy::codec::HarborCodec)
                .unwrap()
        })
        .unwrap();
    assert_eq!(before, after);
    key(&mut app, KeyCode::Escape);
    assert!(tick(&mut app) > saved_tick);
    app.shutdown();
    std::fs::remove_file(path).unwrap();
}

#[test]
fn all_speed_choices_scale_the_authoritative_timer() {
    let mut app = runtime(false, false).unwrap();
    key(&mut app, KeyCode::Enter);
    for (index, steps) in [(7, 5), (27, 10), (28, 20), (29, 40)] {
        button(&mut app, index);
        let before = tick(&mut app);
        for _ in 0..10 {
            app.run_timed_frame(Duration::from_millis(100)).unwrap();
        }
        assert_eq!(tick(&mut app), before + steps);
    }
    button(&mut app, 6);
    let before = tick(&mut app);
    app.run_timed_frame(Duration::from_secs(1)).unwrap();
    assert_eq!(tick(&mut app), before);
    app.shutdown();
}

#[test]
fn reduced_viewport_keeps_menu_buttons_inside_window() {
    let viewport = WindowViewport {
        width: 800,
        height: 600,
    };
    for index in [10, 11, 12, 13, 18, 19, 21, 26] {
        let (p, size) = crate::game::session::layout_bounds(index, viewport);
        assert!(p[0] >= 0.0 && p[1] >= 0.0 && p[0] + size[0] <= 800.0 && p[1] + size[1] <= 600.0);
    }
}

#[test]
fn building_roof_click_selects_its_staff_and_menu_blocks_map_edits() {
    let mut app = runtime(false, false).unwrap();
    key(&mut app, KeyCode::Enter);
    let roof = app
        .world()
        .read_resource(|s: &Session| {
            let point = s
                .projection()
                .cell_center(gridthorn::grid::GridCell::new(-4, -1))
                .unwrap();
            let scale = 800.0 / f64::from(s.height);
            [
                (point.x - f64::from(s.camera[0])) * scale + 640.0,
                (point.y - 30.0 - f64::from(s.camera[1])) * scale + 400.0,
            ]
        })
        .unwrap();
    click(&mut app, roof);
    assert_eq!(
        app.world().read_resource(|s: &Session| s.selected).unwrap(),
        Some(gridthorn::grid::GridObjectId(1))
    );
    assert_eq!(app.world().read_resource(Session::staff).unwrap(), vec![1]);
    key(&mut app, KeyCode::Escape);
    app.world().update_resource(|s: &mut Session| {
        s.tool = crate::game::session::Tool::Build(crate::economy::model::Kind::Home);
    });
    click(&mut app, [300.0, 400.0]);
    assert_eq!(
        app.world()
            .read_resource(|s: &Session| s.data.homes())
            .unwrap(),
        1
    );
    app.shutdown();
}
