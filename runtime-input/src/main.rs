mod game;

use gridthorn::prelude::{WindowConfig, WindowedApplication};

fn main() -> Result<(), gridthorn::ApplicationError> {
    let smoke_enabled = std::env::args()
        .skip(1)
        .any(|argument| argument == "--smoke");
    let config = WindowConfig {
        title: "Gridthorn runtime input example".to_owned(),
        ..WindowConfig::default()
    };
    WindowedApplication::new(config, game::runtime(smoke_enabled)).run()
}
