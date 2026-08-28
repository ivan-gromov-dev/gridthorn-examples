use std::time::Instant;

use gridthorn_app::{WindowControl, WindowLifecycle};

use super::{SmokeLifecycle, SmokePhase};

#[test]
fn disabled_smoke_lifecycle_is_inert() {
    let mut lifecycle = SmokeLifecycle::new(false);
    let mut control = WindowControl::default();

    lifecycle.started(&mut control);
    lifecycle.idle(&mut control);

    assert_eq!(lifecycle.phase, SmokePhase::Resize);
}

#[test]
fn due_smoke_lifecycle_requests_resize_first() {
    let mut lifecycle = SmokeLifecycle::new(true);
    lifecycle.deadline = Instant::now();
    let mut control = WindowControl::default();

    lifecycle.idle(&mut control);

    assert_eq!(lifecycle.phase, SmokePhase::Minimize);
}
