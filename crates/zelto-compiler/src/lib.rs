//! Zelto TSX Compiler — Phase 2.
//!
//! Phase 1 stub. The compiler will:
//! 1. Parse `.zelto` files (TSX subset) using swc_core
//! 2. Build a Zelto IR tree (ZeltoNode)
//! 3. In dev-mode: emit IR for runtime interpretation + hot reload
//! 4. In prod-mode: emit Rust source code that calls zelto-std builders

pub mod ir_builder;
pub mod codegen;

pub use ir_builder::IrBuilder;
pub use codegen::RustCodegen;
