mod arguments;
mod scenario;

use arguments::HeadlessArguments;
use scenario::HeadlessScenario;
use std::process::ExitCode;

fn main() -> ExitCode {
    match execute() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("headless example failed: {error}");
            ExitCode::FAILURE
        }
    }
}

fn execute() -> Result<(), arguments::ArgumentError> {
    let arguments = HeadlessArguments::parse(std::env::args().skip(1))?;
    let snapshot = HeadlessScenario::new().run(arguments.fixed_steps);
    println!(
        "headless schedule completed: startup={}, fixed_steps={}, value={}",
        snapshot.startup_runs, snapshot.fixed_steps, snapshot.value
    );
    Ok(())
}
