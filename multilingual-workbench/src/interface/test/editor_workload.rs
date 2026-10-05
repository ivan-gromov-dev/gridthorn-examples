use super::*;

#[test]
fn editor_modes_invalidate_paint_without_changing_committed_text_at_both_dpis() {
    for dpi in [1_u32, 2] {
        for preedit in [false, true] {
            let mut state = Interface::new().unwrap();
            let viewport = gridthorn::WindowViewport {
                width: 1000 * dpi,
                height: 800 * dpi,
            };
            state.prepare(viewport, f64::from(dpi)).unwrap();
            let initial = state.tree.node(UiNodeId(7)).unwrap().control.clone();
            state.editor_step(1, preedit).unwrap();
            assert_eq!(state.router.focused(), Some(UiNodeId(7)));
            for frame in [11, 12, 119, 120] {
                state.prepare(viewport, f64::from(dpi)).unwrap();
                let route = state.editor_step(frame, preedit).unwrap();
                assert_eq!(route.effects, []);
                assert_eq!(state.tree.node(UiNodeId(7)).unwrap().control, initial);
                assert!(state.prepare(viewport, f64::from(dpi)).unwrap());
                if preedit {
                    let expected = if frame == 120 {
                        ""
                    } else if frame % 2 == 0 {
                        "にほん"
                    } else {
                        "العربية e\u{301}"
                    };
                    assert_eq!(state.router.preedit(), expected);
                } else {
                    let UiControl::TextField { value, .. } = &initial else {
                        panic!("editor");
                    };
                    assert_eq!(
                        state.router.selection(&state.tree),
                        Some(UiSelection {
                            anchor: 0,
                            caret: if frame % 2 == 0 { value.len() } else { 0 }
                        })
                    );
                }
            }
        }
    }
}
