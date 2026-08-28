use gridthorn_app::{WindowControl, WindowLifecycle};

use super::{ExecutionCounts, ScheduleLifecycle};

#[test]
fn window_lifecycle_runs_startup_fixed_and_update_schedules() {
    let mut lifecycle = ScheduleLifecycle::new(false);
    let mut control = WindowControl::default();

    lifecycle.started(&mut control);
    lifecycle.idle(&mut control);

    assert_eq!(
        lifecycle.counts(),
        Some(ExecutionCounts {
            startup: 1,
            fixed_update: 1,
            update: 1,
        })
    );
}

#[test]
fn smoke_lifecycle_requests_exit_after_three_updates() {
    let mut lifecycle = ScheduleLifecycle::new(true);
    let mut control = WindowControl::default();
    lifecycle.started(&mut control);

    lifecycle.idle(&mut control);
    lifecycle.idle(&mut control);
    lifecycle.idle(&mut control);

    assert_eq!(lifecycle.counts().map(|counts| counts.update), Some(3));
}
