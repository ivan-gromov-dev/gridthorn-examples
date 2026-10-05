use std::{hint::black_box, time::Instant};

use gridthorn::TextStyle;
use gridthorn::ui::{
    UiControl, UiFlow, UiLayer, UiLength, UiNode, UiNodeId, UiRouter, UiTheme, UiTree,
};

/// Public asset-font layout and paint under expanded overlapping layer pressure.
#[test]
#[ignore = "manual expanded-font probe; run alone in release without diagnostics"]
fn measure_expanded_font_layers() {
    assert!(!black_box(cfg!(debug_assertions)), "run with --release");
    for flag in ["GRIDTHORN_UI_PERFORMANCE", "GRIDTHORN_TEXT_PERFORMANCE"] {
        assert_eq!(std::env::var_os(flag), None);
    }
    println!("expanded_fonts,script,dpi,layers,fields_per_layer,sample,elapsed_ns,primitives");
    for (script, value) in [
        ("en-US", "Review text"),
        ("ru", "Проверка текста"),
        ("ar-EG", "مراجعة النص"),
        ("ja", "文章を確認する"),
    ] {
        for dpi in [1_u16, 2] {
            for layers in [3_u16, 16, 64] {
                for fields in [1_u16, 16] {
                    measure(script, value, dpi, layers, fields);
                }
            }
        }
    }
}

/// Separate arrangement, editing geometry, paint and text cache pressure.
#[test]
#[ignore = "manual phase probe; enable UI/text diagnostics and run alone in release"]
fn measure_expanded_font_phases() {
    assert!(!black_box(cfg!(debug_assertions)), "run with --release");
    for flag in ["GRIDTHORN_UI_PERFORMANCE", "GRIDTHORN_TEXT_PERFORMANCE"] {
        assert!(std::env::var_os(flag).is_some());
    }
    for (script, value) in [("en-US", "Review text"), ("ja", "文章を確認する")] {
        for dpi in [1, 2] {
            println!("expanded_phase_case,{script},{dpi}");
            measure(script, value, dpi, 64, 16);
        }
    }
}

/// Closed managed layers still participate in sizing; compare visible paint separately.
#[test]
#[ignore = "manual hidden-font sizing probe; run alone in release without diagnostics"]
fn measure_font_layer_visibility() {
    assert!(!black_box(cfg!(debug_assertions)), "run with --release");
    for flag in ["GRIDTHORN_UI_PERFORMANCE", "GRIDTHORN_TEXT_PERFORMANCE"] {
        assert_eq!(std::env::var_os(flag), None);
    }
    for (script, value) in [("en-US", "Review text"), ("ja", "文章を確認する")] {
        for dpi in [1_u16, 2] {
            let mut fonts = super::super::composition::fonts().unwrap();
            let mut tree = authored_tree(value, 64, 16);
            let initial = tree
                .layout([1000.0, 800.0], f32::from(dpi), Some(&mut fonts))
                .unwrap();
            let mut router = UiRouter::new(0);
            for layer in 1..=64 {
                router
                    .open_layer(
                        &mut tree,
                        &initial,
                        UiNodeId(layer * 100),
                        UiLayer::default(),
                    )
                    .unwrap();
            }
            for _ in 0..64 {
                black_box(router.close_layer(&tree, &initial));
            }
            for mode in ["closed", "one_open"] {
                if mode == "one_open" {
                    router
                        .open_layer(&mut tree, &initial, UiNodeId(6400), UiLayer::default())
                        .unwrap();
                }
                for sample in 0..30 {
                    let start = Instant::now();
                    let layout = router
                        .layout(&tree, [1000.0, 800.0], f32::from(dpi), Some(&mut fonts))
                        .unwrap();
                    let elapsed = start.elapsed().as_nanos();
                    assert_eq!(
                        layout.primitives().len(),
                        if mode == "closed" { 0 } else { 16 }
                    );
                    if sample >= 10 {
                        println!(
                            "font_visibility,{script},{dpi},{mode},{},{elapsed},{}",
                            sample - 10,
                            layout.primitives().len()
                        );
                    }
                }
            }
        }
    }
}

fn authored_tree(value: &str, layers: u16, fields: u16) -> UiTree {
    let mut root = UiNode::new(UiNodeId(0), UiControl::Panel);
    root.style.size = [UiLength::Fill; 2];
    for layer in 1..=layers {
        let base = u64::from(layer) * 100;
        let mut panel = UiNode::new(UiNodeId(base), UiControl::Panel);
        panel.style.size = [UiLength::Pixels(600.0), UiLength::Pixels(720.0)];
        panel.style.offset = [f32::from(layer), f32::from(layer)];
        panel.style.clip = true;
        panel.style.flow = UiFlow::Column;
        for field in 1..=fields {
            let mut node = UiNode::new(
                UiNodeId(base + u64::from(field)),
                UiControl::TextField {
                    value: format!("{value} {layer}-{field}"),
                    placeholder: String::new(),
                },
            );
            node.style.size = [UiLength::Fill, UiLength::Pixels(36.0)];
            panel.children.push(node);
        }
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

fn measure(script: &str, value: &str, dpi: u16, layers: u16, fields: u16) {
    let mut fonts = super::super::composition::fonts().unwrap();
    let mut tree = authored_tree(value, layers, fields);
    let scale = f32::from(dpi);
    let initial = tree
        .layout([1000.0, 800.0], scale, Some(&mut fonts))
        .unwrap();
    let mut router = UiRouter::new(0);
    for layer in 1..=layers {
        router
            .open_layer(
                &mut tree,
                &initial,
                UiNodeId(u64::from(layer) * 100),
                UiLayer {
                    modal: layer == layers,
                    ..UiLayer::default()
                },
            )
            .unwrap();
    }
    assert_eq!(router.open_layers().len(), usize::from(layers));
    for sample in 0..30 {
        let start = Instant::now();
        let layout = black_box(
            router
                .layout(&tree, [1000.0, 800.0], scale, Some(&mut fonts))
                .unwrap(),
        );
        let elapsed = start.elapsed().as_nanos();
        assert_eq!(
            router.focused(),
            Some(UiNodeId(u64::from(layers) * 100 + 1))
        );
        let top = layout
            .placement(UiNodeId(u64::from(layers) * 100 + 1))
            .unwrap();
        let point = top.bounds.position.map(|coordinate| coordinate + 10.0);
        assert_eq!(router.hit_test_layers(&tree, &layout, point), Some(top.id));
        assert_eq!(
            layout.primitives().len(),
            usize::from(layers) * usize::from(fields)
        );
        if sample >= 10 {
            println!(
                "expanded_fonts,{script},{dpi},{layers},{fields},{},{elapsed},{}",
                sample - 10,
                layout.primitives().len()
            );
        }
    }
}
