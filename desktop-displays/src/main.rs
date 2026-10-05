mod settings;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::args().any(|arg| arg == "--headless") {
        return settings::headless();
    }
    let smoke = std::env::args().any(|arg| arg == "--smoke");
    gridthorn::WindowedApplication::new(
        gridthorn::WindowConfig {
            title: "Gridthorn — Настройки игры".into(),
            width: 1100,
            height: 860,
        },
        settings::runtime(smoke)?,
    )
    .run()?;
    Ok(())
}
