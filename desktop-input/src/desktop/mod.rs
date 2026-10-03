mod smoke;
use gridthorn::{
    ApplicationRuntime, ButtonState, ExitRequest, InputBuffer, InputEvent, InputState, KeyCode,
    KeyLocation, KeyboardEvent, LogicalKey, Modifiers, PhysicalKey, PointerCapture,
    PointerCaptureMode, ScheduleBuilder, ScheduleStage, ScrollPhase, WheelDelta,
};

/// Build a native event monitor using only public SDK contracts.
pub(crate) fn runtime(smoke: bool) -> ApplicationRuntime {
    let mut schedules = ScheduleBuilder::new();
    schedules.add_system(ScheduleStage::Input, move |world| {
        let input = world
            .read_resource(Clone::clone)
            .unwrap_or_else(InputState::default);
        for event in input.events() {
            println!("{event:?}");
        }
        for (key, mode) in [
            (KeyCode::F1, PointerCaptureMode::Confined),
            (KeyCode::F2, PointerCaptureMode::Locked),
            (KeyCode::F3, PointerCaptureMode::None),
        ] {
            if input.key_just_pressed(key) {
                world.update_resource(|capture: &mut PointerCapture| capture.request(mode));
            }
        }
        if input.key_just_pressed(KeyCode::Escape) {
            world.update_resource(|exit: &mut ExitRequest| exit.request());
        }
    });
    if smoke {
        smoke::register(&mut schedules);
    }
    ApplicationRuntime::new(schedules.build())
}

/// Exercise layout identity, repeat, order, wheel and cancellation without a native window.
pub(crate) fn headless() {
    let mut buffer = InputBuffer::new();
    buffer.push(InputEvent::FocusGained);
    buffer.push(InputEvent::ModifiersChanged(Modifiers {
        shift: true,
        ..Modifiers::default()
    }));
    for (state, repeat) in [
        (ButtonState::Pressed, false),
        (ButtonState::Pressed, true),
        (ButtonState::Released, false),
    ] {
        buffer.push(InputEvent::Key(KeyboardEvent {
            physical_key: PhysicalKey::Code(KeyCode::KeyQ),
            logical_key: LogicalKey::Character("Й".into()),
            location: KeyLocation::Standard,
            state,
            repeat,
            synthetic: false,
        }));
    }
    buffer.push(InputEvent::MouseWheel {
        delta: WheelDelta::Pixels { x: 0.5, y: -12.5 },
        phase: ScrollPhase::Moved,
    });
    let frame = buffer.snapshot();
    assert_eq!(frame.events().len(), 6);
    assert!(frame.key_just_pressed(KeyCode::KeyQ));
    assert!(frame.key_just_released(KeyCode::KeyQ));
    assert!(frame.modifiers().shift);
    for event in frame.events() {
        println!("{event:?}");
    }
    buffer.push(InputEvent::FocusLost);
    assert_eq!(buffer.snapshot().modifiers(), Modifiers::default());
    assert_eq!(buffer.snapshot().events(), []);
    let mut capture = PointerCapture::default();
    capture.request(PointerCaptureMode::Confined);
    capture.request(PointerCaptureMode::None);
    assert_eq!(capture.take_request(), Some(PointerCaptureMode::None));
    println!("Desktop input headless smoke passed");
}
