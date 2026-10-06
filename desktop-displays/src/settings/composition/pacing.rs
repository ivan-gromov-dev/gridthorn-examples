use super::{APPLY_PRESENTATION, FPS_CAP, PRESENT_MODE, Settings, UiNode, VSYNC, button, label};
pub(super) fn controls(settings: &Settings) -> Vec<UiNode> {
    use super::super::frame_pacing::{cap_caption, mode_caption};
    use gridthorn::presentation::PresentMode;
    let pacing = &settings.pacing;
    let vsync = match pacing.config.present_mode {
        PresentMode::Fifo => "включён (FIFO)",
        PresentMode::FifoRelaxed => "адаптивный",
        PresentMode::Immediate => "выключен (Immediate)",
        PresentMode::Mailbox => "включён (Mailbox)",
    };
    let suffix = if pacing.available() && !pacing.can_toggle() {
        " · переключение не поддерживается"
    } else {
        "   →"
    };
    vec![
        label(49, "Показ кадров", 32.0),
        button(
            VSYNC,
            &format!("VSync: {vsync}{suffix}"),
            !settings.busy() && pacing.can_toggle(),
        ),
        button(
            PRESENT_MODE,
            &format!(
                "Режим показа: {}   →",
                mode_caption(pacing.config.present_mode)
            ),
            !settings.busy() && pacing.state.supported_modes.len() > 1,
        ),
        button(
            FPS_CAP,
            &format!(
                "Лимит FPS: {}   →",
                cap_caption(pacing.config.frame_rate_limit)
            ),
            !settings.busy() && pacing.available(),
        ),
        button(
            APPLY_PRESENTATION,
            "Применить VSync и FPS",
            !settings.busy() && pacing.available(),
        ),
        label(54, &pacing.status, 70.0),
        label(
            55,
            format!(
                "Фактический режим показа: {}\nДействующий лимит: {}",
                pacing
                    .state
                    .applied_mode
                    .map_or("ещё не настроен", mode_caption),
                cap_caption(pacing.state.frame_rate_limit)
            ),
            100.0,
        ),
        label(
            56,
            "Частота монитора и лимит FPS — отдельные настройки.\nЛимит FPS не меняет фиксированный шаг симуляции.\nVSync/GPU могут снижать фактический FPS ниже лимита.",
            100.0,
        ),
    ]
}
