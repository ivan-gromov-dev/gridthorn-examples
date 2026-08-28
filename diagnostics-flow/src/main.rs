mod diagnostics;
mod lifecycle;

use gridthorn_app::{WindowApplication, WindowConfig};
use lifecycle::DiagnosticLifecycle;

fn main() -> Result<(), gridthorn_app::ApplicationError> {
    diagnostics::init();
    WindowApplication::new(
        WindowConfig {
            title: "Gridthorn diagnostics flow".to_owned(),
            ..WindowConfig::default()
        },
        DiagnosticLifecycle::default(),
    )
    .run()
}
