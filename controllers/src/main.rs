mod controller;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::args().any(|arg| arg == "--headless") {
        return controller::headless();
    }
    let smoke = std::env::args().any(|arg| arg == "--smoke");
    gridthorn::WindowedApplication::new(
        gridthorn::WindowConfig {
            title: "Gridthorn · устройства ввода".into(),
            width: 1000,
            height: 800,
        },
        controller::runtime(smoke)?,
    )
    .run()?;
    Ok(())
}
