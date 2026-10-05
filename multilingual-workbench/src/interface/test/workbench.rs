use super::super::runtime::Interface;
use gridthorn::ui::{UiCommand, UiControl, UiEffect, UiNodeId};
use gridthorn::{ButtonState, CursorPosition, InputEvent, MouseButton, WindowViewport};

#[test]
fn public_workbench_flow() {
    super::run().expect("public workbench workflow");
}

#[test]
fn language_switch_preserves_editor_and_changes_plural_caption() {
    let mut state = Interface::new().unwrap();
    state
        .tree
        .command(
            UiNodeId(7),
            UiCommand::SetText("Иван العربية 日本語".into()),
        )
        .unwrap();
    state
        .tree
        .command(UiNodeId(5), UiCommand::SetValue(21.0))
        .unwrap();
    state
        .tree
        .command(UiNodeId(6), UiCommand::Select(Some(1)))
        .unwrap();
    state.effect(UiNodeId(6), UiEffect::Changed).unwrap();
    assert!(
        matches!(&state.tree.node(UiNodeId(9)).unwrap().control, UiControl::Label(value) if value.replace(['\u{2068}', '\u{2069}'], "").contains("21 работник"))
    );
    assert!(
        matches!(&state.tree.node(UiNodeId(3)).unwrap().control, UiControl::Button(value) if value == "Открыть проверку")
    );
    assert!(
        matches!(&state.tree.node(UiNodeId(7)).unwrap().control, UiControl::TextField { value, .. } if value == "Иван العربية 日本語")
    );
}

#[test]
fn routed_review_and_outside_context_dismissal_restore_parent_scope() {
    let mut state = Interface::new().unwrap();
    let viewport = WindowViewport {
        width: 1000,
        height: 800,
    };
    state.prepare(viewport, 1.0).unwrap();
    let bounds = state
        .layout
        .as_ref()
        .unwrap()
        .placement(UiNodeId(3))
        .unwrap()
        .bounds;
    let route = state
        .router
        .route_events(
            &mut state.tree,
            state.layout.as_ref().unwrap(),
            &[
                InputEvent::CursorMoved(CursorPosition {
                    x: f64::from(bounds.position[0] + 12.0),
                    y: f64::from(bounds.position[1] + 12.0),
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
        )
        .unwrap();
    assert_eq!(route.effects, [(UiNodeId(3), UiEffect::Activated)]);
    assert_eq!(route.world_events, []);
    for (id, effect) in route.effects {
        state.effect(id, effect).unwrap();
    }
    state.prepare(viewport, 1.0).unwrap();
    state.effect(UiNodeId(203), UiEffect::Activated).unwrap();
    state.prepare(viewport, 1.0).unwrap();
    let route = state
        .router
        .route_events(
            &mut state.tree,
            state.layout.as_ref().unwrap(),
            &[
                InputEvent::CursorMoved(CursorPosition { x: 1.0, y: 1.0 }),
                InputEvent::MouseButton {
                    button: MouseButton::Left,
                    state: ButtonState::Pressed,
                },
                InputEvent::MouseButton {
                    button: MouseButton::Left,
                    state: ButtonState::Released,
                },
            ],
        )
        .unwrap();
    assert_eq!(route.dismissed, [UiNodeId(400)]);
    assert_eq!(route.effects, []);
    assert_eq!(route.world_events, []);
    assert_eq!(state.router.open_layers(), [UiNodeId(200)]);
    assert!(route.keyboard_blocked && route.pointer_blocked);
}

#[test]
fn prepared_layout_is_reused_until_visible_state_changes() {
    let mut state = Interface::new().unwrap();
    let viewport = WindowViewport {
        width: 1000,
        height: 800,
    };
    assert!(state.prepare(viewport, 1.0).unwrap());
    for _ in 0..60 {
        let route = state
            .router
            .route_events(&mut state.tree, state.layout.as_ref().unwrap(), &[])
            .unwrap();
        assert_eq!(route.effects, []);
        assert!(!state.prepare(viewport, 1.0).unwrap());
    }
    state
        .tree
        .command(UiNodeId(5), UiCommand::SetValue(21.0))
        .unwrap();
    state.effect(UiNodeId(5), UiEffect::Changed).unwrap();
    assert!(state.prepare(viewport, 1.0).unwrap());
    assert!(!state.prepare(viewport, 1.0).unwrap());
    state
        .tree
        .command(UiNodeId(7), UiCommand::SetText("Новый текст 日本語".into()))
        .unwrap();
    assert!(state.prepare(viewport, 1.0).unwrap());
    assert!(state.prepare(viewport, 2.0).unwrap());
    assert!(!state.prepare(viewport, 2.0).unwrap());
    assert!(
        state
            .prepare(
                WindowViewport {
                    width: 800,
                    height: 600
                },
                2.0
            )
            .unwrap()
    );
    state.effect(UiNodeId(3), UiEffect::Activated).unwrap();
    assert!(state.prepare(viewport, 1.0).unwrap());
    assert!(!state.prepare(viewport, 1.0).unwrap());
}

#[test]
fn editor_selection_and_preedit_invalidate_cached_paint() {
    use gridthorn::TextInputEvent;
    use gridthorn::ui::{UiNavigation, UiSelection};
    let mut state = Interface::new().unwrap();
    let viewport = WindowViewport {
        width: 1000,
        height: 800,
    };
    state.prepare(viewport, 1.0).unwrap();
    for _ in 0..5 {
        state
            .router
            .navigate(
                &mut state.tree,
                state.layout.as_ref().unwrap(),
                UiNavigation::Next,
            )
            .unwrap();
    }
    assert_eq!(state.router.focused(), Some(UiNodeId(7)));
    assert!(state.prepare(viewport, 1.0).unwrap());
    state
        .router
        .select(
            &state.tree,
            UiSelection {
                anchor: 0,
                caret: 0,
            },
        )
        .unwrap();
    assert!(state.prepare(viewport, 1.0).unwrap());
    state
        .router
        .route_events(
            &mut state.tree,
            state.layout.as_ref().unwrap(),
            &[InputEvent::Text(TextInputEvent::Composition {
                text: "にほん".into(),
                cursor: Some((9, 9)),
            })],
        )
        .unwrap();
    assert!(state.prepare(viewport, 1.0).unwrap());
    assert!(!state.prepare(viewport, 1.0).unwrap());
    state
        .router
        .route_events(
            &mut state.tree,
            state.layout.as_ref().unwrap(),
            &[InputEvent::FocusLost],
        )
        .unwrap();
    assert!(state.prepare(viewport, 1.0).unwrap());
    assert!(!state.prepare(viewport, 1.0).unwrap());
}
