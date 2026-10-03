use crate::game::{runtime, session::Session};
use gridthorn::{
    ButtonState, CursorPosition, InputBuffer, InputEvent, KeyCode, MouseButton, SimulationControl,
    WindowViewport,
};
use std::time::Duration;

pub(super) fn key(runtime: &mut gridthorn::ApplicationRuntime, key: KeyCode) {
    let mut input = InputBuffer::new();
    input.push(InputEvent::Keyboard {
        key,
        state: ButtonState::Pressed,
    });
    runtime.world().insert_resource(input.snapshot());
    runtime.run_timed_frame(Duration::from_millis(100)).unwrap();
    input.push(InputEvent::Keyboard {
        key,
        state: ButtonState::Released,
    });
    runtime.world().insert_resource(input.snapshot());
    runtime.run_timed_frame(Duration::ZERO).unwrap();
}

pub(super) fn click(runtime: &mut gridthorn::ApplicationRuntime, position: [f64; 2]) {
    let mut input = InputBuffer::new();
    input.push(InputEvent::CursorMoved(CursorPosition {
        x: position[0],
        y: position[1],
    }));
    input.push(InputEvent::MouseButton {
        button: MouseButton::Left,
        state: ButtonState::Pressed,
    });
    runtime.world().insert_resource(input.snapshot());
    runtime.run_timed_frame(Duration::ZERO).unwrap();
    input.push(InputEvent::MouseButton {
        button: MouseButton::Left,
        state: ButtonState::Released,
    });
    runtime.world().insert_resource(input.snapshot());
    runtime.run_timed_frame(Duration::from_millis(100)).unwrap();
}

#[test]
fn menu_pause_and_explicit_step_keep_presentation_live() {
    let mut runtime = runtime(false, false).unwrap();
    runtime.run_timed_frame(Duration::from_millis(100)).unwrap();
    assert_eq!(
        runtime.world().read_resource(|s: &Session| s.ticks),
        Some(0)
    );
    key(&mut runtime, KeyCode::Enter);
    key(&mut runtime, KeyCode::Space);
    assert!(
        runtime
            .world()
            .read_resource(|c: &SimulationControl| c.is_paused())
            .unwrap()
    );
    let tick = runtime
        .world()
        .read_resource(|s: &Session| s.ticks)
        .unwrap();
    let frames = runtime
        .world()
        .read_resource(|s: &Session| s.frames)
        .unwrap();
    runtime.run_timed_frame(Duration::from_secs(1)).unwrap();
    assert_eq!(
        runtime.world().read_resource(|s: &Session| s.ticks),
        Some(tick)
    );
    assert!(
        runtime
            .world()
            .read_resource(|s: &Session| s.frames > frames)
            .unwrap()
    );
    let (position, _) = crate::game::session::button_bounds(8);
    click(
        &mut runtime,
        [f64::from(position[0] + 12.0), f64::from(position[1] + 12.0)],
    );
    assert_eq!(
        runtime.world().read_resource(|s: &Session| s.ticks),
        Some(tick + 1)
    );
    runtime.shutdown();
}

#[test]
fn resized_viewport_picks_the_rendered_cell_and_queues_one_build() {
    let mut runtime = runtime(false, false).unwrap();
    key(&mut runtime, KeyCode::Enter);
    runtime.world().insert_resource(WindowViewport {
        width: 1600,
        height: 1000,
    });
    let target = gridthorn::grid::GridCell::new(2, 2);
    let screen = runtime
        .world()
        .update_resource_with(|s: &mut Session| {
            s.tool = crate::game::session::Tool::Build(crate::economy::model::Kind::Home);
            let point = s.projection().cell_center(target).unwrap();
            let scale = 1000.0 / f64::from(s.height);
            [
                (point.x - f64::from(s.camera[0])) * scale + 800.0,
                (point.y - f64::from(s.camera[1])) * scale + 500.0,
            ]
        })
        .unwrap();
    click(&mut runtime, screen);
    assert_eq!(
        runtime.world().read_resource(|s: &Session| s.hover),
        Some(Some(target))
    );
    assert_eq!(
        runtime.world().read_resource(|s: &Session| s.data.homes()),
        Some(2)
    );
    runtime.shutdown();
}

#[test]
fn button_drag_into_the_canvas_does_not_construct_and_speed_drives_exact_ticks() {
    let mut runtime = runtime(false, false).unwrap();
    key(&mut runtime, KeyCode::Enter);
    let target = gridthorn::grid::GridCell::new(2, 2);
    let screen = runtime
        .world()
        .read_resource(|s: &Session| {
            let p = s.projection().cell_center(target).unwrap();
            let scale = 800.0 / f64::from(s.height);
            [
                (p.x - f64::from(s.camera[0])) * scale + 640.0,
                (p.y - f64::from(s.camera[1])) * scale + 400.0,
            ]
        })
        .unwrap();
    let mut input = InputBuffer::new();
    input.push(InputEvent::CursorMoved(CursorPosition {
        x: 30.0,
        y: 300.0,
    }));
    input.push(InputEvent::MouseButton {
        button: MouseButton::Left,
        state: ButtonState::Pressed,
    });
    runtime.world().insert_resource(input.snapshot());
    runtime.run_timed_frame(Duration::ZERO).unwrap();
    input.push(InputEvent::CursorMoved(CursorPosition {
        x: screen[0],
        y: screen[1],
    }));
    input.push(InputEvent::MouseButton {
        button: MouseButton::Left,
        state: ButtonState::Released,
    });
    runtime.world().insert_resource(input.snapshot());
    runtime.run_timed_frame(Duration::from_millis(100)).unwrap();
    assert_eq!(
        runtime.world().read_resource(|s: &Session| s.data.homes()),
        Some(1)
    );
    let tick = runtime
        .world()
        .read_resource(|s: &Session| s.ticks)
        .unwrap();
    runtime
        .world()
        .update_resource(|c: &mut SimulationControl| {
            c.set_speed(gridthorn::SimulationSpeed::new(4, 1).unwrap());
        });
    runtime.world().insert_resource(input.snapshot());
    runtime.run_timed_frame(Duration::from_millis(100)).unwrap();
    assert_eq!(
        runtime.world().read_resource(|s: &Session| s.ticks),
        Some(tick + 4)
    );
    runtime.shutdown();
}
