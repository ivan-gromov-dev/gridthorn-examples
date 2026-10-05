use super::super::presentation::Menu;
use gridthorn::WindowViewport;

#[test]
fn unchanged_frames_reuse_prepared_layout_and_resize_invalidates_it() {
    let mut menu = Menu::new().unwrap();
    let viewport = WindowViewport {
        width: 1100,
        height: 860,
    };
    assert!(menu.prepare(viewport, 1.0).unwrap().is_some());
    let layouts = menu.layouts;
    for _ in 0..120 {
        assert!(menu.prepare(viewport, 1.0).unwrap().is_none());
    }
    assert_eq!(menu.layouts, layouts);
    assert!(menu.prepare(viewport, 1.25).unwrap().is_some());
    assert!(
        menu.prepare(
            WindowViewport {
                width: 760,
                height: 560
            },
            1.0
        )
        .unwrap()
        .is_some()
    );
    assert!(
        menu.prepare(
            WindowViewport {
                width: 0,
                height: 0
            },
            1.0
        )
        .unwrap()
        .is_some()
    );
}
