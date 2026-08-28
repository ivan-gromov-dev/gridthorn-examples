use gridthorn_app::{WindowControl, WindowLifecycle};
use tracing::info;

const UPDATE_LIMIT: u32 = 3;

/// Short-lived lifecycle used to expose cross-layer diagnostic events.
#[derive(Default)]
pub(crate) struct DiagnosticLifecycle {
    updates: u32,
}

impl WindowLifecycle for DiagnosticLifecycle {
    fn started(&mut self, _control: &mut WindowControl) {
        info!(
            component = "example",
            event = "started",
            "diagnostics example lifecycle started"
        );
    }

    fn idle(&mut self, control: &mut WindowControl) {
        if self.updates >= UPDATE_LIMIT {
            return;
        }
        self.updates += 1;
        if self.updates >= UPDATE_LIMIT {
            info!(
                component = "example",
                event = "completed",
                updates = self.updates,
                "diagnostics example lifecycle completed"
            );
            control.exit();
        }
    }
}

#[cfg(test)]
mod test;
