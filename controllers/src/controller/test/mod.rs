use super::model::Devices;
use gridthorn::controller::{ControllerEvent, ControllerId, ControllerInfo};
use gridthorn::{InputBuffer, InputEvent};
fn info(id: u64) -> ControllerInfo {
    ControllerInfo {
        id: ControllerId(id),
        name: "same model".into(),
        model_uuid: [0; 16],
        vendor_id: None,
        product_id: None,
        buttons: vec![],
        axes: vec![],
        rumble_supported: false,
    }
}
#[test]
fn inventory_excludes_unplugged_controllers_and_selection_tracks_connection_identity() {
    let mut devices = Devices::default();
    let mut input = InputBuffer::new();
    devices.sync(&input.snapshot());
    assert_eq!(devices.items(), ["Клавиатура и мышь"]);
    for id in [1, 2] {
        input.push(InputEvent::Controller(ControllerEvent::Connected(info(id))));
    }
    devices.sync(&input.snapshot());
    assert_eq!(devices.items().len(), 3);
    devices.select(2);
    input.push(InputEvent::Controller(ControllerEvent::Disconnected(
        ControllerId(1),
    )));
    devices.sync(&input.snapshot());
    assert_eq!(devices.selected, Some(ControllerId(2)));
    assert_eq!(devices.row(), 1);
    input.push(InputEvent::Controller(ControllerEvent::Disconnected(
        ControllerId(2),
    )));
    devices.sync(&input.snapshot());
    assert_eq!(devices.selected, None);
    assert_eq!(devices.items(), ["Клавиатура и мышь"]);
}
#[test]
fn public_headless_navigation_and_live_indicator_workflow() {
    super::headless().unwrap();
}

#[test]
fn stick_highlights_keyboard_without_switching_until_confirmation() {
    use super::{composition, runtime::Monitor};
    use gridthorn::controller::{ControllerAxis, ControllerButton};
    use gridthorn::ui::{UiControl, UiNodeId};
    use gridthorn::{ButtonState, CursorPosition, MouseButton};

    let mut input = InputBuffer::new();
    input.push(InputEvent::Controller(ControllerEvent::Connected(info(1))));
    let mut monitor = Monitor::new().unwrap();
    let mut fonts = composition::fonts().unwrap();
    monitor
        .frame(&input.snapshot(), [1000.0, 900.0], 1.0)
        .unwrap();
    let layout = monitor
        .tree
        .layout([1000.0, 900.0], 1.0, Some(&mut fonts))
        .unwrap();
    let list = layout.placement(UiNodeId(2)).unwrap().content;
    input.push(InputEvent::CursorMoved(CursorPosition {
        x: f64::from(list.position[0] + 10.0),
        y: f64::from(list.position[1] + 48.0),
    }));
    for state in [ButtonState::Pressed, ButtonState::Released] {
        input.push(InputEvent::MouseButton {
            button: MouseButton::Left,
            state,
        });
    }
    monitor
        .frame(&input.snapshot(), [1000.0, 900.0], 1.0)
        .unwrap();
    assert_eq!(monitor.devices.selected, Some(ControllerId(1)));
    input.push(InputEvent::Controller(ControllerEvent::Axis {
        id: ControllerId(1),
        axis: ControllerAxis::LeftStickY,
        value: 0.8,
    }));
    monitor
        .frame(&input.snapshot(), [1000.0, 900.0], 1.0)
        .unwrap();
    assert_eq!(monitor.devices.selected, Some(ControllerId(1)));
    assert!(matches!(
        &monitor.tree.node(UiNodeId(2)).unwrap().control,
        UiControl::List {
            selected: Some(0),
            ..
        }
    ));
    for _ in 0..3 {
        monitor
            .frame(&input.snapshot(), [1000.0, 900.0], 1.0)
            .unwrap();
    }
    assert_eq!(monitor.devices.selected, Some(ControllerId(1)));
    for button in [
        ControllerButton::RightShoulder,
        ControllerButton::RightShoulder,
        ControllerButton::RightShoulder,
        ControllerButton::South,
    ] {
        for state in [ButtonState::Pressed, ButtonState::Released] {
            input.push(InputEvent::Controller(ControllerEvent::Button {
                id: ControllerId(1),
                button,
                state,
            }));
        }
    }
    monitor
        .frame(&input.snapshot(), [1000.0, 900.0], 1.0)
        .unwrap();
    assert_eq!(monitor.devices.selected, None);
}

#[test]
fn indicators_follow_selected_device_button_pressure_and_stick_samples() {
    use crate::controller::{composition, indicators};
    use gridthorn::controller::{ControllerAxis, ControllerButton};
    use gridthorn::{ButtonState, UiPrimitive};
    let mut input = InputBuffer::new();
    for id in [1, 2] {
        input.push(InputEvent::Controller(ControllerEvent::Connected(info(id))));
    }
    let mut devices = Devices::default();
    devices.sync(&input.snapshot());
    devices.select(1);
    input.push(InputEvent::Controller(ControllerEvent::Button {
        id: ControllerId(1),
        button: ControllerButton::South,
        state: ButtonState::Pressed,
    }));
    for (axis, value) in [
        (ControllerAxis::LeftTrigger, 0.5),
        (ControllerAxis::LeftStickX, 1.0),
        (ControllerAxis::LeftStickY, 1.0),
    ] {
        input.push(InputEvent::Controller(ControllerEvent::Axis {
            id: ControllerId(1),
            axis,
            value,
        }));
    }
    let frame = input.snapshot();
    let tree = composition::tree(&devices).unwrap();
    let mut fonts = composition::fonts().unwrap();
    let layout = tree.layout([1000.0, 800.0], 1.0, Some(&mut fonts)).unwrap();
    let painted = indicators::paint(&layout, &frame, &devices, 1.0);
    let UiPrimitive::Clipped { children, .. } = &painted[0] else {
        panic!("clipped indicators");
    };
    assert!(
        children
            .iter()
            .any(|p| matches!(p, UiPrimitive::Text(label) if label.text() == "LT  0.50"))
    );
    assert!(
        children
            .iter()
            .any(|p| matches!(p, UiPrimitive::Text(label) if label.text() == "X +1.00 Y +1.00"))
    );
    devices.select(2);
    let neutral = indicators::paint(&layout, &frame, &devices, 1.0);
    assert_ne!(painted, neutral);
    input.push(InputEvent::FocusLost);
    devices.select(1);
    let canceled = indicators::paint(&layout, &input.snapshot(), &devices, 1.0);
    assert_ne!(painted, canceled);
}
