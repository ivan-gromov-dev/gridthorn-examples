use super::{runtime, session::Session};
use gridthorn::RenderFrame;
use std::time::Duration;

/// Exercise game integration without a native window, GPU, or audio device.
///
/// # Errors
/// Returns initialization or frame lifecycle failures.
///
/// # Panics
/// Fails assertions if texture/UI extraction or routed delivery regresses.
pub fn smoke() -> Result<(), Box<dyn std::error::Error>> {
    let mut runtime = runtime(false, true)?;
    for _ in 0..60 {
        runtime.run_timed_frame(Duration::from_millis(100))?;
    }
    assert!(
        runtime
            .world()
            .read_resource(|f: &RenderFrame| f.textured_sprites().len() > 144 && f.ui().len() > 20)
            .unwrap()
    );
    assert!(
        runtime
            .world()
            .read_resource(|s: &Session| s.data.shipped > 0)
            .unwrap()
    );
    runtime.shutdown();
    println!(
        "Timber Harbor: input, scenes, textured extraction, UI, routing, economy, audio decode and shutdown passed"
    );
    Ok(())
}
