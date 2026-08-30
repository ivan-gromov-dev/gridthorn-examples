use gridthorn_app::{WindowControl, WindowLifecycle};

use super::DiagnosticLifecycle;

#[test]
fn lifecycle_completes_after_three_updates() {
    let mut lifecycle = DiagnosticLifecycle::default();
    let mut control = WindowControl::default();

    lifecycle
        .started(&mut control)
        .expect("lifecycle should start");
    lifecycle.idle(&mut control).expect("frame should run");
    lifecycle.idle(&mut control).expect("frame should run");
    lifecycle.idle(&mut control).expect("frame should run");
    lifecycle.idle(&mut control).expect("frame should run");

    assert_eq!(lifecycle.updates, 3);
}
