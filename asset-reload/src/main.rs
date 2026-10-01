mod game;
mod smoke;

use gridthorn::{WindowConfig, WindowedApplication};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::args().any(|argument| argument == "--smoke") {
        return smoke::run();
    }
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("assets");
    let runtime = game::runtime(&root, std::time::Duration::from_millis(250))?;
    println!(
        "Edit {} while this window is open.",
        root.join("sprite.ppm").display()
    );
    WindowedApplication::new(
        WindowConfig {
            title: "Gridthorn asset hot reload".to_owned(),
            ..WindowConfig::default()
        },
        runtime,
    )
    .run()?;
    Ok(())
}
