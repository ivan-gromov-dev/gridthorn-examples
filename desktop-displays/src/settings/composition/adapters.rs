use super::{
    ADAPTERS, RENDER_API, SAVE_ADAPTER, Settings, UiControl, UiNode, UiVisualState, button, label,
    node,
};
pub(super) fn controls(settings: &Settings) -> Vec<UiNode> {
    vec![
        label(40, "Видеокарта", 32.0),
        label(
            41,
            settings.adapters.as_ref().map_or_else(
                || "GPU недоступен без renderer".into(),
                |adapters| {
                    format!(
                        "Сейчас работает: {} · {:?}",
                        adapters.selected.name, adapters.selected.backend
                    )
                },
            ),
            52.0,
        ),
        adapter_list(settings),
        label(46, "Графический API", 32.0),
        api_list(settings),
        button(
            SAVE_ADAPTER,
            "Сохранить карту и API для следующего запуска",
            settings.adapters.as_ref().is_some_and(|inventory| {
                settings
                    .graphics_selection
                    .resolve(&inventory.adapters)
                    .is_ok()
            }),
        ),
        label(44, &settings.adapter_status, 70.0),
    ]
}
fn adapter_list(settings: &Settings) -> UiNode {
    let mut items = vec!["Автоматический выбор".into()];
    items.extend(
        settings
            .devices()
            .iter()
            .map(|device| device.key.name.clone()),
    );
    let mut list = node(
        ADAPTERS.0,
        UiControl::List {
            items,
            selected: settings.adapter_row(),
        },
        114.0,
    );
    list.style.scroll = true;
    if settings.adapters.is_none() {
        list.visual = UiVisualState::Disabled;
    }
    list
}
fn api_list(settings: &Settings) -> UiNode {
    let mut items = vec!["Автоматический выбор API".into()];
    items.extend(
        settings
            .rendering_apis()
            .iter()
            .map(|api| format!("{api:?}")),
    );
    let mut list = node(
        RENDER_API.0,
        UiControl::List {
            items,
            selected: settings.api_row(),
        },
        148.0,
    );
    list.style.scroll = true;
    if settings.adapters.is_none() {
        list.visual = UiVisualState::Disabled;
    }
    list
}
