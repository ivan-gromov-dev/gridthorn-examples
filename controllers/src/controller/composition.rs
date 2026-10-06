use super::model::Devices;
use gridthorn::ui::{
    UiCompositionError, UiControl, UiFlow, UiLength, UiNode, UiNodeId, UiTheme, UiTree,
};
use gridthorn::{Color, FontAsset, TextStyle, TextSystem};
pub(super) fn fonts() -> Result<TextSystem, Box<dyn std::error::Error>> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../multilingual-text/assets/fonts/NotoSans-Regular.ttf");
    Ok(TextSystem::new("ru-RU", &[FontAsset::load(path)?])?)
}
fn node(id: u64, control: UiControl, height: f32) -> UiNode {
    let mut node = UiNode::new(UiNodeId(id), control);
    node.style.size = [UiLength::Fill, UiLength::Pixels(height)];
    node.style.padding = [8.0; 4];
    node
}
pub(super) fn tree(devices: &Devices) -> Result<UiTree, UiCompositionError> {
    let mut root = node(0, UiControl::Panel, 760.0);
    root.style.size = [UiLength::Fill; 2];
    root.style.flow = UiFlow::Column;
    root.style.padding = [24.0; 4];
    root.style.gap = 12.0;
    root.style.scroll = true;
    root.style.background = Some(Color::rgb(0.025, 0.035, 0.055));
    let mut list = node(
        2,
        UiControl::List {
            items: devices.items(),
            selected: Some(devices.row()),
        },
        88.0,
    );
    list.style.scroll = true;
    let mut action = node(
        3,
        UiControl::Button("Проверить вибрацию · 200 мс".into()),
        44.0,
    );
    if devices.selected.is_none() {
        action.visual = gridthorn::ui::UiVisualState::Disabled;
    }
    root.children = vec![
        node(1, UiControl::Label("Устройства ввода / Controller lab".into()), 42.0), list, action,
        node(6, UiControl::Button("Обновить устройства".into()), 38.0),
        node(7, UiControl::Button("Применить устройство".into()), 38.0),
        node(4, UiControl::Label(devices.feedback.clone()), 38.0),
        node(10, UiControl::Panel, 330.0),
        node(5, UiControl::Label("Выберите подключённое устройство. Кнопки подсвечиваются при удержании.\nСтики: точка и координаты; триггеры: уровень нажатия.\nКлик — выбор; стрелки / стик — выделение, затем «Применить устройство». Escape — выход.".into()), 100.0),
    ];
    UiTree::new(
        root,
        UiTheme {
            text: Some(TextStyle::new("Noto Sans", 17.0)),
            row_height: 32.0,
            ..UiTheme::default()
        },
    )
}
