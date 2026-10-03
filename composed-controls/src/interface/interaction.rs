use gridthorn::ui::UiPlatformRequest;
use gridthorn::{Clipboard, ClipboardOperation, TextInput, TextInputRequest, WorldAccess};

pub(super) fn apply_platform(world: &mut WorldAccess<'_>, requests: Vec<UiPlatformRequest>) {
    for request in requests {
        match request {
            UiPlatformRequest::Text(TextInputRequest::Start(area)) => {
                world.update_resource(|text: &mut TextInput| {
                    text.start(area).expect("validated UI anchor");
                });
            }
            UiPlatformRequest::Text(TextInputRequest::Stop) => {
                world.update_resource(TextInput::stop);
            }
            UiPlatformRequest::Clipboard(request) => {
                world.update_resource(|clipboard: &mut Clipboard| match request.operation {
                    ClipboardOperation::Read => clipboard.read(request.id),
                    ClipboardOperation::Write(text) => clipboard.write(request.id, text),
                });
            }
        }
    }
}

pub(super) fn headless() -> Result<(), Box<dyn std::error::Error>> {
    use gridthorn::ui::{UiControl, UiEffect, UiNavigation, UiNodeId, UiRouter, UiSelection};
    use gridthorn::{ButtonState, CursorPosition, InputEvent, MouseButton, TextInputEvent};
    let mut tree = super::composition::tree()?;
    let mut fonts = super::composition::fonts()?;
    let mut router = UiRouter::new(10_000);
    let layout = tree.layout([900.0, 700.0], 2.0, Some(&mut fonts))?;
    let button = layout.placement(UiNodeId(3)).expect("button").bounds;
    let route = router.route_events(
        &mut tree,
        &layout,
        &[
            InputEvent::CursorMoved(CursorPosition {
                x: f64::from(button.position[0] + 10.0) * 2.0,
                y: f64::from(button.position[1] + 10.0) * 2.0,
            }),
            InputEvent::MouseButton {
                button: MouseButton::Left,
                state: ButtonState::Pressed,
            },
            InputEvent::MouseButton {
                button: MouseButton::Left,
                state: ButtonState::Released,
            },
        ],
    )?;
    assert_eq!(route.effects, [(UiNodeId(3), UiEffect::Activated)]);
    assert_eq!(route.world_events, []);
    assert!(route.keyboard_blocked && route.pointer_blocked);
    for _ in 0..4 {
        router.navigate(&mut tree, &layout, UiNavigation::Next)?;
    }
    assert_eq!(router.focused(), Some(UiNodeId(7)));
    router.route_events(
        &mut tree,
        &layout,
        &[
            InputEvent::Text(TextInputEvent::Commit(
                "Привет العربية 日本語 e\u{301}".into(),
            )),
            InputEvent::Text(TextInputEvent::Composition {
                text: "にほん".into(),
                cursor: Some((9, 9)),
            }),
        ],
    )?;
    assert_eq!(router.preedit(), "にほん");
    let painted = router.layout(&tree, [900.0, 700.0], 2.0, Some(&mut fonts))?;
    assert_ne!(painted.primitives(), layout.primitives());
    router.route_events(
        &mut tree,
        &painted,
        &[
            InputEvent::Text(TextInputEvent::CompositionCancelled),
            InputEvent::Text(TextInputEvent::Commit("日本".into())),
        ],
    )?;
    let UiControl::TextField { value, .. } = &tree.node(UiNodeId(7)).expect("field").control else {
        unreachable!();
    };
    router.select(
        &tree,
        UiSelection {
            anchor: value.len(),
            caret: 0,
        },
    )?;
    router.route_events(
        &mut tree,
        &painted,
        &[InputEvent::Text(TextInputEvent::Commit(
            "selected replacement".into(),
        ))],
    )?;
    assert!(
        matches!(&tree.node(UiNodeId(7)).expect("field").control, UiControl::TextField { value, .. } if value == "selected replacement")
    );
    let route = router.route_events(&mut tree, &painted, &[InputEvent::FocusLost])?;
    assert_eq!(router.focused(), None);
    assert!(!route.keyboard_blocked);
    assert_eq!(route.world_events, [InputEvent::FocusLost]);
    println!(
        "UI routing: click, hooks, Unicode selection, preedit, DPI, focus cancellation passed"
    );
    Ok(())
}
