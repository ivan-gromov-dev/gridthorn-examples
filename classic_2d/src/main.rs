mod audio;
mod game;
mod presentation;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let headless = std::env::args().any(|arg| arg == "--headless-smoke");
    let smoke = std::env::args().any(|arg| arg == "--smoke");
    if headless {
        game::smoke()?;
    } else {
        let runtime = game::runtime(true, smoke)?;
        gridthorn::WindowedApplication::new(
            gridthorn::WindowConfig {
                title: "Crystal trail - Gridthorn".into(),
                ..Default::default()
            },
            runtime,
        )
        .run()?;
    }
    Ok(())
}
