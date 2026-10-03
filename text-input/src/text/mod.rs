mod smoke;
#[cfg(test)]
mod test;
use gridthorn::{
    ApplicationRuntime, Clipboard, ExitRequest, ImeCursorArea, InputBuffer, InputEvent, InputState,
    KeyCode, ScheduleBuilder, ScheduleStage, TextInput, TextInputEvent,
};

#[derive(Default)]
struct Document(String);

fn anchor() -> ImeCursorArea {
    ImeCursorArea::new(24.0, 40.0, 1.0, 20.0).unwrap()
}

/// Compose text events and physical shortcuts through the public SDK.
pub(crate) fn runtime(smoke: bool) -> ApplicationRuntime {
    let mut schedules = ScheduleBuilder::new();
    schedules.add_system(ScheduleStage::Startup, |world| {
        world.insert_resource(Document::default());
        world.update_resource(|text: &mut TextInput| text.start(anchor()).unwrap());
    });
    schedules.add_system(ScheduleStage::Input, move |world| {
        let input = world
            .read_resource(Clone::clone)
            .unwrap_or_else(InputState::default);
        for event in input.events() {
            match event {
                InputEvent::Text(TextInputEvent::Commit(text)) => {
                    world.update_resource(|document: &mut Document| document.0.push_str(text));
                    println!("Committed: {text:?}");
                }
                InputEvent::Clipboard(response) if response.id == 2 => match &response.result {
                    Ok(Some(text)) => {
                        world.update_resource(|document: &mut Document| document.0.push_str(text));
                    }
                    result => println!("Paste: {result:?}"),
                },
                InputEvent::Clipboard(response) if response.id == 1 => {
                    if let Err(error) = &response.result {
                        println!("Copy failed: {error}");
                    }
                }
                InputEvent::Text(text) => println!("IME: {text:?}"),
                InputEvent::TextInputChanged { active, error } => {
                    println!("Text session: {active}, {error:?}");
                }
                _ => {}
            }
        }
        if input.key_just_pressed(KeyCode::F4) || input.events().contains(&InputEvent::FocusGained)
        {
            world.update_resource(|text: &mut TextInput| text.start(anchor()).unwrap());
        }
        if input.key_just_pressed(KeyCode::F5) {
            world.update_resource(TextInput::stop);
        }
        if input.modifiers().control || input.modifiers().super_key {
            if input.key_just_pressed(KeyCode::KeyC) {
                let text = world
                    .read_resource(|document: &Document| document.0.clone())
                    .unwrap();
                world.update_resource(|clipboard: &mut Clipboard| clipboard.write(1, text));
            }
            if input.key_just_pressed(KeyCode::KeyV) {
                world.update_resource(|clipboard: &mut Clipboard| clipboard.read(2));
            }
        }
        if input.key_just_pressed(KeyCode::Escape) && input.composition().is_none() {
            world.update_resource(|exit: &mut ExitRequest| exit.request());
        }
        if !smoke {
            world.read_resource(|document: &Document| {
                if !input.events().is_empty() {
                    println!(
                        "Document: {:?}; preedit: {:?}",
                        document.0,
                        input.composition()
                    );
                }
            });
        }
    });
    if smoke {
        smoke::register(&mut schedules);
    }
    ApplicationRuntime::new(schedules.build())
}

/// Validate Unicode insertion, preedit cancellation and physical shortcut separation headlessly.
pub(crate) fn headless() {
    let mut buffer = InputBuffer::new();
    buffer.push(InputEvent::Text(TextInputEvent::Commit(
        "Привет e\u{301} 👩‍💻".into(),
    )));
    buffer.push(InputEvent::Text(TextInputEvent::Composition {
        text: "日本".into(),
        cursor: Some((3, 6)),
    }));
    assert_eq!(
        buffer.snapshot().composition(),
        Some(("日本", Some((3, 6))))
    );
    buffer.push(InputEvent::Text(TextInputEvent::Commit("日本".into())));
    let frame = buffer.snapshot();
    assert!(frame.composition().is_none());
    assert!(!frame.key_down(KeyCode::KeyQ));
    buffer.push(InputEvent::Text(TextInputEvent::Composition {
        text: "你".into(),
        cursor: None,
    }));
    buffer.push(InputEvent::FocusLost);
    assert!(buffer.snapshot().composition().is_none());
    assert_eq!(buffer.snapshot().events(), []);
    println!("Unicode/IME headless smoke passed");
}
