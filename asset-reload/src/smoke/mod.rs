use std::path::PathBuf;
use std::time::{Duration, Instant};

use gridthorn::{ApplicationRuntime, AssetId, RenderFrame, TextureAsset};

use crate::game::{self, ReloadState};

struct Scratch(PathBuf);

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// Exercise real file edits and dependency propagation through headless frames.
pub(crate) fn run() -> Result<(), Box<dyn std::error::Error>> {
    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_nanos();
    let directory = std::env::temp_dir().join(format!(
        "gridthorn-reload-smoke-{}-{unique}",
        std::process::id()
    ));
    std::fs::create_dir(&directory)?;
    let scratch = Scratch(directory);
    let path = scratch.0.join("sprite.ppm");
    std::fs::write(&path, b"P3\n1 1\n255\n255 0 0\n")?;
    std::fs::write(scratch.0.join("scene.txt"), b"sprite.ppm\n")?;
    let mut runtime = game::runtime(&scratch.0, Duration::from_millis(2))?;
    runtime.run_frame(1)?;
    let original = texture(&mut runtime);
    std::fs::write(&path, b"P3\n1 1\n255\n0 255 0\n")?;
    assert_eq!(texture(&mut runtime).rgba8(), original.rgba8());
    advance_until(&mut runtime, |runtime| {
        texture(runtime).rgba8() == [0, 255, 0, 255]
    })?;
    let updated = texture(&mut runtime);
    assert_eq!(updated.rgba8(), [0, 255, 0, 255]);
    assert_eq!(original.rgba8(), [255, 0, 0, 255]);
    assert_eq!(
        runtime
            .world()
            .read_resource(|state: &ReloadState| state.changed.clone()),
        Some(vec![
            AssetId::new("sprite.ppm")?,
            AssetId::new("scene.txt")?
        ])
    );
    std::fs::write(&path, b"partial write")?;
    advance_until(&mut runtime, |runtime| {
        runtime
            .world()
            .read_resource(|state: &ReloadState| state.error.is_some())
            .unwrap()
    })?;
    assert!(texture(&mut runtime).shares_data_with(&updated));
    assert!(
        runtime
            .world()
            .read_resource(|state: &ReloadState| state.error.is_some())
            .unwrap()
    );
    std::fs::write(&path, b"P3\n1 1\n255\n0 0 255\n")?;
    advance_until(&mut runtime, |runtime| {
        texture(runtime).rgba8() == [0, 0, 255, 255]
    })?;
    assert_eq!(texture(&mut runtime).rgba8(), [0, 0, 255, 255]);
    assert!(
        runtime
            .world()
            .read_resource(|state: &ReloadState| state.error.is_none())
            .unwrap()
    );
    runtime.shutdown();
    println!(
        "Background asset reload smoke passed: frame boundary, dependencies, rollback, recovery, shutdown."
    );
    Ok(())
}

fn texture(runtime: &mut ApplicationRuntime) -> TextureAsset {
    runtime
        .world()
        .read_resource(|frame: &RenderFrame| frame.textured_sprites()[0].texture().clone())
        .unwrap()
}

fn advance_until(
    runtime: &mut ApplicationRuntime,
    ready: impl Fn(&mut ApplicationRuntime) -> bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        runtime.run_frame(1)?;
        if ready(runtime) {
            return Ok(());
        }
        if Instant::now() >= deadline {
            return Err(std::io::Error::new(
                std::io::ErrorKind::TimedOut,
                "background reload did not reach the expected frame state",
            )
            .into());
        }
        std::thread::sleep(Duration::from_millis(1));
    }
}

#[cfg(test)]
mod test;
