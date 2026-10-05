use super::{model::Request, presentation::Menu, smoke::Smoke};
use gridthorn::display::Displays;
use gridthorn::window::WindowSettings;
use gridthorn::{
    ApplicationRuntime, ExitRequest, InputState, KeyCode, ScheduleBuilder, ScheduleStage,
    WindowScaleFactor, WindowViewport,
};

pub(crate) fn runtime(smoke: bool) -> Result<ApplicationRuntime, Box<dyn std::error::Error>> {
    let mut initial = Some(Menu::new()?);
    let mut schedules = ScheduleBuilder::new();
    schedules.add_system(ScheduleStage::Startup, move |world| {
        world.insert_resource(initial.take().expect("startup once"));
        world.update_resource(Displays::request_refresh);
    });
    let mut smoke = smoke.then(Smoke::default);
    schedules.add_system(ScheduleStage::Input, move |world| {
        let viewport = world
            .read_resource(|value: &WindowViewport| *value)
            .unwrap_or_default();
        let dpi = world
            .read_resource(|value: &WindowScaleFactor| value.0)
            .unwrap_or(1.0);
        let inventory = world
            .read_resource(|menu: &Menu| {
                world.read_resource(|displays: &Displays| {
                    menu.model.needs_sync(displays).then(|| displays.clone())
                })
            })
            .flatten()
            .flatten();
        let input = world
            .read_resource(|input: &InputState| (!input.events().is_empty()).then(|| input.clone()))
            .flatten();
        let exit_key = input
            .as_ref()
            .is_some_and(|input| input.key_just_pressed(KeyCode::Escape));
        let window = world
            .read_resource(|menu: &Menu| {
                world.read_resource(|window: &WindowSettings| {
                    menu.model
                        .graphics
                        .needs_sync(window)
                        .then(|| window.clone())
                })
            })
            .flatten()
            .flatten();
        let result = world.update_resource_with(|menu: &mut Menu| {
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
            if let Some(smoke) = &mut smoke {
                requests.extend(smoke.advance(menu, viewport, dpi)?);
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
    });
    let mut runtime = ApplicationRuntime::new(schedules.build());
    runtime.world().insert_resource(ExitRequest::default());
    Ok(runtime)
}

fn dispatch(world: &mut gridthorn::WorldAccess<'_>, request: Request) {
    match request {
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
