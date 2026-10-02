pub(crate) mod model;

use gridthorn::prelude::*;
use model::{Arena, Session, scene, state};
use std::time::Duration;

pub(crate) fn runtime(
    native_audio: bool,
    smoke: bool,
) -> Result<ApplicationRuntime, Box<dyn std::error::Error>> {
    let assets = crate::presentation::Assets::load()?;
    let audio = crate::audio::Sound::new(native_audio)?;
    let mut schedules = ScheduleBuilder::new();
    register_startup(&mut schedules);
    schedules.add_system(ScheduleStage::Input, move |world| {
        let input = world
            .read_resource(Clone::clone)
            .unwrap_or_else(InputState::default);
        let active = world
            .read_resource(|stack: &GameStateStack| stack.current().as_str().to_owned())
            .unwrap();
        let clicked = world
            .update_resource_with(|session: &mut Session| {
                let interaction = session.button.update(&input);
                session.hovered = interaction.hovered();
                session.movement = [
                    i16::from(input.key_down(KeyCode::KeyD) || input.key_down(KeyCode::ArrowRight))
                        - i16::from(
                            input.key_down(KeyCode::KeyA) || input.key_down(KeyCode::ArrowLeft),
                        ),
                    i16::from(input.key_down(KeyCode::KeyS) || input.key_down(KeyCode::ArrowDown))
                        - i16::from(
                            input.key_down(KeyCode::KeyW) || input.key_down(KeyCode::ArrowUp),
                        ),
                ];
                interaction.activated()
            })
            .unwrap();
        let activate =
            clicked || input.key_just_pressed(KeyCode::Enter) || (smoke && active == "menu");
        if activate && (active == "menu" || active == "won") {
            world.update_resource(|stack: &mut GameStateStack| stack.request_set(state("play")));
            world.update_resource(|scenes: &mut SceneController| {
                scenes.request_switch(scene("arena"));
            });
        } else if (activate && active == "paused") || input.key_just_pressed(KeyCode::Space) {
            world.update_resource(|stack: &mut GameStateStack| {
                if active == "play" {
                    stack.request_push(state("paused"));
                } else if active == "paused" {
                    stack.request_pop().expect("pause has parent");
                }
            });
        }
        if input.key_just_pressed(KeyCode::Escape) {
            world.update_resource(|exit: &mut ExitRequest| exit.request());
        }
    });
    schedules.add_system(ScheduleStage::SceneTransition, |world| {
        let active = world
            .read_resource(|scenes: &SceneController| scenes.current().clone())
            .unwrap();
        let entity =
            (active == scene("arena")).then(|| world.spawn_in_scene(active, Arena::default()));
        world.update_resource(|session: &mut Session| session.entity = entity);
    });
    let sound = audio.clone();
    let shutdown_sound = audio.clone();
    schedules.add_system(ScheduleStage::Shutdown, move |_| shutdown_sound.shutdown());
    schedules.add_system(ScheduleStage::FixedUpdate, move |world| {
        if !world
            .read_resource(|stack: &GameStateStack| stack.current() == &state("play"))
            .unwrap_or(false)
        {
            return;
        }
        let (entity, movement) = world
            .read_resource(|s: &Session| (s.entity, s.movement))
            .unwrap();
        if let Some(entity) = entity {
            let mut pickups = 0;
            world.update_component(entity, |arena: &mut Arena| pickups = arena.step(movement));
            if pickups > 0 {
                sound.pickup();
            }
            if world
                .read_component(entity, |arena: &Arena| {
                    arena.collected.iter().all(|gem| *gem)
                })
                .unwrap_or(false)
            {
                world.update_resource(|stack: &mut GameStateStack| stack.request_set(state("won")));
                world.update_resource(|scenes: &mut SceneController| {
                    scenes.request_switch(scene("won"));
                });
            }
        }
    });
    schedules.add_system(ScheduleStage::Update, move |world| {
        let paused = world
            .read_resource(|s: &GameStateStack| s.current() == &state("paused"))
            .unwrap();
        audio.pause(paused);
        world.update_resource(|session: &mut Session| session.frames += 1);
        if smoke && world.read_resource(|s: &Session| s.frames >= 30).unwrap() {
            world.update_resource(|exit: &mut ExitRequest| exit.request());
        }
    });
    crate::presentation::register(&mut schedules, assets);
    Ok(ApplicationRuntime::new(schedules.build()))
}

pub(crate) fn smoke() -> Result<(), Box<dyn std::error::Error>> {
    let mut runtime = runtime(false, true)?;
    for _ in 0..30 {
        runtime.run_timed_frame(Duration::from_millis(17))?;
    }
    assert!(
        runtime
            .world()
            .read_resource(|s: &Session| s.entity.is_some())
            .unwrap()
    );
    assert!(
        runtime
            .world()
            .read_resource(|frame: &RenderFrame| !frame.textured_sprites().is_empty()
                && !frame.ui().is_empty())
            .unwrap()
    );
    runtime.shutdown();
    println!("classic_2d headless smoke passed");
    Ok(())
}

fn register_startup(schedules: &mut ScheduleBuilder) {
    schedules.add_system(ScheduleStage::Startup, move |world| {
        world.insert_resource(GameStateStack::new(state("menu")));
        world.insert_resource(SceneController::new(scene("menu")));
        world.insert_resource(Session {
            entity: None,
            movement: [0, 0],
            button: UiButton::new([24.0, 110.0], [240.0, 44.0]).expect("button bounds"),
            hovered: false,
            frames: 0,
        });
    });
}

#[cfg(test)]
mod test;
