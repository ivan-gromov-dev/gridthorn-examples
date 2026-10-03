use super::{Document, runtime};
use gridthorn::{Clipboard, ClipboardResponse, InputBuffer, InputEvent, TextInput, TextInputEvent};

#[test]
fn public_example_appends_committed_and_pasted_unicode_in_order() {
    let mut app = runtime(false);
    app.world().insert_resource(TextInput::default());
    app.world().insert_resource(Clipboard::default());
    let mut buffer = InputBuffer::new();
    buffer.push(InputEvent::Text(TextInputEvent::Composition {
        text: "日本".into(),
        cursor: Some((3, 6)),
    }));
    buffer.push(InputEvent::Text(TextInputEvent::Commit(
        "Привет e\u{301} ".into(),
    )));
    buffer.push(InputEvent::Clipboard(ClipboardResponse {
        id: 2,
        result: Ok(Some("日本 👩‍💻".into())),
    }));
    app.world().insert_resource(buffer.snapshot());
    app.run_frame(0).unwrap();
    assert_eq!(
        app.world()
            .read_resource(|document: &Document| document.0.clone()),
        Some("Привет e\u{301} 日本 👩‍💻".into())
    );
    app.world().insert_resource(buffer.snapshot());
    app.run_frame(0).unwrap();
    assert_eq!(
        app.world()
            .read_resource(|document: &Document| document.0.clone()),
        Some("Привет e\u{301} 日本 👩‍💻".into())
    );
}
