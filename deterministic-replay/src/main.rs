mod scenario;

use scenario::{ScenarioInput, SimulationCommand, run_scenario};
use std::process::ExitCode;

fn main() -> ExitCode {
    let input = ScenarioInput {
        seed: 0x5eed,
        fixed_steps: 12,
        commands: vec![
            SimulationCommand { tick: 1, delta: 4 },
            SimulationCommand { tick: 4, delta: -3 },
            SimulationCommand { tick: 4, delta: 9 },
            SimulationCommand { tick: 10, delta: 2 },
        ],
    };
    let first = run_scenario(input.clone());
    let second = run_scenario(input);

    if first != second {
        eprintln!("deterministic replay failed: repeated runs produced different fingerprints");
        return ExitCode::FAILURE;
    }

    println!("deterministic replay completed: fingerprint={first:016x}");
    ExitCode::SUCCESS
}
