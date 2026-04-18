//! Phase 2: ZeltoNode IR → Rust source code.
//! The generated Rust code uses zelto-std builder calls, then gets compiled by rustc.

use anyhow::Result;
use zelto_ir::ZeltoNode;

pub struct RustCodegen;

impl RustCodegen {
    pub fn new() -> Self { Self }

    /// Emit a Rust source file from a ZeltoNode tree.
    pub fn emit(&self, _root: &ZeltoNode) -> Result<String> {
        anyhow::bail!("Rust codegen not yet implemented — coming in Phase 2")
    }
}

impl Default for RustCodegen {
    fn default() -> Self { Self::new() }
}
