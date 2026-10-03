//! Shared-atlas extraction, animation, development reload, and screen-space UI.
mod assets;
mod atlas;
mod interface;
mod runtime;
mod skin;
mod world;

pub(crate) use assets::Assets;
use atlas::{region, sprite};
pub use runtime::register;
