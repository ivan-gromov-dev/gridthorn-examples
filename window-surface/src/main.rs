mod diagnostics;
mod smoke;

use gridthorn_app::{WindowApplication, WindowConfig};
use smoke::SmokeLifecycle;

fn main() -> Result<(), gridthorn_app::ApplicationError> {
    diagnostics::init();
    let smoke_enabled = std::env::args()
        .skip(1)
        .any(|argument| argument == "--smoke");
    let config = WindowConfig {
        title: "Gridthorn window surface example".to_owned(),
        ..WindowConfig::default()
    };
    WindowApplication::new(config, SmokeLifecycle::new(smoke_enabled)).run()
}
