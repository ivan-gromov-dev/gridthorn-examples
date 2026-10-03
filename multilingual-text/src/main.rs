mod text;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::args().any(|argument| argument == "--headless") {
        text::headless()?;
        return Ok(());
    }
    let smoke = std::env::args().any(|argument| argument == "--smoke");
    gridthorn::WindowedApplication::new(
        gridthorn::WindowConfig {
            title: "Gridthorn: Cyrillic / Arabic bidi / Japanese / DPI".into(),
            width: 1000,
            height: 720,
        },
        text::runtime(smoke)?,
    )
    .run()?;
    Ok(())
}
