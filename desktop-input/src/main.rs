mod desktop;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    if std::env::args().any(|arg| arg == "--headless") {
        desktop::headless();
    } else {
        let smoke = std::env::args().any(|arg| arg == "--smoke");
        let mut runtime = desktop::runtime(smoke);
        if std::env::args().any(|arg| arg == "--smoke-immediate") {
            runtime
                .world()
                .update_resource(|exit: &mut gridthorn::ExitRequest| exit.request());
        }
        let application = gridthorn::WindowedApplication::new(
            gridthorn::WindowConfig {
                title: "Desktop input: F1 confine, F2 lock, F3 release, Escape exit".into(),
                ..gridthorn::WindowConfig::default()
            },
            runtime,
        );
        if std::env::args().any(|arg| arg == "--gpu") {
            application.run()?;
        } else {
            application.without_renderer().run()?;
        }
    }
    Ok(())
}
