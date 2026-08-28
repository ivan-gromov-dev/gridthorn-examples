mod diagnostics;
mod lifecycle;

use gridthorn_app::{WindowApplication, WindowConfig};
use lifecycle::ScheduleLifecycle;

fn main() -> Result<(), gridthorn_app::ApplicationError> {
    diagnostics::init();
    let smoke_enabled = std::env::args()
        .skip(1)
        .any(|argument| argument == "--smoke");
    let config = WindowConfig {
        title: "Gridthorn ECS schedule example".to_owned(),
        ..WindowConfig::default()
    };
    WindowApplication::new(config, ScheduleLifecycle::new(smoke_enabled)).run()
}
