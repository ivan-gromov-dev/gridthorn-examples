mod snapshot;
use super::{model::Request, presentation::Menu, smoke::Smoke};
use gridthorn::display::Displays;
use gridthorn::presentation::{PresentationError, PresentationOperation, PresentationSettings};
use gridthorn::window::WindowSettings;
use gridthorn::{
    ApplicationRuntime, ExitRequest, KeyCode, ScheduleBuilder, ScheduleStage, WindowViewport,
};

pub(crate) fn runtime(
    smoke: bool,
    preferences: crate::adapters::Preferences,
    selection: gridthorn::GraphicsSelection,
) -> Result<ApplicationRuntime, Box<dyn std::error::Error>> {
    let mut initial = Some(Menu::new()?);
    let mut schedules = ScheduleBuilder::new();
    schedules.add_system(ScheduleStage::Startup, move |world| {
        let mut menu = initial.take().expect("startup once");
        menu.model.graphics_selection.clone_from(&selection);
        menu.model.adapters =
            world.read_resource(|adapters: &gridthorn::GraphicsAdapters| adapters.clone());
        if let Some(settings) =
            world.read_resource(|settings: &PresentationSettings| settings.clone())
        {
            menu.model.pacing.sync(&settings);
        }
        if let Some(adapters) = &menu.model.adapters {
            println!("effective_graphics_adapter: {:?}", adapters.selected);
        }
        menu.rebuild().expect("valid initial settings tree");
        world.insert_resource(menu);
        world.update_resource(Displays::request_refresh);
    });
    let mut smoke = smoke.then(Smoke::default);
    let mut adapter_smoke = std::env::args()
        .any(|arg| arg == "--adapter-ui-smoke")
        .then(|| super::adapter_smoke::AdapterSmoke::new(preferences.clone()));
    let mut pacing_smoke = std::env::args()
        .any(|arg| arg == "--presentation-smoke")
        .then(super::pacing_smoke::PacingSmoke::default);
    schedules.add_system(ScheduleStage::Input, move |world| {
        update(world, &mut smoke, &mut adapter_smoke, &mut pacing_smoke);
    });
    let mut runtime = ApplicationRuntime::new(schedules.build());
    runtime.world().insert_resource(ExitRequest::default());
    runtime.world().insert_resource(preferences);
    Ok(runtime)
}

fn update(
    world: &mut gridthorn::WorldAccess<'_>,
    smoke: &mut Option<Smoke>,
    adapter_smoke: &mut Option<super::adapter_smoke::AdapterSmoke>,
    pacing_smoke: &mut Option<super::pacing_smoke::PacingSmoke>,
) {
    let snapshot::Snapshot {
        viewport,
        dpi,
        inventory,
        input,
        window,
        pacing,
        pacing_timing,
    } = snapshot::read(world);
    let exit_key = input
        .as_ref()
        .is_some_and(|input| input.key_just_pressed(KeyCode::Escape));
    let result = world.update_resource_with(|menu: &mut Menu| {
        if let Some(pacing) = pacing {
            menu.model.pacing.sync(&pacing);
            menu.rebuild()?;
        }
        if let Some(window) = window {
            menu.model.sync_window(&window);
            menu.rebuild()?;
        }
        if let Some(displays) = inventory {
            menu.model.sync(&displays);
            menu.rebuild()?;
        }
        let mut frame = menu.prepare(viewport, dpi)?;
        let mut requests = if let Some(input) = input {
            menu.input(&input)?
        } else {
            Vec::new()
        };
        if let Some(smoke) = smoke.as_mut() {
            requests.extend(smoke.advance(menu, viewport, dpi)?);
        }
        if let Some(smoke) = adapter_smoke.as_mut() {
            requests.extend(smoke.advance(menu, viewport, dpi)?);
        }
        if let Some(smoke) = pacing_smoke.as_mut() {
            requests.extend(smoke.advance(menu, viewport, dpi, pacing_timing)?);
        }
        if let Some(updated) = menu.prepare(viewport, dpi)? {
            frame = Some(updated);
        }
        Ok::<_, gridthorn::ui::UiCompositionError>((requests, frame))
    });
    match result {
        Some(Ok((requests, frame))) => {
            if let Some(frame) = frame {
                world.insert_resource(frame);
            }
            for request in requests {
                dispatch(world, request);
            }
        }
        Some(Err(error)) => {
            eprintln!("Settings UI failed: {error}");
            world.update_resource(|exit: &mut ExitRequest| exit.request());
        }
        None => {}
    }
    if exit_key {
        dispatch(world, Request::Exit);
    }
}

fn dispatch(world: &mut gridthorn::WorldAccess<'_>, request: Request) {
    match request {
        Request::Present(config) => {
            let result = world
                .update_resource_with(|settings: &mut PresentationSettings| {
                    settings.request(config)
                })
                .unwrap_or(Err(PresentationError::Unavailable));
            if let Err(error) = result {
                world.update_resource(|menu: &mut Menu| {
                    let state = menu.model.pacing.state.clone();
                    menu.model
                        .pacing
                        .observe(state, Some(PresentationOperation::Failed { id: 0, error }));
                    if let Err(error) = menu.rebuild() {
                        eprintln!("Settings UI failed: {error}");
                    }
                });
            }
        }
        Request::Refresh => {
            world.update_resource(Displays::request_refresh);
        }
        Request::Configure(request) => {
            if let Some(Err(error)) =
                world.update_resource_with(|window: &mut WindowSettings| window.request(request))
            {
                world.update_resource(|menu: &mut Menu| {
                    menu.model.applying = false;
                    menu.model.status = format!("Некорректные параметры: {error}");
                    if let Err(error) = menu.rebuild() {
                        eprintln!("Settings UI failed: {error}");
                    }
                });
            }
        }
        Request::Exit => {
            world.update_resource(|exit: &mut ExitRequest| exit.request());
        }
        Request::SaveAdapter => {
            let preferences = world
                .read_resource(|preferences: &crate::adapters::Preferences| preferences.clone());
            let selection =
                world.read_resource(|menu: &Menu| menu.model.graphics_selection.clone());
            if let (Some(preferences), Some(selection)) = (preferences, selection) {
                let result = preferences.save(&selection);
                world.update_resource(|menu: &mut Menu| {
                    menu.model.adapter_status = match &result {
                        Ok(()) => {
                            "Выбор сохранён. Новый GPU будет выбран после перезапуска приложения."
                                .into()
                        }
                        Err(error) => format!("Не удалось сохранить выбор GPU: {error}"),
                    };
                    if let Err(error) = menu.rebuild() {
                        eprintln!("Settings UI failed: {error}");
                    }
                });
            }
        }
    }
}

pub(crate) fn headless() -> Result<(), Box<dyn std::error::Error>> {
    let mut menu = Menu::new()?;
    menu.model.waiting = false;
    menu.rebuild()?;
    let viewport = WindowViewport {
        width: 1100,
        height: 860,
    };
    for dpi in [1.0, 1.25, 2.0] {
        assert!(menu.prepare(viewport, dpi)?.is_some());
        let layouts = menu.layouts;
        assert!(menu.prepare(viewport, dpi)?.is_none());
        assert_eq!(menu.layouts, layouts, "idle UI must reuse layout");
    }
    println!("settings_headless: graphics/empty inventory, DPI and idle layout cache passed");
    Ok(())
}
