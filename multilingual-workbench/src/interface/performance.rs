use super::runtime::Interface;
use gridthorn::WindowViewport;
use gridthorn::ui::{UiCommand, UiEffect, UiNodeId};
use std::time::Instant;

const SAMPLES: usize = 100;

#[cfg(test)]
#[path = "test/layered_editing.rs"]
mod layered_editing;

#[cfg(test)]
#[path = "test/paint_clip.rs"]
mod paint_clip;

#[cfg(test)]
#[path = "test/expanded_fonts.rs"]
mod expanded_fonts;

/// Measure public CPU work separately from native rendering and presentation.
pub(crate) fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "CPU-only workbench; 100 warm samples; 1000x800 logical pixels; GPU/present/allocations unavailable"
    );
    println!("locale,dpi,operation,median_us,p95_us,p99_us,max_us,primitives");
    for dpi in [1_u16, 2] {
        for (index, locale) in ["en-US", "ru", "ar-EG", "ja"].into_iter().enumerate() {
            let started = Instant::now();
            let mut state = Interface::new()?;
            report(
                locale,
                dpi,
                "construction",
                vec![started.elapsed().as_micros()],
                0,
            );
            state
                .tree
                .command(UiNodeId(6), UiCommand::Select(Some(index)))?;
            state.effect(UiNodeId(6), UiEffect::Changed)?;
            let viewport = WindowViewport {
                width: 1000 * u32::from(dpi),
                height: 800 * u32::from(dpi),
            };
            let started = Instant::now();
            state.prepare(viewport, f64::from(dpi))?;
            report(
                locale,
                dpi,
                "first_prepare",
                vec![started.elapsed().as_micros()],
                state.layout.as_ref().unwrap().primitives().len(),
            );
            for operation in [
                "router_layout",
                "idle_prepare",
                "empty_route",
                "frame_clone",
                "edit_prepare",
            ] {
                let mut samples = Vec::with_capacity(SAMPLES);
                for sample in 0..SAMPLES + 10 {
                    if operation == "edit_prepare" {
                        state.tree.command(
                            UiNodeId(7),
                            UiCommand::SetText(format!("Привет مرحبًا 日本語 e\u{301} {sample}")),
                        )?;
                    }
                    let started = Instant::now();
                    match operation {
                        "router_layout" => {
                            std::hint::black_box(state.router.layout(
                                &state.tree,
                                [1000.0, 800.0],
                                f32::from(dpi),
                                Some(&mut state.fonts),
                            )?);
                        }
                        "empty_route" => {
                            std::hint::black_box(state.router.route_events(
                                &mut state.tree,
                                state.layout.as_ref().unwrap(),
                                &[],
                            )?);
                        }
                        "frame_clone" => {
                            std::hint::black_box(
                                gridthorn::RenderFrame::default()
                                    .with_ui(state.layout.as_ref().unwrap().primitives().to_vec()),
                            );
                        }
                        _ => {
                            std::hint::black_box(state.prepare(viewport, f64::from(dpi))?);
                        }
                    }
                    if sample >= 10 {
                        samples.push(started.elapsed().as_micros());
                    }
                }
                report(
                    locale,
                    dpi,
                    operation,
                    samples,
                    state.layout.as_ref().unwrap().primitives().len(),
                );
            }
        }
    }
    Ok(())
}

fn report(locale: &str, dpi: u16, operation: &str, mut samples: Vec<u128>, primitives: usize) {
    samples.sort_unstable();
    let percentile = |percent: usize| samples[(samples.len() * percent).div_ceil(100) - 1];
    println!(
        "{locale},{dpi},{operation},{},{},{},{},{primitives}",
        percentile(50),
        percentile(95),
        percentile(99),
        samples.last().unwrap()
    );
}
