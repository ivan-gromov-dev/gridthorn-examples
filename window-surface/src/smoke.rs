use std::time::{Duration, Instant};

use gridthorn_app::{WindowControl, WindowLifecycle};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SmokePhase {
    Resize,
    Minimize,
    Restore,
    Close,
    Complete,
}

/// Timed window operations used by the executable smoke path.
pub(crate) struct SmokeLifecycle {
    enabled: bool,
    phase: SmokePhase,
    deadline: Instant,
}

impl SmokeLifecycle {
    /// Create an enabled or inert smoke lifecycle.
    pub(crate) fn new(enabled: bool) -> Self {
        Self {
            enabled,
            phase: SmokePhase::Resize,
            deadline: Instant::now(),
        }
    }

    fn schedule(&mut self, control: &mut WindowControl, phase: SmokePhase) {
        self.phase = phase;
        self.deadline = Instant::now() + Duration::from_millis(300);
        control.wake_at(self.deadline);
    }
}

impl WindowLifecycle for SmokeLifecycle {
    fn started(&mut self, control: &mut WindowControl) {
        if self.enabled {
            self.deadline = Instant::now() + Duration::from_millis(200);
            control.wake_at(self.deadline);
        }
    }

    fn idle(&mut self, control: &mut WindowControl) {
        if !self.enabled || self.phase == SmokePhase::Complete || Instant::now() < self.deadline {
            return;
        }

        match self.phase {
            SmokePhase::Resize => {
                control.set_size(800, 600);
                self.schedule(control, SmokePhase::Minimize);
            }
            SmokePhase::Minimize => {
                control.set_minimized(true);
                self.schedule(control, SmokePhase::Restore);
            }
            SmokePhase::Restore => {
                control.set_minimized(false);
                control.set_size(640, 480);
                self.schedule(control, SmokePhase::Close);
            }
            SmokePhase::Close => {
                self.phase = SmokePhase::Complete;
                control.exit();
            }
            SmokePhase::Complete => {}
        }
    }
}

#[cfg(test)]
mod test;
