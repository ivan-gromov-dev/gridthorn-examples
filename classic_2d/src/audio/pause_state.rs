use std::sync::atomic::{AtomicU8, Ordering};

/// Coalesces pause state before requesting a wake, so a full queue cannot lose it.
/// A failed wake on a full queue leaves an existing request to trigger consumption.
#[derive(Default)]
pub(super) struct PauseState(AtomicU8);

impl PauseState {
    pub(super) fn publish(&self, paused: bool) {
        self.0.store(if paused { 2 } else { 1 }, Ordering::Release);
    }

    pub(super) fn take(&self) -> Option<bool> {
        match self.0.swap(0, Ordering::AcqRel) {
            0 => None,
            1 => Some(false),
            2 => Some(true),
            _ => unreachable!("pause states are encoded by publish"),
        }
    }
}
