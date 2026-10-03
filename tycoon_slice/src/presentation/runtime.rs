use super::{Assets, interface, world};
use crate::game::session::Session;
use gridthorn::{
    FrameTiming, GameStateStack, RenderFrame, ScheduleBuilder, ScheduleStage, SimulationControl,
    WindowViewport,
};

pub fn register(schedules: &mut ScheduleBuilder, assets: Assets) {
    let mut assets = Some(assets);
    schedules.add_system(ScheduleStage::Startup, move |world| {
        world.insert_resource(assets.take().expect("assets startup once"));
    });
    schedules.add_system(ScheduleStage::PollEvents, |world| {
        world.update_resource(Assets::poll);
    });
    schedules.add_system(ScheduleStage::Render, |world| {
        let elapsed = world
            .read_resource(|t: &FrameTiming| t.frame_elapsed())
            .unwrap_or_default();
        let active = world
            .read_resource(|s: &GameStateStack| s.current().as_str().to_owned())
            .expect("state");
        let control = world
            .read_resource(|c: &SimulationControl| *c)
            .unwrap_or_default();
        let viewport = world
            .read_resource(|v: &WindowViewport| *v)
            .expect("viewport");
        let (texture, expansion, skin, worker, reloads, error) = world
            .update_resource_with(|assets: &mut Assets| assets.snapshot(elapsed))
            .expect("assets");
        let mut frame = world
            .read_resource(|s: &Session| world::extract(s, &texture, &expansion, worker))
            .expect("session");
        let ui = world
            .read_resource(|s: &Session| {
                interface::extract(s, &active, control, viewport, reloads, error.as_deref())
            })
            .expect("session");
        let mut sprites = frame.textured_sprites().to_vec();
        sprites.extend(
            world
                .read_resource(|s: &Session| {
                    super::skin::extract(s, &skin, &active, control, viewport)
                })
                .expect("skin"),
        );
        frame = frame.with_textured_sprites(sprites).with_ui(ui);
        if let Some(values) = world
            .read_resource(|i: &crate::game::inspection::Inspector| format!("INSPECTOR {:?}", i.0))
            && active == "play"
            && world.read_resource(|s: &Session| s.paths).unwrap_or(false)
        {
            let (scale, _, _) = crate::game::session::ui_layout(viewport);
            let mut ui = frame.ui().to_vec();
            ui.push(
                gridthorn::TextLabel::new(
                    values,
                    [250.0 * scale, 174.0 * scale],
                    scale,
                    gridthorn::Color::rgb(0.7, 0.85, 0.8),
                )
                .expect("inspector label")
                .into(),
            );
            frame = frame.with_ui(ui);
        }
        world.insert_resource::<RenderFrame>(frame);
    });
    schedules.add_system(ScheduleStage::Shutdown, |world| {
        world.update_resource(Assets::shutdown);
    });
}
