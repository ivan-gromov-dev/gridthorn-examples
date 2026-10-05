use super::model::{Section, Settings};
use gridthorn::ui::{
    UiControl, UiFlow, UiLength, UiNode, UiNodeId, UiTheme, UiTree, UiVisualState,
};
use gridthorn::window::WindowModeKind;
use gridthorn::{Color, FontAsset, TextStyle, TextSystem};

pub(super) const GRAPHICS: UiNodeId = UiNodeId(10);
pub(super) const AUDIO: UiNodeId = UiNodeId(11);
pub(super) const CONTROLS: UiNodeId = UiNodeId(12);
pub(super) const MONITORS: UiNodeId = UiNodeId(20);
pub(super) const REFRESH: UiNodeId = UiNodeId(21);
pub(super) const APPLY: UiNodeId = UiNodeId(22);
pub(super) const RESET: UiNodeId = UiNodeId(23);
pub(super) const CLOSE: UiNodeId = UiNodeId(24);
pub(super) const MODE: UiNodeId = UiNodeId(25);
pub(super) const SIZE: UiNodeId = UiNodeId(26);
pub(super) const RATE: UiNodeId = UiNodeId(27);
pub(super) const RESIZABLE: UiNodeId = UiNodeId(28);

pub(super) fn fonts() -> Result<TextSystem, Box<dyn std::error::Error>> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../multilingual-text/assets/fonts/NotoSans-Regular.ttf");
    Ok(TextSystem::new("ru", &[FontAsset::load(path)?])?)
}

fn node(id: u64, control: UiControl, height: f32) -> UiNode {
    let mut node = UiNode::new(UiNodeId(id), control);
    node.style.size = [UiLength::Fill, UiLength::Pixels(height)];
    node.style.padding = [8.0; 4];
    node.style.clip = true;
    node
}
fn label(id: u64, text: impl Into<String>, height: f32) -> UiNode {
    node(id, UiControl::Label(text.into()), height)
}
fn button(id: UiNodeId, text: &str, enabled: bool) -> UiNode {
    let mut node = node(id.0, UiControl::Button(text.into()), 46.0);
    if !enabled {
        node.visual = UiVisualState::Disabled;
    }
    node
}
fn panel(id: u64, flow: UiFlow) -> UiNode {
    let mut panel = UiNode::new(UiNodeId(id), UiControl::Panel);
    panel.style.size = [UiLength::Fill; 2];
    panel.style.flow = flow;
    panel.style.gap = 10.0;
    panel
}

