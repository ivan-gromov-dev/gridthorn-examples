use gridthorn::localization::{CatalogAsset, LocaleId, Localization, MessageId, MessageParameters};
use gridthorn::ui::{UiControl, UiNodeId, UiTree};

pub(super) const LANGUAGES: [&str; 4] = ["en-US", "ru", "ar-EG", "ja"];

pub(super) fn load() -> Result<Localization, Box<dyn std::error::Error>> {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("assets");
    let catalogs = LANGUAGES
        .iter()
        .map(|locale| {
            Ok(CatalogAsset::load(
                LocaleId::new(locale)?,
                root.join(format!("{locale}.ftl")),
            )?)
        })
        .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?;
    Ok(Localization::new(
        catalogs,
        LocaleId::new("en-US")?,
        vec![],
    )?)
}

/// Publish localized captions without replacing editor values or router identity.
pub(super) fn refresh(
    tree: &mut UiTree,
    service: &mut Localization,
) -> Result<(), Box<dyn std::error::Error>> {
    let UiControl::List { selected, .. } = &tree.node(UiNodeId(6)).expect("language list").control
    else {
        unreachable!()
    };
    let locale = LocaleId::new(LANGUAGES[selected.unwrap_or(0)])?;
    let fallback = if locale == LocaleId::new("en-US")? {
        vec![]
    } else {
        vec![LocaleId::new("en-US")?]
    };
    service.select_locale(locale, fallback)?;
    let UiControl::TextField { value, .. } = &tree.node(UiNodeId(7)).expect("editor").control
    else {
        unreachable!()
    };
    let UiControl::Slider { value: count, .. } = tree.node(UiNodeId(5)).expect("count").control
    else {
        unreachable!()
    };
    let mut parameters = MessageParameters::new();
    parameters.set_text("name", value)?;
    parameters.set_number("count", f64::from(count.round()))?;
    parameters.set_number("amount", 12345.5)?;
    let summary = ["workers", "balance", "fallback-only"]
        .iter()
        .map(|id| Ok(service.format(&MessageId::new(id)?, &parameters)?.text))
        .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?
        .join("\n");
    let welcome = service
        .format(&MessageId::new("welcome")?, &parameters)?
        .text;
    let mut root = tree.root().clone();
    for (id, message) in [
        (3, "review"),
        (4, "animation"),
        (8, "editor-hint"),
        (201, "nested"),
        (203, "context"),
        (204, "close"),
        (301, "confirm"),
        (401, "sample"),
        (402, "close-context"),
    ] {
        let caption = service.format(&MessageId::new(message)?, &parameters)?.text;
        caption_node(&mut root, id, &caption);
    }
    set(&mut root, 9, &summary);
    set(&mut root, 202, &welcome);
    tree.replace(root)?;
    Ok(())
}

fn set(node: &mut gridthorn::ui::UiNode, id: u64, value: &str) {
    if node.id == UiNodeId(id) {
        node.control = UiControl::Label(value.into());
    }
    for child in &mut node.children {
        set(child, id, value);
    }
}

fn caption_node(node: &mut gridthorn::ui::UiNode, id: u64, caption: &str) {
    if node.id == UiNodeId(id) {
        match &mut node.control {
            UiControl::Label(text) | UiControl::Button(text) => *text = caption.into(),
            UiControl::Toggle { label, .. } => *label = caption.into(),
            _ => {}
        }
    }
    for child in &mut node.children {
        caption_node(child, id, caption);
    }
}
