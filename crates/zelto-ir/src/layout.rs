use crate::style::{Dimension, Edges, FlexDirection, AlignItems, JustifyContent};

/// Layout constraints passed to taffy for flex computation.
#[derive(Debug, Clone, PartialEq)]
pub struct LayoutProps {
    pub width: Option<Dimension>,
    pub height: Option<Dimension>,
    pub padding: Option<Edges>,
    pub margin: Option<Edges>,
    pub flex_direction: Option<FlexDirection>,
    pub align_items: Option<AlignItems>,
    pub justify_content: Option<JustifyContent>,
    pub gap: Option<f32>,
    pub flex_grow: Option<f32>,
    pub flex_shrink: Option<f32>,
}

impl Default for LayoutProps {
    fn default() -> Self {
        Self {
            width: None,
            height: None,
            padding: None,
            margin: None,
            flex_direction: None,
            align_items: None,
            justify_content: None,
            gap: None,
            flex_grow: None,
            flex_shrink: None,
        }
    }
}
