use super::*;

#[test]
fn wheel_workload_scrolls_and_returns_without_editing_controls_at_both_dpis() {
    for dpi in [1_u32, 2] {
        let mut state = Interface::new().unwrap();
        let viewport = gridthorn::WindowViewport {
            width: 1000 * dpi,
            height: 400 * dpi,
        };
        state.prepare(viewport, f64::from(dpi)).unwrap();
        let editor = state.tree.node(UiNodeId(7)).unwrap().control.clone();
        let language = state.tree.node(UiNodeId(6)).unwrap().control.clone();
        assert_eq!(state.scroll_step(10, f64::from(dpi)).unwrap().effects, []);
        for frame in [11, 12, 119, 120] {
            let route = state.scroll_step(frame, f64::from(dpi)).unwrap();
            assert!(
                route
                    .world_events
                    .iter()
                    .all(|event| !matches!(event, InputEvent::MouseWheel { .. }))
            );
            assert!(state.prepare(viewport, f64::from(dpi)).unwrap());
            let offset = state
                .layout
                .as_ref()
                .unwrap()
                .placement(UiNodeId(0))
                .unwrap()
                .scroll_offset[1];
            if frame % 2 == 0 {
                assert!(offset.abs() < 0.01);
            } else {
                assert!(offset > 0.0 && offset <= 96.0);
            }
            assert_eq!(state.tree.node(UiNodeId(7)).unwrap().control, editor);
            assert_eq!(state.tree.node(UiNodeId(6)).unwrap().control, language);
        }
    }
}

#[test]
fn nonoverflowing_viewport_is_rejected_instead_of_measuring_idle() {
    let mut state = Interface::new().unwrap();
    state
        .prepare(
            gridthorn::WindowViewport {
                width: 1000,
                height: 4000,
            },
            1.0,
        )
        .unwrap();
    assert!(state.scroll_step(11, 1.0).is_err());
}
