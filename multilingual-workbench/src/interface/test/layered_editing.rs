use std::{hint::black_box, time::Instant};

use gridthorn::ui::{
    UiControl, UiLayer, UiLength, UiNode, UiNodeId, UiRouter, UiSelection, UiTheme, UiTree,
};
use gridthorn::{CursorPosition, InputEvent, TextInputEvent, TextStyle};

/// Public asset-font probe of overlapping layers, injected preedit and commit paint.
#[test]
#[ignore = "manual layered font/editing probe; run alone in release mode without diagnostics"]
fn measure_overlapping_font_editing() {
    assert!(!black_box(cfg!(debug_assertions)), "run with --release");
    for flag in ["GRIDTHORN_UI_PERFORMANCE", "GRIDTHORN_TEXT_PERFORMANCE"] {
        assert_eq!(std::env::var_os(flag), None);
    }
    println!("font_editing,script,dpi,modal,operation,sample,elapsed_ns");
    for (script, value) in [
        ("en-US", "Review text"),
        ("ru", "Проверка текста"),
        ("ar-EG", "مراجعة النص"),
        ("ja", "文章を確認する"),
    ] {
        for dpi in [1_u16, 2] {
            for modal in [false, true] {
                for operation in ["pointer_32", "preedit_commit_paint"] {
                    measure(script, value, dpi, modal, operation);
                }
            }
        }
    }
}

/// Measure long-field composition and replacement with warm repeated text.
#[test]
#[ignore = "manual long-field probe; run alone in release mode without diagnostics"]
fn measure_long_field_editing() {
    assert!(!black_box(cfg!(debug_assertions)), "run with --release");
    for flag in ["GRIDTHORN_UI_PERFORMANCE", "GRIDTHORN_TEXT_PERFORMANCE"] {
        assert_eq!(std::env::var_os(flag), None);
    }
    long_field_cases();
}

/// Attribute long-field cycles using bounded UI and text phase diagnostics.
#[test]
#[ignore = "manual attribution probe; enable UI/text diagnostics and run alone in release"]
fn measure_long_field_phases() {
    assert!(!black_box(cfg!(debug_assertions)), "run with --release");
    for flag in ["GRIDTHORN_UI_PERFORMANCE", "GRIDTHORN_TEXT_PERFORMANCE"] {
        assert!(std::env::var_os(flag).is_some());
    }
    long_field_cases();
}

fn long_field_cases() {
    println!("font_editing,script,dpi,modal,operation,sample,elapsed_ns");
    for (script, phrase) in [
        ("en-US", "Review text "),
        ("ru", "Проверка текста "),
        ("ar-EG", "مراجعة النص "),
        ("ja", "文章を確認する "),
    ] {
        for repetitions in [8, 64, 256] {
            let value = phrase.repeat(repetitions);
            let label = format!("{script}-x{repetitions}");
            println!(
                "long_field_input,{label},utf8_bytes={},scalars={}",
                value.len(),
                value.chars().count()
            );
            for dpi in [1, 2] {
                let mut fonts = super::super::composition::fonts().unwrap();
                match authored_tree(&value).layout(
                    [1000.0, 800.0],
                    f32::from(dpi),
                    Some(&mut fonts),
                ) {
                    Ok(_) => {}
                    Err(gridthorn::ui::UiCompositionError::Text(
                        gridthorn::TextError::TooLarge,
                    )) => {
                        println!("long_field_rejected,{label},{dpi},layout_text_too_large");
                        continue;
                    }
                    Err(error) => panic!("{label}/DPI{dpi}: {error}"),
                }
                measure(&label, &value, dpi, true, "preedit_commit_paint");
            }
        }
    }
}

