use super::*;

#[test]
fn locale_workload_cycles_captions_and_preserves_editor() {
    let mut state = Interface::new().unwrap();
    let viewport = gridthorn::WindowViewport {
        width: 2000,
        height: 1600,
    };
    state.prepare(viewport, 2.0).unwrap();
    let initial_editor = state.tree.node(UiNodeId(7)).unwrap().control.clone();
    let initial_caption = state.tree.node(UiNodeId(3)).unwrap().control.clone();
    assert_eq!(state.locale_step(10).unwrap().effects, []);
    for (frame, expected) in [(11, 1), (12, 2), (13, 3), (14, 0)] {
        let route = state.locale_step(frame).unwrap();
        assert_eq!(
            route.effects,
            [(UiNodeId(6), gridthorn::ui::UiEffect::Changed)]
        );
        for (id, effect) in route.effects {
            state.effect(id, effect).unwrap();
        }
        state.prepare(viewport, 2.0).unwrap();
        let UiControl::List { selected, .. } = state.tree.node(UiNodeId(6)).unwrap().control else {
            panic!("language list");
        };
        assert_eq!(selected, Some(expected));
        assert_eq!(
            state.tree.node(UiNodeId(7)).unwrap().control,
            initial_editor
        );
        if expected == 0 {
            assert_eq!(
                state.tree.node(UiNodeId(3)).unwrap().control,
                initial_caption
            );
        } else {
            assert_ne!(
                state.tree.node(UiNodeId(3)).unwrap().control,
                initial_caption
            );
        }
    }
}
