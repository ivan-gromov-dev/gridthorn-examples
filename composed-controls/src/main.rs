mod interface;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::args().any(|argument| argument == "--headless") {
        return interface::headless();
    }
    let smoke = std::env::args().any(|argument| argument == "--smoke");
    gridthorn::WindowedApplication::new(
        gridthorn::WindowConfig {
            title: "Gridthorn: composed controls".into(),
            width: 900,
            height: 700,
        },
        interface::runtime(smoke)?,
    )
    .run()?;
    Ok(())
}
