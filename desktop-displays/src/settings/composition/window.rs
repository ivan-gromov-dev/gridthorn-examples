use super::{
    APPLY, MODE, MONITORS, RATE, REFRESH, RESET, RESIZABLE, SIZE, Settings, UiControl, UiFlow,
    UiLength, UiNode, UiVisualState, button, label, node, panel,
};
use gridthorn::window::WindowModeKind;
pub(super) fn controls(settings: &Settings) -> Vec<UiNode> {
    let size = if settings.graphics.mode == WindowModeKind::Borderless {
        settings
            .chosen()
            .map_or(settings.graphics.size, |monitor| monitor.resolution)
    } else {
        settings.graphics.size
    };
    let mut actions = panel(9, UiFlow::Row);
    actions.style.size[1] = UiLength::Pixels(46.0);
    actions.children = vec![
        button(
            APPLY,
            "Применить",
            !settings.busy() && settings.chosen().is_some(),
        ),
        button(RESET, "Сбросить выбор", !settings.busy()),
    ];
    let mut nodes = vec![
        actions,
        label(17, &settings.status, 70.0),
        button(
            MODE,
            &format!("Режим: {}   →", mode_caption(settings.graphics.mode)),
            !settings.busy() && settings.graphics.capabilities.available,
        ),
        button(
            SIZE,
            &format!("Размер: {} × {}   →", size.width, size.height),
            !settings.busy()
                && settings.graphics.capabilities.size
                && settings.graphics.mode != WindowModeKind::Borderless,
        ),
        button(
            RATE,
            &format!(
                "Частота монитора: {}   →",
                if settings.graphics.mode == WindowModeKind::Exclusive {
                    refresh(
                        settings
                            .graphics
                            .exclusive
                            .map(|mode| mode.refresh_rate_millihertz),
                    )
                } else {
                    "режим рабочего стола".into()
                }
            ),
            !settings.busy() && settings.graphics.mode == WindowModeKind::Exclusive,
        ),
        button(
            RESIZABLE,
            if settings.graphics.resizable {
                "Изменение размера: разрешено"
            } else {
                "Изменение размера: запрещено"
            },
            !settings.busy()
                && settings.graphics.mode == gridthorn::window::WindowModeKind::Windowed
                && settings.graphics.capabilities.resize_policy,
        ),
    ];
    nodes.extend(inventory(settings));
    nodes.push(label(
        18,
        settings.graphics.actual.map_or_else(
            || "Состояние окна недоступно".into(),
            |state| {
                format!(
                    "Фактически: {}, {} × {}, {}\nРазмер: {}.",
                    mode_caption(state.mode),
                    state.size.width,
                    state.size.height,
                    state.display_mode.map_or_else(
                        || "частота рабочего стола".into(),
                        |mode| refresh(Some(mode.refresh_rate_millihertz))
                    ),
                    match state.resizable {
                        Some(true) => "изменяемый",
                        Some(false) => "фиксированный",
                        None => "нет данных ОС",
                    }
                )
            },
        ),
        80.0,
    ));
    nodes
}

fn inventory(settings: &Settings) -> Vec<UiNode> {
    let current = settings
        .monitors
        .iter()
        .position(|monitor| Some(monitor.id) == settings.active)
        .map_or_else(
            || "Не определён".into(),
            |index| format!("Монитор {}", index + 1),
        );
    let mut tools = panel(7, UiFlow::Row);
    tools.style.size[1] = UiLength::Pixels(46.0);
    tools.children = vec![
        button(REFRESH, "Обновить список", !settings.busy()),
        label(8, format!("Текущий: {current}"), 46.0),
    ];
    let items = settings
        .monitors
        .iter()
        .enumerate()
        .map(|(index, monitor)| {
            format!(
                "Монитор {}   ·   {} × {}   ·   {}",
                index + 1,
                monitor.resolution.width,
                monitor.resolution.height,
                refresh(monitor.refresh_rate_millihertz)
            )
        })
        .collect();
    let selected = settings
        .monitors
        .iter()
        .position(|monitor| Some(monitor.id) == settings.selected);
    let mut list = node(MONITORS.0, UiControl::List { items, selected }, 114.0);
    list.style.scroll = true;
    if settings.busy() || settings.monitors.is_empty() {
        list.visual = UiVisualState::Disabled;
    }
    let details = settings.chosen().map_or_else(|| "Выберите доступный монитор из списка.".into(), |monitor| format!("Рабочий стол: {} × {} пикселей   ·   {}\nМасштаб интерфейса ОС: {:.0}%\nРежимов, сообщённых ОС: {}", monitor.resolution.width, monitor.resolution.height, refresh(monitor.refresh_rate_millihertz), monitor.scale_factor * 100.0, monitor.modes.len()));
    vec![
        tools,
        label(15, "Доступные мониторы", 32.0),
        list,
        label(16, details, 94.0),
    ]
}
fn mode_caption(mode: WindowModeKind) -> &'static str {
    match mode {
        WindowModeKind::Windowed => "Оконный",
        WindowModeKind::Borderless => "Без рамки",
        WindowModeKind::Exclusive => "Эксклюзивный",
    }
}

fn refresh(rate: Option<u32>) -> String {
    rate.map_or_else(
        || "Частота неизвестна".into(),
        |rate| format!("{:.2} Гц", f64::from(rate) / 1000.0),
    )
}
