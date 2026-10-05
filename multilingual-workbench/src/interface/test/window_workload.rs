use super::*;

#[test]
fn window_modes_follow_identical_layer_sequences_without_changing_editor_or_locale() {
    for animated in [false, true] {
        let mut state = Interface::new().unwrap();
        let viewport = gridthorn::WindowViewport {
            width: 2000,
            height: 1600,
        };
        state.prepare(viewport, 2.0).unwrap();
        assert_eq!(state.window_step(1, animated).unwrap().effects, []);
        let editor = state.tree.node(UiNodeId(7)).unwrap().control.clone();
        let language = state.tree.node(UiNodeId(6)).unwrap().control.clone();
        for cycle in 0..2 {
            for (frame, layers) in [
                (11, vec![200]),
                (26, vec![200, 300]),
                (41, vec![200]),
                (56, vec![200, 400]),
                (71, vec![200]),
                (86, vec![]),
                (101, vec![200]),
                (116, vec![]),
            ] {
                state.prepare(viewport, 2.0).unwrap();
                let route = state.window_step(frame + cycle * 120, animated).unwrap();
                assert_eq!(route.effects.len(), 1);
                for (id, effect) in route.effects {
                    state.effect(id, effect).unwrap();
                }
                state.prepare(viewport, 2.0).unwrap();
                assert_eq!(
                    state.router.open_layers(),
                    layers.into_iter().map(UiNodeId).collect::<Vec<_>>()
                );
                if frame == 11 {
                    assert_eq!(
                        state.tree.node(UiNodeId(200)).unwrap().style.offset[0],
                        if animated { 120.0 } else { 80.0 }
                    );
                }
                assert_eq!(state.tree.node(UiNodeId(7)).unwrap().control, editor);
                assert_eq!(state.tree.node(UiNodeId(6)).unwrap().control, language);
            }
        }
    }
}
