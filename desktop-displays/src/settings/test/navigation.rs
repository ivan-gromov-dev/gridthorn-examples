use super::super::{
    composition,
    model::{Request, Section},
    presentation::Menu,
};
use gridthorn::WindowViewport;

#[test]
fn tabs_and_refresh_request_native_inventory_only_on_graphics_entry() {
    let mut menu = Menu::new().unwrap();
    menu.model.waiting = false;
    menu.rebuild().unwrap();
    let viewport = WindowViewport {
        width: 1100,
        height: 860,
    };
    menu.prepare(viewport, 1.0).unwrap();
    assert_eq!(menu.click(composition::AUDIO, 0, 1.0).unwrap(), []);
    assert_eq!(menu.model.section, Section::Audio);
    menu.prepare(viewport, 1.0).unwrap();
    assert_eq!(menu.click(composition::CONTROLS, 0, 1.0).unwrap(), []);
    assert_eq!(menu.model.section, Section::Controls);
    menu.prepare(viewport, 1.0).unwrap();
    assert_eq!(
        menu.click(composition::GRAPHICS, 0, 1.0).unwrap(),
        [Request::Refresh]
    );
    assert!(menu.model.waiting);
    menu.prepare(viewport, 1.0).unwrap();
    assert_eq!(menu.click(composition::REFRESH, 0, 1.0).unwrap(), []);
    menu.model.waiting = false;
    menu.rebuild().unwrap();
    menu.prepare(viewport, 1.0).unwrap();
    assert_eq!(
        menu.click(composition::REFRESH, 0, 1.0).unwrap(),
        [Request::Refresh]
    );
}

#[test]
fn empty_inventory_disables_apply_and_close_works_with_dpi() {
    let mut menu = Menu::new().unwrap();
    menu.model.waiting = false;
    menu.rebuild().unwrap();
    let viewport = WindowViewport {
        width: 2200,
        height: 1720,
    };
    menu.prepare(viewport, 2.0).unwrap();
    assert_eq!(menu.click(composition::APPLY, 0, 2.0).unwrap(), []);
    assert_eq!(
        menu.click(composition::CLOSE, 0, 2.0).unwrap(),
        [Request::Exit]
    );
}
