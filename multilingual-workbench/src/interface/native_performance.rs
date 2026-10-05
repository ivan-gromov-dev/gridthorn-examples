use std::time::{Duration, Instant};

const LIMIT: usize = 240;

#[derive(Clone, Copy)]
pub(super) enum Phase {
    PrepareBefore,
    InputRoute,
    Workload,
    Effects,
    PrepareAfter,
    Anchor,
    Snapshot,
    Layout,
}

/// Bounded input-system phase diagnostics; snapshot/layout are nested prepare costs.
pub(super) struct NativePerformance {
    enabled: bool,
    active: bool,
    current: [Duration; 8],
    samples: Vec<(u32, [Duration; 8])>,
}

impl NativePerformance {
    pub(super) fn new() -> Self {
        Self {
            enabled: std::env::var_os("GRIDTHORN_WORKBENCH_PERFORMANCE").is_some(),
            active: false,
            current: [Duration::ZERO; 8],
            samples: Vec::new(),
        }
    }

    pub(super) fn start(&self) -> Option<Instant> {
        self.active.then(Instant::now)
    }

    pub(super) fn begin(&mut self) {
        self.current = [Duration::ZERO; 8];
        self.active = self.enabled && self.samples.len() < LIMIT;
    }

    pub(super) fn record(&mut self, phase: Phase, start: Option<Instant>) {
        if let Some(start) = start {
            self.current[phase as usize] += start.elapsed();
        }
    }

    pub(super) fn finish(&mut self, frame: u32) {
        if self.active {
            self.samples.push((frame, self.current));
        }
        self.current = [Duration::ZERO; 8];
        self.active = false;
    }
}

impl Drop for NativePerformance {
    fn drop(&mut self) {
        if !self.enabled {
            return;
        }
        eprintln!(
            "workbench_cpu,frame,prepare_before_us,input_route_us,workload_us,effects_us,prepare_after_us,anchor_us,snapshot_us,layout_us"
        );
        for (frame, values) in &self.samples {
            let [
                before,
                input,
                workload,
                effects,
                after,
                anchor,
                snapshot,
                layout,
            ] = values.map(|value| value.as_micros());
            eprintln!(
                "workbench_cpu,{frame},{before},{input},{workload},{effects},{after},{anchor},{snapshot},{layout}"
            );
        }
    }
}

#[cfg(test)]
#[path = "test/native_performance.rs"]
mod test;
