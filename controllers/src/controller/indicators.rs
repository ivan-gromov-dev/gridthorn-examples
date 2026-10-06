use super::model::Devices;
use gridthorn::controller::{ControllerAxis, ControllerButton, ControllerState};
use gridthorn::ui::{UiLayout, UiNodeId};
use gridthorn::{Color, InputState, KeyCode, MouseButton, TextLabel, UiPrimitive, UiRect};
fn accent() -> Color {
    Color::rgb(0.2, 0.85, 0.65)
}
fn dim() -> Color {
    Color::rgb(0.13, 0.18, 0.25)
}
/// Indicators are presentation-only and use the selected connection's immutable state.
pub(super) fn paint(
    layout: &UiLayout,
    input: &InputState,
    devices: &Devices,
    dpi: f32,
) -> Vec<UiPrimitive> {
    let mut output = Vec::new();
    let Some(panel) = layout.placement(UiNodeId(10)) else {
        return output;
    };
    let origin = panel.bounds.position;
    let mut canvas = Canvas {
        output: &mut output,
        origin,
        dpi,
    };
    let state = devices.selected.and_then(|id| input.controller(id));
    if let Some(state) = state {
        controller(&mut canvas, state);
    } else {
        desktop(&mut canvas, input);
    }
    let clip = panel.clip;
    vec![UiPrimitive::Clipped {
        bounds: UiRect::new(
            [clip.position[0] * dpi, clip.position[1] * dpi],
            [clip.size[0].max(1.0) * dpi, clip.size[1].max(1.0) * dpi],
            dim(),
        )
        .expect("valid clip"),
        children: output,
    }]
}
const BUTTONS: &[(ControllerButton, &str)] = &[
    (ControllerButton::South, "SOUTH"),
    (ControllerButton::East, "EAST"),
    (ControllerButton::West, "WEST"),
    (ControllerButton::North, "NORTH"),
    (ControllerButton::LeftShoulder, "LB"),
    (ControllerButton::RightShoulder, "RB"),
    (ControllerButton::LeftTrigger, "LT"),
    (ControllerButton::RightTrigger, "RT"),
    (ControllerButton::Mode, "HOME"),
    (ControllerButton::DPadUp, "UP"),
    (ControllerButton::DPadDown, "DOWN"),
    (ControllerButton::DPadLeft, "LEFT"),
    (ControllerButton::DPadRight, "RIGHT"),
    (ControllerButton::LeftStick, "L CLICK"),
    (ControllerButton::RightStick, "R CLICK"),
    (ControllerButton::Select, "SELECT"),
    (ControllerButton::Start, "START"),
];
struct Canvas<'a> {
    output: &'a mut Vec<UiPrimitive>,
    origin: [f32; 2],
    dpi: f32,
}
impl Canvas<'_> {
    fn rect(&mut self, point: [f32; 2], size: [f32; 2], color: Color) {
        self.output.push(
            UiRect::new(
                [
                    (self.origin[0] + point[0]) * self.dpi,
                    (self.origin[1] + point[1]) * self.dpi,
                ],
                [size[0] * self.dpi, size[1] * self.dpi],
                color,
            )
            .expect("valid indicator rectangle")
            .into(),
        );
    }
    fn text(&mut self, point: [f32; 2], label: &str) {
        self.output.push(
            TextLabel::new(
                label,
                [
                    (self.origin[0] + point[0]) * self.dpi,
                    (self.origin[1] + point[1]) * self.dpi,
                ],
                1.5 * self.dpi,
                Color::rgb(0.9, 0.95, 1.0),
            )
            .expect("valid indicator label")
            .into(),
        );
    }
}
fn stick(
    canvas: &mut Canvas<'_>,
    state: &ControllerState,
    point: [f32; 2],
    x: ControllerAxis,
    y: ControllerAxis,
    label: &str,
) {
    let value = [state.axis(x), state.axis(y)];
    canvas.text([point[0], point[1] - 20.0], label);
    canvas.rect(point, [100.0; 2], dim());
    canvas.rect(
        [point[0] + 49.0, point[1]],
        [2.0, 100.0],
        Color::rgb(0.3, 0.4, 0.5),
    );
    canvas.rect(
        [point[0], point[1] + 49.0],
        [100.0, 2.0],
        Color::rgb(0.3, 0.4, 0.5),
    );
    canvas.rect(
        [
            point[0] + 46.0 + value[0] * 44.0,
            point[1] + 46.0 - value[1] * 44.0,
        ],
        [8.0; 2],
        accent(),
    );
    canvas.text(
        [point[0], point[1] + 110.0],
        &format!("X {:+.2} Y {:+.2}", value[0], value[1]),
    );
}

