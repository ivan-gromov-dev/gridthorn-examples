use super::session::Session;
use super::{input, inspection, session};
use gridthorn::SimulationControl;
use gridthorn::prelude::*;

/// Build the playable harbor through public SDK schedules and presentation services.
///
/// # Errors
/// Returns contextual initialization failures for simulation, assets, and audio decoding.
///
/// # Panics
/// Schedule execution requires the resources installed here to remain present.
pub fn runtime(
    native_audio: bool,
    smoke: bool,
) -> Result<ApplicationRuntime, Box<dyn std::error::Error>> {
    let session = Session::new("harbor")?;
    let assets = crate::presentation::Assets::new()?;
    let sound = crate::audio::Sound::new(native_audio)?;
    let mut schedules = ScheduleBuilder::new();
    schedules.add_system(ScheduleStage::Startup, |world| {
        world.insert_resource(GameStateStack::new(session::state("menu")));
        world.insert_resource(SceneController::new(session::scene("menu")));
        let mut control = SimulationControl::default();
        control.pause();
        world.insert_resource(control);
    });
    schedules.add_system(ScheduleStage::SceneTransition, |world| {
        let scene = world
            .read_resource(|s: &SceneController| s.current().clone())
            .expect("scene");
        world.spawn_in_scene(scene, inspection::SceneBanner);
    });
    input::register(&mut schedules, smoke);
    let effects = sound.clone();
    schedules.add_system(ScheduleStage::FixedUpdate, move |world| {
        let play = world
            .read_resource(|s: &GameStateStack| s.current().as_str() == "play")
            .unwrap_or(false);
        if play {
            world.update_resource(|session: &mut Session| {
                let shipped = session.data.shipped;
                if let Err(error) = session.advance() {
                    session.notice = error.to_string();
                }
                if session.data.shipped > shipped {
                    effects.pickup();
                }
            });
        }
    });
    let audio = sound.clone();
    schedules.add_system(ScheduleStage::Update, move |world| {
        let control = world
            .read_resource(|c: &SimulationControl| *c)
            .unwrap_or_default();
        audio.pause(control.is_paused());
        let frames = world
            .update_resource_with(|session: &mut Session| {
                session.frames += 1;
                session.frames
            })
            .expect("session");
        if smoke && frames >= 60 {
            world.update_resource(|e: &mut ExitRequest| e.request());
        }
        inspection::publish(world);
    });
    crate::presentation::register(&mut schedules, assets);
    schedules.add_system(ScheduleStage::Shutdown, move |world| {
        sound.shutdown();
        world.update_resource(|s: &mut Session| s.runner.shutdown());
    });
    let mut runtime =
        ApplicationRuntime::with_fixed_step(schedules.build(), crate::economy::config());
    runtime.world().insert_resource(session);
    runtime.world().insert_resource(gridthorn::WindowViewport {
        width: 1280,
        height: 800,
    });
    Ok(runtime)
}
