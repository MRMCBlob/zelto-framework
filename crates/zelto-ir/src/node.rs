use crate::style::Style;
use crate::event::EventId;
use crate::layout::LayoutProps;

/// The core IR node type. Every UI element in Zelto maps to one of these.
#[derive(Debug, Clone, PartialEq)]
pub enum ZeltoNode {
    /// Top-level application window.
    Window {
        title: String,
        width: u32,
        height: u32,
        resizable: bool,
        children: Vec<ZeltoNode>,
    },
    /// Flex container — maps to a grouping HWND child or a layout region.
    View {
        layout: LayoutProps,
        style: Style,
        children: Vec<ZeltoNode>,
    },
    /// Static text label.
    Text {
        content: String,
        style: Style,
    },
    /// Native Win32 button.
    Button {
        label: String,
        on_click: Option<EventId>,
        disabled: bool,
        style: Style,
    },
    /// Native Win32 single-line text input.
    TextInput {
        value: String,
        placeholder: String,
        on_change: Option<EventId>,
        on_submit: Option<EventId>,
        disabled: bool,
        style: Style,
    },
    /// Direct2D custom canvas — draw_fn called each paint cycle.
    Canvas {
        width: u32,
        height: u32,
        draw_fn: Option<EventId>,
        style: Style,
    },
    /// Checkbox control.
    Checkbox {
        checked: bool,
        label: String,
        on_change: Option<EventId>,
        style: Style,
    },
    /// Transparent fragment — multiple children without a wrapper node.
    Fragment(Vec<ZeltoNode>),
    /// User-defined component instance (resolved during reconciliation).
    Component {
        name: String,
        key: Option<String>,
        state_slot_count: usize,
        children: Vec<ZeltoNode>,
    },
}

impl ZeltoNode {
    pub fn type_name(&self) -> &'static str {
        match self {
            Self::Window { .. } => "Window",
            Self::View { .. } => "View",
            Self::Text { .. } => "Text",
            Self::Button { .. } => "Button",
            Self::TextInput { .. } => "TextInput",
            Self::Canvas { .. } => "Canvas",
            Self::Checkbox { .. } => "Checkbox",
            Self::Fragment(_) => "Fragment",
            Self::Component { .. } => "Component",
        }
    }

    pub fn children(&self) -> &[ZeltoNode] {
        match self {
            Self::Window { children, .. } => children,
            Self::View { children, .. } => children,
            Self::Fragment(children) => children,
            Self::Component { children, .. } => children,
            _ => &[],
        }
    }

    pub fn children_mut(&mut self) -> &mut Vec<ZeltoNode> {
        match self {
            Self::Window { children, .. } => children,
            Self::View { children, .. } => children,
            Self::Fragment(children) => children,
            Self::Component { children, .. } => children,
            _ => panic!("Node type {} has no children", self.type_name()),
        }
    }
}
