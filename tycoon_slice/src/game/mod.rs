//! Window lifecycle composition and interactive/headless integration checks.
mod controls;
mod input;
pub(crate) mod inspection;
mod runtime;
pub(crate) mod session;
mod smoke;
mod view_bookmark;

pub use runtime::runtime;
pub use smoke::smoke;

#[cfg(test)]
mod test;
