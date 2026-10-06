use super::super::{composition, model::Request, presentation::Menu};
use gridthorn::{
    WindowViewport,
    presentation::{
        FrameRateLimit, PresentMode, PresentationConfig, PresentationError, PresentationOperation,
        PresentationState,
    },
    ui::{UiCommand, UiNodeId, UiVisualState},
};

fn menu(modes: Vec<PresentMode>, dpi: f64) -> (Menu, WindowViewport) {
    let mut menu = Menu::new().unwrap();
    menu.model.waiting = false;
    menu.model.pacing.observe(
        PresentationState {
            supported_modes: modes,
            applied_mode: Some(PresentMode::Fifo),
            frame_rate_limit: None,
        },
        None,
    );
    menu.rebuild().unwrap();
    let viewport = if dpi > 1.0 {
        WindowViewport {
            width: 2200,
            height: 1720,
        }
    } else {
        WindowViewport {
            width: 1100,
            height: 860,
        }
    };
    menu.tree
        .command(UiNodeId(5), UiCommand::ScrollTo([0.0, 650.0]))
        .unwrap();
    menu.prepare(viewport, dpi).unwrap();
    (menu, viewport)
}

#[test]
fn vsync_and_caps_are_staged_then_dispatched_without_a_monitor_operation() {
    let (mut menu, viewport) = menu(
        vec![
            PresentMode::Fifo,
            PresentMode::Immediate,
            PresentMode::Mailbox,
        ],
        2.0,
    );
    let observed = menu.model.pacing.state.clone();
    assert_eq!(menu.click(composition::VSYNC, 0, 2.0).unwrap(), []);
    assert_eq!(
        menu.model.pacing.config.present_mode,
        PresentMode::Immediate
    );
    menu.prepare(viewport, 2.0).unwrap();
    assert_eq!(menu.click(composition::FPS_CAP, 0, 2.0).unwrap(), []);
    let config = PresentationConfig {
        present_mode: PresentMode::Immediate,
        frame_rate_limit: Some(FrameRateLimit::new(30).unwrap()),
    };
    assert_eq!(menu.model.pacing.config, config);
    assert_eq!(menu.model.pacing.state, observed);
    menu.prepare(viewport, 2.0).unwrap();
    assert_eq!(
        menu.click(composition::APPLY_PRESENTATION, 0, 2.0).unwrap(),
        [Request::Present(config)]
    );
    assert!(menu.model.busy());
    assert!(
        !menu.model.applying,
        "presentation-only application must not wait for a window operation"
    );
    menu.prepare(viewport, 2.0).unwrap();
    assert_eq!(menu.click(composition::FPS_CAP, 0, 2.0).unwrap(), []);
    assert_eq!(menu.model.pacing.config, config);
}

#[test]
fn unavailable_vsync_off_is_disabled_and_cap_cycle_preserves_the_queue_policy() {
    let (mut menu, viewport) = menu(vec![PresentMode::Fifo], 1.0);
    assert_eq!(
        menu.tree.node(composition::VSYNC).unwrap().visual,
        UiVisualState::Disabled
    );
    assert_eq!(
        menu.tree.node(composition::PRESENT_MODE).unwrap().visual,
        UiVisualState::Disabled
    );
    assert_eq!(menu.click(composition::VSYNC, 0, 1.0).unwrap(), []);
    assert_eq!(menu.model.pacing.config.present_mode, PresentMode::Fifo);
    for fps in [Some(30), Some(60), Some(90), Some(144), Some(1), None] {
        menu.prepare(viewport, 1.0).unwrap();
        assert_eq!(menu.click(composition::FPS_CAP, 0, 1.0).unwrap(), []);
        assert_eq!(
            menu.model
                .pacing
                .config
                .frame_rate_limit
                .map(FrameRateLimit::fps),
            fps
        );
        assert_eq!(menu.model.pacing.config.present_mode, PresentMode::Fifo);
        assert_eq!(menu.model.pacing.state.frame_rate_limit, None);
    }
}

#[test]
fn failed_or_pending_requests_keep_actual_state_separate_and_reset_uses_observations() {
    let (mut menu, viewport) = menu(vec![PresentMode::Fifo, PresentMode::Immediate], 1.0);
    menu.click(composition::VSYNC, 0, 1.0).unwrap();
    menu.prepare(viewport, 1.0).unwrap();
    menu.click(composition::FPS_CAP, 0, 1.0).unwrap();
    let observed = menu.model.pacing.state.clone();
    menu.model.pacing.observe(
        observed.clone(),
        Some(PresentationOperation::Pending { id: 1 }),
    );
    menu.model.applying = false;
    assert!(
        menu.model.busy(),
        "window feedback must not clear a pending presentation request"
    );
    menu.model.pacing.observe(
        observed.clone(),
        Some(PresentationOperation::Failed {
            id: 1,
            error: PresentationError::UnsupportedMode(PresentMode::Immediate),
        }),
    );
    assert!(!menu.model.busy());
    assert_eq!(menu.model.pacing.state, observed);
    assert_eq!(
        menu.model.pacing.config.present_mode,
        PresentMode::Immediate
    );
    assert!(menu.model.pacing.status.contains("unsupported"));
    menu.rebuild().unwrap();
    menu.tree
        .command(UiNodeId(5), UiCommand::ScrollTo([0.0, 0.0]))
        .unwrap();
    menu.prepare(viewport, 1.0).unwrap();
    assert_eq!(menu.click(composition::RESET, 0, 1.0).unwrap(), []);
    assert_eq!(menu.model.pacing.config, PresentationConfig::default());
}

#[test]
fn renderer_free_menu_disables_presentation_requests() {
    let mut menu = Menu::new().unwrap();
    menu.model.waiting = false;
    menu.rebuild().unwrap();
    assert_eq!(
        menu.tree
            .node(composition::APPLY_PRESENTATION)
            .unwrap()
            .visual,
        UiVisualState::Disabled
    );
    assert_eq!(
        menu.tree.node(composition::FPS_CAP).unwrap().visual,
        UiVisualState::Disabled
    );
    assert_eq!(menu.model.pacing.request(), None);
}
