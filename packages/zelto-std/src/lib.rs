//! Zelto standard component library.
//!
//! Provides builder helpers that produce `ZeltoNode` values, mimicking the
//! ergonomics of TSX component calls.  These are the Rust-API equivalents —
//! the TSX compiler will emit calls to this crate automatically.

pub mod components;
pub mod hooks;

pub use components::{window, view, text, button, text_input, checkbox};
pub use hooks::{use_state, use_effect};
