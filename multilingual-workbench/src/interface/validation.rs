use super::runtime::Interface;
use gridthorn::WindowViewport;
use gridthorn::ui::{UiCommand, UiControl, UiEffect, UiNodeId};

/// Validate locale publication, editing, modal scopes and resize using the native tree.
pub(super) fn run() -> Result<(), Box<dyn std::error::Error>> {
    for dpi in [1.0, 2.0] {
        let mut state = Interface::new()?;
        let viewport = WindowViewport {
            width: 1000,
            height: 800,
        };
        state.prepare(viewport, dpi)?;
        assert!(
            state
                .layout
                .as_ref()
                .unwrap()
                .placement(UiNodeId(200))
                .is_none()
        );
        for index in 0..4 {
            state
                .tree
                .command(UiNodeId(6), UiCommand::Select(Some(index)))?;
            state.effect(UiNodeId(6), UiEffect::Changed)?;
            state.prepare(viewport, dpi)?;
            let UiControl::Label(summary) = &state.tree.node(UiNodeId(9)).unwrap().control else {
                unreachable!()
            };
            assert!(summary.contains("English fallback"));
        }
        state.effect(UiNodeId(3), UiEffect::Activated)?;
        state.prepare(viewport, dpi)?;
        assert_eq!(state.router.open_layers(), [UiNodeId(200)]);
        state.effect(UiNodeId(201), UiEffect::Activated)?;
        state.prepare(viewport, dpi)?;
        assert_eq!(state.router.open_layers(), [UiNodeId(200), UiNodeId(300)]);
        let route =
            state
                .router
                .route_events(&mut state.tree, state.layout.as_ref().unwrap(), &[])?;
        assert!(route.keyboard_blocked && route.pointer_blocked);
        state.effect(UiNodeId(301), UiEffect::Activated)?;
        state.prepare(viewport, dpi)?;
        assert_eq!(state.router.focused(), Some(UiNodeId(201)));
        state.effect(UiNodeId(203), UiEffect::Activated)?;
        state.prepare(viewport, dpi)?;
        assert_eq!(state.router.open_layers(), [UiNodeId(200), UiNodeId(400)]);
        state.effect(UiNodeId(401), UiEffect::Activated)?;
        state.prepare(viewport, dpi)?;
        assert!(
            matches!(&state.tree.node(UiNodeId(7)).unwrap().control, UiControl::TextField { value, .. } if value.contains("日本語"))
        );
        state.effect(UiNodeId(204), UiEffect::Activated)?;
        state.prepare(viewport, dpi)?;
        assert_eq!(state.router.open_layers(), []);
        state.effect(UiNodeId(3), UiEffect::Activated)?;
        state.prepare(
            WindowViewport {
                width: 640,
                height: 480,
            },
            dpi,
        )?;
        assert_ne!(state.layout.as_ref().unwrap().primitives(), []);
    }
    super::interaction::headless()?;
    println!(
        "Workbench: four catalogs, fallback, editable values, nested modal/context actions, reopening and DPI passed"
    );
    Ok(())
}

#[cfg(test)]
#[path = "test/workbench.rs"]
mod test;
