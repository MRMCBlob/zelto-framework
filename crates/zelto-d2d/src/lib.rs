//! Direct2D canvas renderer for Zelto.
//!
//! Phase 3 implementation. This module provides the `D2DCanvas` type which
//! wraps an `ID2D1HwndRenderTarget` and exposes a high-level draw API.

pub mod canvas;

pub use canvas::D2DCanvas;
