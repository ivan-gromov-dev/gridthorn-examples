use super::*;

#[test]
fn vertical_keys_follow_screen_direction() {
    use gridthorn::{ButtonState, InputBuffer, InputEvent};
    for (key, expected_y) in [
        (KeyCode::KeyW, -3),
        (KeyCode::ArrowUp, -3),
        (KeyCode::KeyS, 3),
        (KeyCode::ArrowDown, 3),
    ] {
        let mut runtime = runtime(false, true).unwrap();
        let mut input = InputBuffer::new();
        input.push(InputEvent::Keyboard {
            key,
            state: ButtonState::Pressed,
        });
        runtime.world().insert_resource(input.snapshot());
        runtime.run_frame(1).unwrap();
        let entity = runtime
            .world()
            .read_resource(|s: &Session| s.entity.unwrap())
            .unwrap();
        assert_eq!(
            runtime
                .world()
                .read_component(entity, |a: &Arena| a.position),
            Some([0, expected_y])
        );
        runtime.shutdown();
    }
}

#[test]
fn pause_victory_cleanup_and_restart() {
    let mut runtime = runtime(false, false).unwrap();
    runtime.startup().unwrap();
    runtime
        .world()
        .update_resource(|s: &mut GameStateStack| s.request_set(state("play")));
    runtime
        .world()
        .update_resource(|s: &mut SceneController| s.request_switch(scene("arena")));
    runtime.run_frame(1).unwrap();
    let entity = runtime
        .world()
        .read_resource(|s: &Session| s.entity.unwrap())
        .unwrap();
    runtime
        .world()
        .update_resource(|s: &mut GameStateStack| s.request_push(state("paused")));
    runtime
        .world()
        .update_component(entity, |a: &mut Arena| a.position = model::GEMS[0]);
    runtime.run_frame(5).unwrap();
    assert_eq!(
        runtime
            .world()
            .read_component(entity, |a: &Arena| a.collected),
        Some([false; 4])
    );
    runtime
        .world()
        .update_resource(|s: &mut GameStateStack| s.request_pop().unwrap());
    runtime.run_frame(1).unwrap();
    assert_eq!(
        runtime
            .world()
            .read_component(entity, |a: &Arena| a.collected[0]),
        Some(true)
    );
    for gem in model::GEMS {
        runtime
            .world()
            .update_component(entity, |a: &mut Arena| a.position = gem);
        runtime.run_frame(1).unwrap();
    }
    runtime.run_frame(0).unwrap();
    assert!(
        runtime
            .world()
            .read_component(entity, |a: &Arena| a.clone())
            .is_none()
    );
    assert!(
        runtime
            .world()
            .read_resource(|s: &GameStateStack| s.current() == &state("won"))
            .unwrap()
    );
    runtime
        .world()
        .update_resource(|s: &mut GameStateStack| s.request_set(state("play")));
    runtime
        .world()
        .update_resource(|s: &mut SceneController| s.request_switch(scene("arena")));
    runtime.run_frame(1).unwrap();
    let fresh = runtime
        .world()
        .read_resource(|s: &Session| s.entity.unwrap())
        .unwrap();
    assert_eq!(
        runtime.world().read_component(fresh, Clone::clone),
        Some(Arena::default())
    );
    runtime.shutdown();
}

#[test]
fn headless_assets_and_presentation() {
    smoke().unwrap();
}

#[test]
fn mouse_start_and_keyboard_movement_use_public_input() {
    use gridthorn::{ButtonState, CursorPosition, InputBuffer, InputEvent, MouseButton};
    let mut runtime = runtime(false, false).unwrap();
    let mut input = InputBuffer::new();
    input.push(InputEvent::CursorMoved(CursorPosition {
        x: 40.0,
        y: 130.0,
    }));
    input.push(InputEvent::MouseButton {
        button: MouseButton::Left,
        state: ButtonState::Pressed,
    });
    runtime.world().insert_resource(input.snapshot());
    runtime.run_frame(0).unwrap();
    assert!(
        runtime
            .world()
            .read_resource(|s: &Session| s.entity.is_none())
            .unwrap()
    );
    input.push(InputEvent::MouseButton {
        button: MouseButton::Left,
        state: ButtonState::Released,
    });
    input.push(InputEvent::Keyboard {
        key: KeyCode::KeyD,
        state: ButtonState::Pressed,
    });
    runtime.world().insert_resource(input.snapshot());
    runtime.run_frame(1).unwrap();
    let entity = runtime
        .world()
        .read_resource(|s: &Session| s.entity.unwrap())
        .unwrap();
    assert_eq!(
        runtime
            .world()
            .read_component(entity, |a: &Arena| a.position),
        Some([3, 0])
    );
    runtime.shutdown();
}
