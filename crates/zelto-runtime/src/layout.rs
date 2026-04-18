use anyhow::Result;
use taffy::prelude::*;
use zelto_ir::node::ZeltoNode;
use zelto_ir::style::{Dimension, FlexDirection as ZFlexDir, AlignItems as ZAlign, JustifyContent as ZJustify};

/// Computed absolute pixel layout for a node.
#[derive(Debug, Clone, Copy)]
pub struct ComputedLayout {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

pub struct LayoutEngine {
    taffy: TaffyTree<()>,
}

impl LayoutEngine {
    pub fn new() -> Self {
        Self { taffy: TaffyTree::new() }
    }

    /// Compute layout for the entire tree. Returns flattened Vec in DFS order matching the tree.
    pub fn compute(&mut self, root: &ZeltoNode, available: (f32, f32)) -> Result<Vec<ComputedLayout>> {
        self.taffy = TaffyTree::new();
        let root_node = self.build_taffy(root)?;

        self.taffy.compute_layout(
            root_node,
            Size {
                width: AvailableSpace::Definite(available.0),
                height: AvailableSpace::Definite(available.1),
            },
        )?;

        let mut results = Vec::new();
        self.collect_layouts(root_node, 0.0, 0.0, &mut results)?;
        Ok(results)
    }

    fn build_taffy(&mut self, node: &ZeltoNode) -> Result<NodeId> {
        let style = self.node_to_taffy_style(node);
        let children: Vec<NodeId> = node.children()
            .iter()
            .map(|c| self.build_taffy(c))
            .collect::<Result<_>>()?;

        let taffy_node = if children.is_empty() {
            self.taffy.new_leaf(style)?
        } else {
            self.taffy.new_with_children(style, &children)?
        };

        Ok(taffy_node)
    }

    fn node_to_taffy_style(&self, node: &ZeltoNode) -> Style {
        let mut style = Style::default();
        style.display = Display::Flex;

        match node {
            ZeltoNode::View { layout, .. } => {
                if let Some(fd) = layout.flex_direction {
                    style.flex_direction = match fd {
                        ZFlexDir::Row => FlexDirection::Row,
                        ZFlexDir::Column => FlexDirection::Column,
                        ZFlexDir::RowReverse => FlexDirection::RowReverse,
                        ZFlexDir::ColumnReverse => FlexDirection::ColumnReverse,
                    };
                }
                if let Some(ai) = layout.align_items {
                    style.align_items = Some(match ai {
                        ZAlign::Start => AlignItems::Start,
                        ZAlign::Center => AlignItems::Center,
                        ZAlign::End => AlignItems::End,
                        ZAlign::Stretch => AlignItems::Stretch,
                        ZAlign::Baseline => AlignItems::Baseline,
                    });
                }
                if let Some(jc) = layout.justify_content {
                    style.justify_content = Some(match jc {
                        ZJustify::Start => JustifyContent::Start,
                        ZJustify::Center => JustifyContent::Center,
                        ZJustify::End => JustifyContent::End,
                        ZJustify::SpaceBetween => JustifyContent::SpaceBetween,
                        ZJustify::SpaceAround => JustifyContent::SpaceAround,
                        ZJustify::SpaceEvenly => JustifyContent::SpaceEvenly,
                    });
                }
                if let Some(gap) = layout.gap {
                    style.gap = Size { width: length(gap), height: length(gap) };
                }
                if let Some(w) = layout.width {
                    style.size.width = dim_to_taffy(w);
                }
                if let Some(h) = layout.height {
                    style.size.height = dim_to_taffy(h);
                }
                if let Some(p) = layout.padding {
                    style.padding = Rect {
                        left: length(p.left),
                        right: length(p.right),
                        top: length(p.top),
                        bottom: length(p.bottom),
                    };
                }
                if let Some(fg) = layout.flex_grow {
                    style.flex_grow = fg;
                }
            }
            ZeltoNode::Button { style: s, .. } => {
                style.size = Size {
                    width: s.width.map(dim_to_taffy).unwrap_or(auto()),
                    height: s.height.map(dim_to_taffy).unwrap_or(length(32.0)),
                };
                style.min_size = Size { width: length(80.0), height: length(28.0) };
            }
            ZeltoNode::Text { style: s, .. } => {
                style.size = Size {
                    width: s.width.map(dim_to_taffy).unwrap_or(auto()),
                    height: s.height.map(dim_to_taffy).unwrap_or(length(24.0)),
                };
            }
            ZeltoNode::TextInput { style: s, .. } => {
                style.size = Size {
                    width: s.width.map(dim_to_taffy).unwrap_or(auto()),
                    height: s.height.map(dim_to_taffy).unwrap_or(length(28.0)),
                };
                style.flex_grow = s.flex_grow.unwrap_or(0.0);
            }
            ZeltoNode::Window { width, height, .. } => {
                style.size = Size {
                    width: length(*width as f32),
                    height: length(*height as f32),
                };
                style.flex_direction = FlexDirection::Column;
            }
            _ => {}
        }

        style
    }

    fn collect_layouts(
        &self,
        node: NodeId,
        parent_x: f32,
        parent_y: f32,
        out: &mut Vec<ComputedLayout>,
    ) -> Result<()> {
        let layout = self.taffy.layout(node)?;
        let abs_x = parent_x + layout.location.x;
        let abs_y = parent_y + layout.location.y;

        out.push(ComputedLayout {
            x: abs_x,
            y: abs_y,
            width: layout.size.width,
            height: layout.size.height,
        });

        for child in self.taffy.children(node)? {
            self.collect_layouts(child, abs_x, abs_y, out)?;
        }

        Ok(())
    }
}

impl Default for LayoutEngine {
    fn default() -> Self {
        Self::new()
    }
}

fn dim_to_taffy(d: Dimension) -> taffy::style::Dimension {
    match d {
        Dimension::Px(v) => length(v),
        Dimension::Percent(v) => percent(v / 100.0),
        Dimension::Auto => auto(),
    }
}
