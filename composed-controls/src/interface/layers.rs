use gridthorn::ui::{
    UiCompositionError, UiControl, UiLayer, UiLayout, UiLength, UiNavigation, UiNode, UiNodeId,
    UiRouter, UiTheme, UiTree,
};

pub(super) fn tree() -> Result<UiTree, UiCompositionError> {
    let mut root = UiNode::new(UiNodeId(100), UiControl::Panel);
    root.style.size = [UiLength::Fill; 2];
    root.children
        .push(super::composition::tree()?.root().clone());
    for (id, offset, caption) in [
        (200, 80.0, "Dialog: Escape closes"),
        (300, 180.0, "Context popup: click outside"),
    ] {
        let mut panel = UiNode::new(UiNodeId(id), UiControl::Panel);
        panel.style.size = [UiLength::Pixels(460.0), UiLength::Pixels(120.0)];
        panel.style.offset = [offset; 2];
        panel.style.padding = [12.0; 4];
        panel.style.background = Some(gridthorn::Color::rgb(0.12, 0.18, 0.28));
        let mut button = UiNode::new(UiNodeId(id + 1), UiControl::Button(caption.into()));
        button.style.size = [UiLength::Fill, UiLength::Pixels(48.0)];
        panel.children.push(button);
        root.children.push(panel);
    }
    UiTree::new(
        root,
        UiTheme {
            text: Some(gridthorn::TextStyle::new("Noto Sans", 20.0)),
            row_height: 32.0,
            ..UiTheme::default()
        },
    )
}

pub(super) fn open(
    tree: &mut UiTree,
    router: &mut UiRouter,
    layout: &UiLayout,
) -> Result<(), UiCompositionError> {
    router.navigate(tree, layout, UiNavigation::Next)?;
    router.open_layer(
        tree,
        layout,
        UiNodeId(200),
        UiLayer {
            modal: true,
            dismiss_escape: true,
            dismiss_outside: false,
        },
    )?;
    router.open_layer(
        tree,
        layout,
        UiNodeId(300),
        UiLayer {
            modal: true,
            dismiss_escape: true,
            dismiss_outside: true,
        },
    )?;
    Ok(())
}

pub(super) fn headless() -> Result<(), Box<dyn std::error::Error>> {
    let mut tree = tree()?;
    let mut fonts = super::composition::fonts()?;
    let mut router = UiRouter::new(20_000);
    let layout = tree.layout([900.0, 700.0], 2.0, Some(&mut fonts))?;
    open(&mut tree, &mut router, &layout)?;
    assert_eq!(router.focused(), Some(UiNodeId(301)));
    let paint = router.layout(&tree, [900.0, 700.0], 2.0, Some(&mut fonts))?;
    assert_eq!(paint.placements().last().unwrap().id, UiNodeId(301));
    let route = router.route_events(
        &mut tree,
        &paint,
        &[gridthorn::InputEvent::Keyboard {
            key: gridthorn::KeyCode::Escape,
            state: gridthorn::ButtonState::Pressed,
        }],
    )?;
    assert_eq!(route.dismissed, [UiNodeId(300)]);
    assert_eq!(router.focused(), Some(UiNodeId(201)));
    assert!(route.keyboard_blocked && route.pointer_blocked);
    router.close_layer(&tree, &layout);
    assert_eq!(router.focused(), Some(UiNodeId(3)));
    println!("Nested dialog/context layers: order, dismissal, blocking and restored focus passed");
    Ok(())
}
