use gridthorn::{Clipboard, ExitRequest, InputEvent, InputState, ScheduleBuilder, ScheduleStage};
use std::time::{Duration, Instant};

struct Smoke {
    started: Instant,
    requested: bool,
    original: Option<String>,
    passed: bool,
}

/// Exercise native text-session setup and clipboard round-trip while restoring original text.
pub(super) fn register(schedules: &mut ScheduleBuilder) {
    schedules.add_system(ScheduleStage::Startup, |world| {
        world.insert_resource(Smoke {
            started: Instant::now(),
            requested: false,
            original: None,
            passed: false,
        });
    });
    schedules.add_system(ScheduleStage::Input, |world| {
        let input = world.read_resource(Clone::clone).unwrap_or_else(InputState::default);
        let request = world.update_resource_with(|state: &mut Smoke| {
            assert!(state.started.elapsed() < Duration::from_secs(10), "native smoke timeout");
            if input.text_input_active() && !state.requested {
                state.requested = true;
                true
            } else { false }
        }).unwrap();
        if request { world.update_resource(|clipboard: &mut Clipboard| clipboard.read(10)); }
        for event in input.events() {
            if let InputEvent::Clipboard(response) = event {
                match response.id {
                    10 => {
                        if let Ok(Some(original)) = &response.result {
                            world.update_resource(|state: &mut Smoke| state.original = Some(original.clone()));
                            world.update_resource(|clipboard: &mut Clipboard| { clipboard.write(11, "Gridthorn Привет 日本 e\u{301} 👩‍💻"); clipboard.read(12); });
                        } else {
                            println!("Native text session passed; clipboard round-trip skipped: original text unavailable");
                            world.update_resource(|exit: &mut ExitRequest| exit.request());
                        }
                    }
                    12 => {
                        let passed = response.result == Ok(Some("Gridthorn Привет 日本 e\u{301} 👩‍💻".into()));
                        let original = world.read_resource(|state: &Smoke| state.original.clone()).flatten().unwrap();
                        world.update_resource(|state: &mut Smoke| state.passed = passed);
                        world.update_resource(|clipboard: &mut Clipboard| clipboard.write(13, original));
                    }
                    13 => {
                        assert_eq!(response.result, Ok(None), "clipboard restoration failed");
                        assert_eq!(world.read_resource(|state: &Smoke| state.passed), Some(true), "clipboard round-trip failed");
                        println!("Native text session and Unicode clipboard round-trip passed; original text restored");
                        world.update_resource(|exit: &mut ExitRequest| exit.request());
                    }
                    _ => {}
                }
            }
        }
    });
}
