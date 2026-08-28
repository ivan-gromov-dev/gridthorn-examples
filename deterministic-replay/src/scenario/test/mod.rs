use super::{ScenarioInput, SimulationCommand, run_scenario};

fn input() -> ScenarioInput {
    ScenarioInput {
        seed: 42,
        fixed_steps: 8,
        commands: vec![
            SimulationCommand { tick: 1, delta: 5 },
            SimulationCommand { tick: 5, delta: -2 },
        ],
    }
}

#[test]
fn identical_input_produces_identical_fingerprint() {
    assert_eq!(run_scenario(input()), run_scenario(input()));
}

#[test]
fn changed_seed_changes_fingerprint() {
    let baseline = input();
    let mut changed = baseline.clone();
    changed.seed += 1;

    assert_ne!(run_scenario(baseline), run_scenario(changed));
}

#[test]
fn changed_command_changes_fingerprint() {
    let baseline = input();
    let mut changed = baseline.clone();
    changed.commands[0].delta += 1;

    assert_ne!(run_scenario(baseline), run_scenario(changed));
}
