use gridthorn::ui::{UiEasing, UiNodeId, UiProperty, UiTransition, UiTree};
use std::time::Duration;

/// Slide a control or modal panel into place and fade its background override.
pub(super) fn transitions(
    tree: &UiTree,
    layers: bool,
) -> Result<Vec<UiTransition>, gridthorn::ui::UiAnimationError> {
    let id = UiNodeId(if layers { 200 } else { 3 });
    let end = tree
        .node(id)
        .expect("example animation target")
        .style
        .offset;
    Ok(vec![
        UiTransition::new(
            id,
            UiProperty::Offset([end[0] + 60.0, end[1]]),
            UiProperty::Offset(end),
            Duration::from_secs(1),
            UiEasing::SmoothStep,
        )?,
        UiTransition::new(
            id,
            UiProperty::Background([0.12, 0.18, 0.28, 0.0]),
            UiProperty::Background([0.12, 0.18, 0.28, 1.0]),
            Duration::from_secs(1),
            UiEasing::EaseOut,
        )?,
    ])
}

/// Check explicit transition application and interruption through the public facade.
pub(super) fn headless() -> Result<(), Box<dyn std::error::Error>> {
    for layers in [false, true] {
        let mut tree = if layers {
            super::layers::tree()?
        } else {
            super::composition::tree()?
        };
        let mut fonts = super::composition::fonts()?;
        let mut animations = transitions(&tree, layers)?;
        let id = UiNodeId(if layers { 200 } else { 3 });
        let end = tree.node(id).unwrap().style.offset;
        for transition in &mut animations {
            transition.advance(&mut tree, Duration::from_millis(500))?;
        }
        assert_eq!(tree.node(id).unwrap().style.offset, [end[0] + 30.0, end[1]]);
        for dpi in [1.0, 2.0] {
            assert_ne!(
                tree.layout([900.0, 700.0], dpi, Some(&mut fonts))?
                    .primitives(),
                []
            );
        }
        animations[0].retarget(
            UiProperty::Offset(end),
            Duration::from_millis(250),
            UiEasing::Linear,
        )?;
        animations[0].pause();
        animations[0].advance(&mut tree, Duration::from_secs(1))?;
        assert_eq!(tree.node(id).unwrap().style.offset, [end[0] + 30.0, end[1]]);
        animations[0].resume();
        for transition in &mut animations {
            assert!(transition.advance(&mut tree, Duration::from_millis(500))?);
        }
        assert_eq!(tree.node(id).unwrap().style.offset, end);
    }
    println!("UI transitions: slide, background alpha, interruption, pause and DPI passed");
    Ok(())
}