pub(super) fn tree(settings: &Settings) -> Result<UiTree, gridthorn::ui::UiCompositionError> {
    let mut root = panel(0, UiFlow::Column);
    root.style.padding = [28.0; 4];
    root.style.gap = 18.0;
    root.style.background = Some(Color::rgb(0.035, 0.045, 0.065));
    let mut header = panel(1, UiFlow::Row);
    header.style.size[1] = UiLength::Pixels(64.0);
    let mut heading = label(2, "НАСТРОЙКИ\nПараметры игры", 64.0);
    heading.style.foreground = Some(Color::rgb(0.75, 0.85, 1.0));
    let mut close = button(CLOSE, "Закрыть", true);
    close.style.size[0] = UiLength::Pixels(150.0);
    header.children = vec![heading, close];
    let mut body = panel(3, UiFlow::Row);
    body.style.gap = 20.0;
    let mut sidebar = panel(4, UiFlow::Column);
    sidebar.style.size[0] = UiLength::Pixels(200.0);
    sidebar.style.padding = [12.0; 4];
    sidebar.style.background = Some(Color::rgb(0.055, 0.07, 0.095));
    sidebar.children = [
        (GRAPHICS, "Графика", Section::Graphics),
        (AUDIO, "Звук", Section::Audio),
        (CONTROLS, "Управление", Section::Controls),
    ]
    .into_iter()
    .map(|(id, caption, section)| {
        let mut tab = button(id, caption, true);
        if settings.section == section {
            tab.style.foreground = Some(Color::rgb(0.4, 0.8, 1.0));
        }
        tab
    })
    .collect();
    let mut content = panel(5, UiFlow::Column);
    content.style.padding = [16.0; 4];
    content.style.background = Some(Color::rgb(0.065, 0.08, 0.11));
    content.style.scroll = true;
    content.children = match settings.section {
        Section::Graphics => graphics(settings),
        Section::Audio => vec![
            label(30, "ЗВУК", 36.0),
            label(
                31,
                "Настройки аудиоустройств и громкости будут добавлены в этот раздел.\nСейчас доступен выбор монитора во вкладке «Графика».",
                108.0,
            ),
        ],
        Section::Controls => vec![
            label(30, "УПРАВЛЕНИЕ", 36.0),
            label(
                31,
                "Настройки устройств и привязки действий будут добавлены в этот раздел.\nВ меню уже работают мышь и клавиатура: Tab, стрелки и Enter.",
                108.0,
            ),
        ],
    };
    body.children = vec![sidebar, content];
    let mut footer = label(
        6,
        "Tab / Shift+Tab — фокус     Enter — выбрать     Колесо — прокрутка     Esc — закрыть",
        34.0,
    );
    footer.style.foreground = Some(Color::rgb(0.5, 0.58, 0.7));
    root.children = vec![header, body, footer];
    UiTree::new(
        root,
        UiTheme {
            surface: Color::rgb(0.11, 0.14, 0.19),
            hovered: Color::rgb(0.16, 0.21, 0.29),
            pressed: Color::rgb(0.07, 0.1, 0.15),
            disabled: Color::rgb(0.08, 0.095, 0.12),
            accent: Color::rgb(0.22, 0.58, 0.9),
            foreground: Color::rgb(0.9, 0.93, 0.98),
            text: Some(TextStyle::new("Noto Sans", 18.0)),
            row_height: 34.0,
            ..UiTheme::default()
        },
    )
}

fn graphics(settings: &Settings) -> Vec<UiNode> {
    let size = if settings.graphics.mode == WindowModeKind::Borderless {
        settings
            .chosen()
            .map_or(settings.graphics.size, |monitor| monitor.resolution)
    } else {
        settings.graphics.size
    };
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
    vec![
        actions,
        label(17, &settings.status, 70.0),
        button(MODE, &format!("Режим: {}   →", mode_caption(settings.graphics.mode)), !settings.busy() && settings.graphics.capabilities.available),
        button(SIZE, &format!("Размер: {} × {}   →", size.width, size.height), !settings.busy() && settings.graphics.capabilities.size && settings.graphics.mode != WindowModeKind::Borderless),
        button(RATE, &format!("Частота: {}   →", if settings.graphics.mode == WindowModeKind::Exclusive { refresh(settings.graphics.exclusive.map(|mode| mode.refresh_rate_millihertz)) } else { "режим рабочего стола".into() }), !settings.busy() && settings.graphics.mode == WindowModeKind::Exclusive),
        button(RESIZABLE, if settings.graphics.resizable { "Изменение размера: разрешено" } else { "Изменение размера: запрещено" }, !settings.busy() && settings.graphics.mode == gridthorn::window::WindowModeKind::Windowed && settings.graphics.capabilities.resize_policy),
        tools,
        label(15, "Доступные мониторы", 32.0),
        list,
        label(16, details, 94.0),
        label(
            18,
            settings.graphics.actual.map_or_else(|| "Состояние окна недоступно".into(), |state| format!("Фактически: {}, {} × {}, {}\nРазмер: {}. VSync и ограничение FPS будут добавлены позже.", mode_caption(state.mode), state.size.width, state.size.height, state.display_mode.map_or_else(|| "частота рабочего стола".into(), |mode| refresh(Some(mode.refresh_rate_millihertz))), match state.resizable { Some(true) => "изменяемый", Some(false) => "фиксированный", None => "нет данных ОС" })),
            80.0,
        ),
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