fn controller(canvas: &mut Canvas<'_>, state: &ControllerState) {
    canvas.text(
        [12.0, 8.0],
        &format!("{}  /  #{}", state.info().name, state.info().id.0),
    );
    for (index, (button, label)) in BUTTONS.iter().enumerate() {
        let column = u16::try_from(index % 9).expect("nine columns");
        let row = u16::try_from(index / 9).expect("two rows");
        let point = [
            12.0 + f32::from(column) * 86.0,
            42.0 + f32::from(row) * 42.0,
        ];
        canvas.rect(
            point,
            [78.0, 32.0],
            if state.button_down(*button) {
                accent()
            } else {
                dim()
            },
        );
        canvas.text([point[0] + 6.0, point[1] + 10.0], label);
    }
    stick(
        canvas,
        state,
        [15.0, 155.0],
        ControllerAxis::LeftStickX,
        ControllerAxis::LeftStickY,
        "LEFT STICK",
    );
    stick(
        canvas,
        state,
        [260.0, 155.0],
        ControllerAxis::RightStickX,
        ControllerAxis::RightStickY,
        "RIGHT STICK",
    );
    for (axis, label, y) in [
        (ControllerAxis::LeftTrigger, "LT", 170.0),
        (ControllerAxis::RightTrigger, "RT", 235.0),
    ] {
        let value = state.axis(axis);
        canvas.text([510.0, y], &format!("{label}  {value:.2}"));
        canvas.rect([510.0, y + 22.0], [200.0, 20.0], dim());
        if value > 0.0 {
            canvas.rect([510.0, y + 22.0], [200.0 * value, 20.0], accent());
        }
    }
}
fn desktop(canvas: &mut Canvas<'_>, input: &InputState) {
    canvas.text([12.0, 15.0], "KEYBOARD + MOUSE");
    for (index, (key, label)) in [
        (KeyCode::KeyW, "W"),
        (KeyCode::KeyA, "A"),
        (KeyCode::KeyS, "S"),
        (KeyCode::KeyD, "D"),
        (KeyCode::Space, "SPACE"),
        (KeyCode::Enter, "ENTER"),
    ]
    .iter()
    .enumerate()
    {
        let x = 12.0 + f32::from(u16::try_from(index).unwrap()) * 95.0;
        canvas.rect(
            [x, 55.0],
            [85.0, 38.0],
            if input.key_down(*key) {
                accent()
            } else {
                dim()
            },
        );
        canvas.text([x + 8.0, 68.0], label);
    }
    for (index, (button, label)) in [
        (MouseButton::Left, "LEFT"),
        (MouseButton::Middle, "MIDDLE"),
        (MouseButton::Right, "RIGHT"),
    ]
    .iter()
    .enumerate()
    {
        let x = 12.0 + f32::from(u16::try_from(index).unwrap()) * 125.0;
        canvas.rect(
            [x, 115.0],
            [110.0, 38.0],
            if input.mouse_button_down(*button) {
                accent()
            } else {
                dim()
            },
        );
        canvas.text([x + 8.0, 128.0], label);
    }
    if let Some(cursor) = input.cursor_position() {
        canvas.text(
            [12.0, 180.0],
            &format!("POINTER  X {:.0}  Y {:.0}", cursor.x, cursor.y),
        );
    }
    canvas.text(
        [12.0, 230.0],
        "CONNECT A CONTROLLER TO ADD IT TO THE DEVICE LIST",
    );
}
