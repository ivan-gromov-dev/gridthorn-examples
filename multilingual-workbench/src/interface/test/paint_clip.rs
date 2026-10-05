use gridthorn::ui::{UiControl, UiLength, UiNode, UiNodeId, UiTheme, UiTree};
use gridthorn::{Color, TextStyle, UiPrimitive};

#[test]
fn ancestor_clip_reduces_asset_text_draw_data_without_truncating_value() {
    let mut fonts = super::super::composition::fonts().unwrap();
    let value = "office e\u{301} ".repeat(64);
    let mut root = UiNode::new(UiNodeId(0), UiControl::Panel);
    root.style.size = [UiLength::Fill; 2];
    root.style.clip = true;
    let mut label = UiNode::new(UiNodeId(1), UiControl::Label(value.clone()));
    label.style.size = [UiLength::Pixels(180.0), UiLength::Pixels(120.0)];
    label.style.offset = [-50.25, -100.75];
    root.children.push(label);
    let mut style = TextStyle::new("Noto Sans", 20.0);
    let tree = UiTree::new(
        root,
        UiTheme {
            text: Some(style.clone()),
            ..UiTheme::default()
        },
    )
    .unwrap();
    style.width = Some(180.0);
    let shaped = fonts.layout(&value, &style).unwrap();
    for scale in [1.0, 1.25, 2.0] {
        let full = fonts.rasterize(&shaped, scale, Color::default()).unwrap();
        let layout = tree.layout([100.0, 80.0], scale, Some(&mut fonts)).unwrap();
        let raster = raster(layout.primitives()).unwrap();
        assert_eq!(
            raster.position(),
            layout.placement(UiNodeId(1)).unwrap().content.position
        );
        assert_eq!(raster.measurement(), full.measurement());
        assert!(raster.pixel_count() > 0);
        assert!(raster.pixel_count() < full.pixel_count());
        assert_eq!(
            tree.node(UiNodeId(1)).unwrap().control,
            UiControl::Label(value.clone())
        );
    }
}

fn raster(primitives: &[UiPrimitive]) -> Option<&gridthorn::RasterText> {
    primitives.iter().find_map(|primitive| match primitive {
        UiPrimitive::ShapedText(raster) => Some(raster),
        UiPrimitive::Clipped { children, .. } => raster(children),
        _ => None,
    })
}
