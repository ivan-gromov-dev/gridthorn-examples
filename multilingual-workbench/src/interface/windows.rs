use gridthorn::ui::{UiControl, UiFlow, UiLength, UiNode, UiNodeId, UiTree};

pub(super) fn tree() -> Result<UiTree, gridthorn::ui::UiCompositionError> {
    let base = super::composition::tree()?;
    let mut root = UiNode::new(UiNodeId(100), UiControl::Panel);
    root.style.size = [UiLength::Fill; 2];
    root.children.push(base.root().clone());
    for (id, offset, title, actions) in [
        (
            200,
            [80.0, 100.0],
            "Review / Проверка / مراجعة / 確認",
            vec![
                (201, "Open nested confirmation"),
                (203, "Context actions"),
                (204, "Close review"),
            ],
        ),
        (
            300,
            [150.0, 160.0],
            "Confirm / Подтвердить / تأكيد / 確認",
            vec![(301, "Confirm and return")],
        ),
        (
            400,
            [240.0, 200.0],
            "Context / Контекст / القائمة / メニュー",
            vec![(401, "Insert multilingual sample"), (402, "Close context")],
        ),
    ] {
        let mut panel = UiNode::new(UiNodeId(id), UiControl::Panel);
        panel.style.size = [UiLength::Pixels(560.0), UiLength::Pixels(350.0)];
        panel.style.offset = offset;
        panel.style.flow = UiFlow::Column;
        panel.style.padding = [16.0; 4];
        panel.style.gap = 8.0;
        panel.style.clip = true;
        panel.style.scroll = true;
        panel.style.background = Some(gridthorn::Color::rgb(0.10, 0.16, 0.25));
        let mut heading = UiNode::new(UiNodeId(id + 10), UiControl::Label(title.into()));
        heading.style.size = [UiLength::Fill, UiLength::Pixels(44.0)];
        panel.children.push(heading);
        if id == 200 {
            let mut preview = UiNode::new(UiNodeId(202), UiControl::Label(String::new()));
            preview.style.size = [UiLength::Fill, UiLength::Pixels(100.0)];
            preview.style.clip = true;
            panel.children.push(preview);
        }
        for (button_id, label) in actions {
            let mut button = UiNode::new(UiNodeId(button_id), UiControl::Button(label.into()));
            button.style.size = [UiLength::Fill, UiLength::Pixels(44.0)];
            panel.children.push(button);
        }
        root.children.push(panel);
    }
    UiTree::new(
        root,
        gridthorn::ui::UiTheme {
            text: Some(gridthorn::TextStyle::new("Noto Sans", 20.0)),
            row_height: 32.0,
            ..gridthorn::ui::UiTheme::default()
        },
    )
}
