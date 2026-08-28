use super::{HeadlessScenario, SimulationSnapshot};

#[test]
fn zero_step_run_executes_only_startup() {
    let snapshot = HeadlessScenario::new().run(0);

    assert_eq!(
        snapshot,
        SimulationSnapshot {
            startup_runs: 1,
            fixed_steps: 0,
            value: 0,
        }
    );
}

#[test]
fn requested_steps_execute_without_application_or_renderer() {
    let snapshot = HeadlessScenario::new().run(4);

    assert_eq!(
        snapshot,
        SimulationSnapshot {
            startup_runs: 1,
            fixed_steps: 4,
            value: 280,
        }
    );
}
