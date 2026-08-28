use gridthorn_app::{WindowControl, WindowLifecycle};

use super::DiagnosticLifecycle;

#[test]
fn lifecycle_completes_after_three_updates() {
    let mut lifecycle = DiagnosticLifecycle::default();
    let mut control = WindowControl::default();

    lifecycle.started(&mut control);
    lifecycle.idle(&mut control);
    lifecycle.idle(&mut control);
    lifecycle.idle(&mut control);
    lifecycle.idle(&mut control);

    assert_eq!(lifecycle.updates, 3);
}
