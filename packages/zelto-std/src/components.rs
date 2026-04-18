use zelto_ir::{
    ZeltoNode,
    Style,
    EventId,
    LayoutProps,
    style::{FlexDirection, AlignItems, JustifyContent, Edges, Dimension},
};

// ─── Window ──────────────────────────────────────────────────────────────────

pub struct WindowBuilder {
    title: String,
    width: u32,
    height: u32,
    resizable: bool,
    children: Vec<ZeltoNode>,
}

pub fn window(title: impl Into<String>) -> WindowBuilder {
    WindowBuilder {
        title: title.into(),
        width: 800,
        height: 600,
        resizable: true,
        children: Vec::new(),
    }
}

impl WindowBuilder {
    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }
    pub fn resizable(mut self, r: bool) -> Self { self.resizable = r; self }
    pub fn child(mut self, node: ZeltoNode) -> Self { self.children.push(node); self }
    pub fn children(mut self, nodes: Vec<ZeltoNode>) -> Self { self.children = nodes; self }
    pub fn build(self) -> ZeltoNode {
        ZeltoNode::Window {
            title: self.title,
            width: self.width,
            height: self.height,
            resizable: self.resizable,
            children: self.children,
        }
    }
}

// ─── View ─────────────────────────────────────────────────────────────────────

pub struct ViewBuilder {
    layout: LayoutProps,
    style: Style,
    children: Vec<ZeltoNode>,
}

pub fn view() -> ViewBuilder {
    ViewBuilder {
        layout: LayoutProps::default(),
        style: Style::default(),
        children: Vec::new(),
    }
}

impl ViewBuilder {
    pub fn column(mut self) -> Self {
        self.layout.flex_direction = Some(FlexDirection::Column);
        self
    }
    pub fn row(mut self) -> Self {
        self.layout.flex_direction = Some(FlexDirection::Row);
        self
    }
    pub fn padding(mut self, v: f32) -> Self {
        self.layout.padding = Some(Edges::all(v));
        self
    }
    pub fn gap(mut self, v: f32) -> Self {
        self.layout.gap = Some(v);
        self
    }
    pub fn align_center(mut self) -> Self {
        self.layout.align_items = Some(AlignItems::Center);
        self
    }
    pub fn justify_center(mut self) -> Self {
        self.layout.justify_content = Some(JustifyContent::Center);
        self
    }
    pub fn child(mut self, node: ZeltoNode) -> Self { self.children.push(node); self }
    pub fn children(mut self, nodes: Vec<ZeltoNode>) -> Self { self.children = nodes; self }
    pub fn build(self) -> ZeltoNode {
        ZeltoNode::View {
            layout: self.layout,
            style: self.style,
            children: self.children,
        }
    }
}

// ─── Text ─────────────────────────────────────────────────────────────────────

pub fn text(content: impl Into<String>) -> ZeltoNode {
    ZeltoNode::Text {
        content: content.into(),
        style: Style::default(),
    }
}

// ─── Button ───────────────────────────────────────────────────────────────────

pub struct ButtonBuilder {
    label: String,
    on_click: Option<EventId>,
    disabled: bool,
    style: Style,
}

pub fn button(label: impl Into<String>) -> ButtonBuilder {
    ButtonBuilder {
        label: label.into(),
        on_click: None,
        disabled: false,
        style: Style::default(),
    }
}

impl ButtonBuilder {
    pub fn on_click(mut self, id: EventId) -> Self { self.on_click = Some(id); self }
    pub fn disabled(mut self, d: bool) -> Self { self.disabled = d; self }
    pub fn build(self) -> ZeltoNode {
        ZeltoNode::Button {
            label: self.label,
            on_click: self.on_click,
            disabled: self.disabled,
            style: self.style,
        }
    }
}

// ─── TextInput ────────────────────────────────────────────────────────────────

pub struct TextInputBuilder {
    value: String,
    placeholder: String,
    on_change: Option<EventId>,
    on_submit: Option<EventId>,
    disabled: bool,
    style: Style,
}

pub fn text_input() -> TextInputBuilder {
    TextInputBuilder {
        value: String::new(),
        placeholder: String::new(),
        on_change: None,
        on_submit: None,
        disabled: false,
        style: Style::default(),
    }
}

impl TextInputBuilder {
    pub fn value(mut self, v: impl Into<String>) -> Self { self.value = v.into(); self }
    pub fn placeholder(mut self, p: impl Into<String>) -> Self { self.placeholder = p.into(); self }
    pub fn on_change(mut self, id: EventId) -> Self { self.on_change = Some(id); self }
    pub fn on_submit(mut self, id: EventId) -> Self { self.on_submit = Some(id); self }
    pub fn build(self) -> ZeltoNode {
        ZeltoNode::TextInput {
            value: self.value,
            placeholder: self.placeholder,
            on_change: self.on_change,
            on_submit: self.on_submit,
            disabled: self.disabled,
            style: self.style,
        }
    }
}

// ─── Checkbox ─────────────────────────────────────────────────────────────────

pub fn checkbox(label: impl Into<String>, checked: bool) -> ZeltoNode {
    ZeltoNode::Checkbox {
        label: label.into(),
        checked,
        on_change: None,
        style: Style::default(),
    }
}
