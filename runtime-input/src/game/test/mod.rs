use gridthorn::{ButtonState, InputBuffer, InputEvent, KeyCode};

use super::{model::Controlled, model::Position, runtime};

#[test]
fn held_input_moves_the_world_entity_on_every_fixed_tick() {
    let mut runtime = runtime(false);
    let mut input = InputBuffer::new();
    input.push(InputEvent::Keyboard {
        key: KeyCode::KeyD,
        state: ButtonState::Pressed,
    });
    runtime.world().insert_resource(input.snapshot());

    runtime.run_frame(3).expect("controlled frame should run");

    let entity = runtime
        .world()
        .read_resource(|controlled: &Controlled| controlled.0)
        .expect("startup should create a controlled entity");
    assert_eq!(
        runtime
            .world()
            .read_component(entity, |position: &Position| *position),
        Some(Position { x: 3, y: 0 })
    );
}

#[test]
fn reset_command_is_consumed_once_before_continuous_movement() {
    let mut runtime = runtime(false);
    let mut input = InputBuffer::new();
    input.push(InputEvent::Keyboard {
        key: KeyCode::KeyD,
        state: ButtonState::Pressed,
    });
    runtime.world().insert_resource(input.snapshot());
    runtime.run_frame(3).expect("movement frame should run");
    input.push(InputEvent::Keyboard {
        key: KeyCode::KeyD,
        state: ButtonState::Released,
    });
    input.push(InputEvent::Keyboard {
        key: KeyCode::Space,
        state: ButtonState::Pressed,
    });
    runtime.world().insert_resource(input.snapshot());

    runtime.run_frame(2).expect("reset frame should run");

    let entity = runtime
        .world()
        .read_resource(|controlled: &Controlled| controlled.0)
        .expect("startup should create a controlled entity");
    assert_eq!(
        runtime
            .world()
            .read_component(entity, |position: &Position| *position),
        Some(Position::default())
    );
}
