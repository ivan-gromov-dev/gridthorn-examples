mod adapters;
mod settings;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let selection = adapters::selection()?;
    if selection.list_only {
        return Ok(());
    }
    if std::env::args().any(|arg| arg == "--headless") {
        return settings::headless();
    }
    let smoke = std::env::args().any(|arg| arg == "--smoke");
    let runtime = if std::env::args().any(|arg| arg == "--adapter-smoke") {
        adapters::smoke_runtime(selection.graphics.clone())
    } else {
        settings::runtime(smoke, selection.preferences, selection.graphics.clone())?
    };
    let mut application = gridthorn::WindowedApplication::new(
        gridthorn::WindowConfig {
            title: "Gridthorn — Настройки игры".into(),
            width: 1100,
            height: 860,
        },
        runtime,
    );
    application = application.with_graphics_selection(selection.graphics);
    application.run()?;
    Ok(())
}
