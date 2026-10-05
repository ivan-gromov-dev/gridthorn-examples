use super::*;

#[path = "locale_workload.rs"]
mod locale;

#[test]
fn slider_workload_drags_at_both_dpis_and_releases_capture() {
    for dpi in [1_u32, 2] {
        let mut state = Interface::new().unwrap();
        let viewport = gridthorn::WindowViewport {
            width: 1000 * dpi,
            height: 800 * dpi,
        };
        state.prepare(viewport, f64::from(dpi)).unwrap();
        assert_eq!(state.slider_step(10, f64::from(dpi)).unwrap().effects, []);
        for frame in [11, 12, 119, 120] {
            state.prepare(viewport, f64::from(dpi)).unwrap();
            let route = state.slider_step(frame, f64::from(dpi)).unwrap();
            assert!(
                route
                    .effects
                    .contains(&(UiNodeId(5), gridthorn::ui::UiEffect::Changed))
            );
            let UiControl::Slider { value, .. } = state.tree.node(UiNodeId(5)).unwrap().control
            else {
                panic!("slider");
            };
            let expected = if frame % 2 == 0 { 20.0 } else { 80.0 };
            assert!((value - expected).abs() < 0.01);
            for (id, effect) in route.effects {
                state.effect(id, effect).unwrap();
            }
        }
        state.prepare(viewport, f64::from(dpi)).unwrap();
        let layout = state.layout.as_ref().unwrap();
        let content = layout.placement(UiNodeId(5)).unwrap().content;
        let route = state
            .router
            .route_events(
                &mut state.tree,
                layout,
                &[InputEvent::CursorMoved(CursorPosition {
                    x: f64::from(content.position[0] + content.size[0] * 0.8) * f64::from(dpi),
                    y: f64::from(content.position[1] + content.size[1] * 0.5) * f64::from(dpi),
                })],
            )
            .unwrap();
        assert!(
            !route
                .effects
                .contains(&(UiNodeId(5), gridthorn::ui::UiEffect::Changed))
        );
    }
}

#[test]
fn editing_workload_focuses_and_replaces_bounded_multilingual_text() {
    let mut state = Interface::new().unwrap();
    let viewport = gridthorn::WindowViewport {
        width: 1000,
        height: 800,
    };
    state.prepare(viewport, 1.0).unwrap();
    state.editing_step(1).unwrap();
    assert_eq!(state.router.focused(), Some(UiNodeId(7)));
    for frame in [11, 120] {
        state.prepare(viewport, 1.0).unwrap();
        let route = state.editing_step(frame).unwrap();
        assert!(
            route
                .effects
                .contains(&(UiNodeId(7), gridthorn::ui::UiEffect::Changed))
        );
        let UiControl::TextField { value, .. } = &state.tree.node(UiNodeId(7)).unwrap().control
        else {
            panic!("field");
        };
        assert_eq!(value, &format!("Привет مرحبًا 日本語 e\u{301} {frame}"));
    }
}
