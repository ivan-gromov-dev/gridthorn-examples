use gridthorn_app::{ApplicationError, WindowControl, WindowLifecycle};
use tracing::info;

const UPDATE_LIMIT: u32 = 3;

/// Short-lived lifecycle used to expose cross-layer diagnostic events.
#[derive(Default)]
pub(crate) struct DiagnosticLifecycle {
    updates: u32,
}

impl WindowLifecycle for DiagnosticLifecycle {
    fn started(&mut self, _control: &mut WindowControl) -> Result<(), ApplicationError> {
        info!(
            component = "example",
            event = "started",
            "diagnostics example lifecycle started"
        );
        Ok(())
    }

    fn idle(&mut self, control: &mut WindowControl) -> Result<(), ApplicationError> {
        if self.updates >= UPDATE_LIMIT {
            return Ok(());
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
        Ok(())
    }
}

#[cfg(test)]
mod test;
