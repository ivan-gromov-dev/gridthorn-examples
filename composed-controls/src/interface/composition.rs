use gridthorn::ui::{UiControl, UiFlow, UiLength, UiNode, UiNodeId, UiTheme, UiTree};
use gridthorn::{Color, FontAsset, TextStyle, TextSystem};

pub(super) fn fonts() -> Result<TextSystem, Box<dyn std::error::Error>> {
    let directory =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../multilingual-text/assets/fonts");
    let assets = [
        "NotoSans-Regular.ttf",
        "NotoSansArabic-Regular.ttf",
        "NotoSansJP-Regular.otf",
    ]
    .map(|name| FontAsset::load(directory.join(name)))
    .into_iter()
    .collect::<Result<Vec<_>, _>>()?;
    Ok(TextSystem::new("en-US", &assets)?)
}

fn control(id: u64, value: UiControl, height: UiLength) -> UiNode {
    let mut node = UiNode::new(UiNodeId(id), value);
    node.style.size = [UiLength::Fill, height];
    node.style.padding = [8.0; 4];
    node.style.clip = true;
    node
}

pub(super) fn tree() -> Result<UiTree, gridthorn::ui::UiCompositionError> {
    let mut root = UiNode::new(UiNodeId(0), UiControl::Panel);
    root.style.size = [UiLength::Fill; 2];
    root.style.padding = [24.0; 4];
    root.style.gap = 12.0;
    root.style.flow = UiFlow::Column;
    root.style.scroll = true;
    root.style.background = Some(Color::rgb(0.025, 0.035, 0.055));
    let title = control(
        1,
        UiControl::Label("Settings / Настройки / العربية / 日本語".into()),
        UiLength::Auto,
    );
    let mut row = UiNode::new(UiNodeId(2), UiControl::Panel);
    row.style.size = [UiLength::Fill, UiLength::Pixels(56.0)];
    row.style.flow = UiFlow::Row;
    row.style.gap = 12.0;
    row.children = vec![
        control(3, UiControl::Button("Apply".into()), UiLength::Fill),
        control(
            4,
            UiControl::Toggle {
                label: "Enabled".into(),
                checked: true,
            },
            UiLength::Fill,
        ),
    ];
    let slider = control(
        5,
        UiControl::Slider {
            min: 0.0,
            max: 100.0,
            value: 40.0,
        },
        UiLength::Pixels(40.0),
    );
    let mut list = control(
        6,
        UiControl::List {
            items: vec![
                "English".into(),
                "Русский".into(),
                "العربية".into(),
                "日本語".into(),
                "Nested clipping".into(),
                "Scroll limits".into(),
                "Reusable controls".into(),
                "Presentation only".into(),
            ],
            selected: Some(0),
        },
        UiLength::Pixels(128.0),
    );
    list.style.scroll = true;
    let field = control(
        7,
        UiControl::TextField {
            value: String::new(),
            placeholder: "Click or Tab here to type multilingual text".into(),
        },
        UiLength::Pixels(72.0),
    );
    let hint = control(8, UiControl::Label("Click controls · drag slider · wheel scrolls\nTab / Shift+Tab: focus · Enter/Space: activate\nArrows: navigation/value/text caret · Shift: select\nCtrl/Command+A/C/X/V: select/copy/cut/paste\nUnicode/IME text editing · Escape: clear focus, then exit".into()), UiLength::Auto);
    root.children = vec![title, row, slider, list, field, hint];
    UiTree::new(
        root,
        UiTheme {
            text: Some(TextStyle::new("Noto Sans", 20.0)),
            row_height: 32.0,
            ..UiTheme::default()
        },
    )
}
