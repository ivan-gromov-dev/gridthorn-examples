use std::path::Path;
use std::time::{Duration, Instant};

use gridthorn::{
    ApplicationRuntime, AssetId, AssetReloadError, AssetReloader, AssetStore, Camera2d,
    RenderFrame, ScheduleBuilder, ScheduleStage, TexturedSprite,
};

pub(crate) struct ReloadState {
    reloader: AssetReloader,
    next_request: Instant,
    interval: Duration,
    stopped: bool,
    pub(crate) changed: Vec<AssetId>,
    pub(crate) error: Option<String>,
}

/// Publish prepared background assets before frame systems and extract a snapshot.
pub(crate) fn runtime(
    root: &Path,
    interval: Duration,
) -> Result<ApplicationRuntime, Box<dyn std::error::Error>> {
    let texture_id = AssetId::new("sprite.ppm")?;
    let scene_id = AssetId::new("scene.txt")?;
    let mut assets = AssetStore::new(root);
    assets.load_texture(texture_id.clone())?;
    assets.load_source(scene_id.clone())?;
    assets.set_dependencies(&scene_id, std::slice::from_ref(&texture_id))?;
    let mut schedules = ScheduleBuilder::new();
    schedules.add_system(ScheduleStage::PollEvents, |world| {
        world.update_resource(ReloadState::advance);
    });
    schedules.add_system(ScheduleStage::Render, move |world| {
        let texture = world
            .read_resource(|state: &ReloadState| {
                state.reloader.assets().texture(&texture_id).cloned()
            })
            .flatten();
        if let Some(texture) = texture {
            world.insert_resource(
                RenderFrame::new(Camera2d::default(), Vec::new()).with_textured_sprites(vec![
                    TexturedSprite::new([0.0, 0.0], [160.0, 160.0], texture),
                ]),
            );
        }
    });
    schedules.add_system(ScheduleStage::Shutdown, |world| {
        world.update_resource(|state: &mut ReloadState| {
            if let Err(error) = state.reloader.shutdown() {
                state.report(&error);
            }
            state.stopped = true;
        });
    });
    let mut runtime = ApplicationRuntime::new(schedules.build());
    runtime.world().insert_resource(ReloadState {
        reloader: AssetReloader::new(assets)?,
        next_request: Instant::now(),
        interval,
        stopped: false,
        changed: Vec::new(),
        error: None,
    });
    Ok(runtime)
}

impl ReloadState {
    fn advance(&mut self) {
        self.changed.clear();
        if self.stopped {
            return;
        }
        match self.reloader.poll() {
            Ok(Some(changed)) => {
                if !changed.is_empty() {
                    println!("Reloaded: {changed:?}");
                }
                self.changed = changed;
                self.error = None;
            }
            Ok(None) => {}
            Err(error) => self.report(&error),
        }
        let now = Instant::now();
        if !self.stopped && now >= self.next_request && !self.reloader.is_pending() {
            match self.reloader.request_reload() {
                Ok(true) => self.next_request = now + self.interval,
                Ok(false) => {}
                Err(error) => self.report(&error),
            }
        }
    }

    fn report(&mut self, error: &AssetReloadError) {
        let message = error.to_string();
        if self.error.as_ref() != Some(&message) {
            eprintln!("{message}; keeping the last good frame assets");
        }
        self.stopped = matches!(error, AssetReloadError::Stopped);
        self.error = Some(message);
    }
}
