//! Phase 2: TSX AST → ZeltoNode IR.
//! Currently a stub — will use swc_core visitor pattern.

use anyhow::Result;
use zelto_ir::ZeltoNode;

pub struct IrBuilder;

impl IrBuilder {
    pub fn new() -> Self { Self }

    /// Parse TSX source and produce a ZeltoNode tree.
    /// Phase 2 implementation will use swc_core here.
    pub fn build_from_source(&self, _source: &str) -> Result<ZeltoNode> {
        anyhow::bail!("TSX compiler not yet implemented — coming in Phase 2")
    }
}

impl Default for IrBuilder {
    fn default() -> Self { Self::new() }
}
