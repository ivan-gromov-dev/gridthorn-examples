use super::super::{composition, presentation::Menu};
use gridthorn::{
    WindowViewport,
    window::{WindowCapabilities, WindowModeKind},
};

#[test]
fn graphics_choices_are_staged_and_busy_controls_do_not_change_them() {
    let mut menu = Menu::new().unwrap();
    menu.model.waiting = false;
    menu.model.graphics.capabilities = WindowCapabilities {
        available: true,
        size: true,
        resize_policy: true,
        borderless: true,
        ..WindowCapabilities::default()
    };
    menu.rebuild().unwrap();
    let viewport = WindowViewport {
        width: 1100,
        height: 860,
    };
    menu.prepare(viewport, 1.0).unwrap();
    let initial = menu.model.graphics.size;
    assert_eq!(menu.click(composition::SIZE, 0, 1.0).unwrap(), []);
    assert_ne!(menu.model.graphics.size, initial);
    menu.prepare(viewport, 1.0).unwrap();
    assert_eq!(menu.click(composition::RESIZABLE, 0, 1.0).unwrap(), []);
    assert!(!menu.model.graphics.resizable);
    menu.prepare(viewport, 1.0).unwrap();
    assert_eq!(menu.click(composition::MODE, 0, 1.0).unwrap(), []);
    assert_eq!(menu.model.graphics.mode, WindowModeKind::Borderless);
    menu.model.applying = true;
    menu.rebuild().unwrap();
    menu.prepare(viewport, 1.0).unwrap();
    assert_eq!(menu.click(composition::MODE, 0, 1.0).unwrap(), []);
    assert_eq!(menu.model.graphics.mode, WindowModeKind::Borderless);
    menu.model.applying = false;
    menu.rebuild().unwrap();
    menu.prepare(viewport, 1.0).unwrap();
    menu.click(composition::MODE, 0, 1.0).unwrap();
    assert_eq!(
        menu.model.graphics.mode,
        WindowModeKind::Windowed,
        "unsupported exclusive must not be offered"
    );
}
