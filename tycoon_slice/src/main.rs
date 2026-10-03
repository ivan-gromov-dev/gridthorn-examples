fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::args().any(|arg| arg == "--headless-smoke") {
        return tycoon_slice::smoke();
    }
    let smoke = std::env::args().any(|arg| arg == "--smoke");
    let native_audio = !std::env::args().any(|arg| arg == "--silent");
    let runtime = tycoon_slice::runtime(native_audio, smoke)?;
    gridthorn::WindowedApplication::new(
        gridthorn::WindowConfig {
            title: "Timber Harbor | Gridthorn tycoon slice".into(),
            width: 1280,
            height: 800,
        },
        runtime,
    )
    .run()?;
    Ok(())
}
