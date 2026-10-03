mod text;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::args().any(|arg| arg == "--headless") {
        text::headless();
        return Ok(());
    }
    let smoke = std::env::args().any(|arg| arg == "--smoke");
    gridthorn::WindowedApplication::new(
        gridthorn::WindowConfig {
            title: "Unicode/IME: F4 start, F5 stop, Ctrl+C copy, Ctrl+V paste, Escape exit".into(),
            ..gridthorn::WindowConfig::default()
        },
        text::runtime(smoke),
    )
    .without_renderer()
    .run()?;
    Ok(())
}
