mod cache;
mod composition;
mod editor_workload;
mod interaction;
mod messages;
mod native_performance;
mod performance;
mod runtime;
mod scrolling;
mod validation;
mod window_workload;
mod windows;
mod workload;
pub(crate) use workload::SmokeWorkload;

pub(crate) use performance::run as performance;
pub(crate) use runtime::{headless, runtime};