fn measure(script: &str, value: &str, dpi: u16, modal: bool, operation: &str) {
    let mut fonts = super::super::composition::fonts().unwrap();
    let mut tree = authored_tree(value);
    let scale = f32::from(dpi);
    let raw = tree
        .layout([1000.0, 800.0], scale, Some(&mut fonts))
        .unwrap();
    let mut router = UiRouter::new(0);
    for id in [10, 20, 30] {
        router
            .open_layer(
                &mut tree,
                &raw,
                UiNodeId(id),
                UiLayer {
                    modal: modal && id == 30,
                    ..UiLayer::default()
                },
            )
            .unwrap();
    }
    let mut layout = router
        .layout(&tree, [1000.0, 800.0], scale, Some(&mut fonts))
        .unwrap();
    router
        .route_events(
            &mut tree,
            &layout,
            &[InputEvent::TextInputChanged {
                active: false,
                error: None,
            }],
        )
        .unwrap();
    let pointers = pointer_events(&layout, dpi);
    let commits = [format!("{value} 0"), format!("{value} 1")];
    let preedit = [InputEvent::Text(TextInputEvent::Composition {
        text: value.into(),
        cursor: None,
    })];
    let commit_events = commits
        .each_ref()
        .map(|text| [InputEvent::Text(TextInputEvent::Commit(text.clone()))]);
    for sample in 0..110 {
        let start = Instant::now();
        if operation == "pointer_32" {
            let route = router
                .route_events(&mut tree, &layout, black_box(&pointers))
                .unwrap();
            black_box(route);
        } else {
            let UiControl::TextField { value: current, .. } =
                &tree.node(UiNodeId(31)).unwrap().control
            else {
                panic!("field")
            };
            router
                .select(
                    &tree,
                    UiSelection {
                        anchor: 0,
                        caret: current.len(),
                    },
                )
                .unwrap();
            black_box(router.route_events(&mut tree, &layout, &preedit).unwrap());
            let composing = router
                .layout(&tree, [1000.0, 800.0], scale, Some(&mut fonts))
                .unwrap();
            if sample == 0 {
                assert_eq!(router.preedit(), value);
                assert_ne!(composing.primitives(), layout.primitives());
            }
            black_box(
                router
                    .route_events(&mut tree, &composing, &commit_events[sample % 2])
                    .unwrap(),
            );
            layout = router
                .layout(&tree, [1000.0, 800.0], scale, Some(&mut fonts))
                .unwrap();
            black_box(&layout);
        }
        let elapsed = start.elapsed().as_nanos();
        if sample >= 10 {
            println!(
                "font_editing,{script},{dpi},{modal},{operation},{},{elapsed}",
                sample - 10
            );
        }
    }
    validate(
        &mut tree,
        &mut router,
        &layout,
        &pointers,
        &commits[1],
        modal,
        operation,
    );
}

fn authored_tree(value: &str) -> UiTree {
    let mut root = UiNode::new(UiNodeId(0), UiControl::Panel);
    root.style.size = [UiLength::Fill; 2];
    for index in 1_u16..=3 {
        let id = u64::from(index) * 10;
        let mut panel = UiNode::new(UiNodeId(id), UiControl::Panel);
        panel.style.size = [UiLength::Pixels(600.0), UiLength::Pixels(300.0)];
        panel.style.offset = [f32::from(index) * 60.0, f32::from(index) * 30.0];
        panel.style.clip = true;
        let mut field = UiNode::new(
            UiNodeId(id + 1),
            UiControl::TextField {
                value: value.into(),
                placeholder: String::new(),
            },
        );
        field.style.size = [UiLength::Fill, UiLength::Pixels(120.0)];
        panel.children.push(field);
        root.children.push(panel);
    }
    UiTree::new(
        root,
        UiTheme {
            text: Some(TextStyle::new("Noto Sans", 20.0)),
            ..UiTheme::default()
        },
    )
    .unwrap()
}

fn validate(
    tree: &mut UiTree,
    router: &mut UiRouter,
    layout: &gridthorn::ui::UiLayout,
    pointers: &[InputEvent],
    expected: &str,
    modal: bool,
    operation: &str,
) {
    assert_eq!(
        router.open_layers(),
        [UiNodeId(10), UiNodeId(20), UiNodeId(30)]
    );
    assert_eq!(router.focused(), Some(UiNodeId(31)));
    if operation == "pointer_32" {
        let route = router.route_events(tree, layout, pointers).unwrap();
        assert_eq!(route.world_events, []);
        assert_eq!(route.consumed, (0..32).collect::<Vec<_>>());
        assert_eq!(route.effects, []);
        let point = layout
            .placement(UiNodeId(11))
            .unwrap()
            .bounds
            .position
            .map(|v| v + 5.0);
        assert_eq!(
            router.hit_test_layers(tree, layout, point),
            if modal { None } else { Some(UiNodeId(11)) }
        );
    } else {
        let UiControl::TextField { value, .. } = &tree.node(UiNodeId(31)).unwrap().control else {
            panic!("field")
        };
        assert_eq!(value, expected);
        assert_eq!(router.preedit(), "");
        assert_ne!(layout.primitives(), []);
    }
}

fn pointer_events(layout: &gridthorn::ui::UiLayout, dpi: u16) -> Vec<InputEvent> {
    (0..32)
        .map(|index| {
            let id = [11, 21, 31][index % 3];
            let bounds = layout.placement(UiNodeId(id)).unwrap().bounds;
            InputEvent::CursorMoved(CursorPosition {
                x: f64::from(bounds.position[0] + 5.0) * f64::from(dpi),
                y: f64::from(bounds.position[1] + 5.0) * f64::from(dpi),
            })
        })
        .collect()
}
