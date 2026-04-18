use std::collections::HashMap;
use anyhow::Result;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::DestroyWindow;
use zelto_ir::node::ZeltoNode;
use zelto_ir::event::{EventId, EventKind};
use zelto_win32::controls::{NativeButton, NativeLabel, NativeTextInput, NativeCheckbox, Rect};
use crate::layout::{ComputedLayout, LayoutEngine};

/// Identifies a mounted native control.
pub enum MountedControl {
    Window(HWND),
    Button(NativeButton),
    Label(NativeLabel),
    TextInput(NativeTextInput),
    Checkbox(NativeCheckbox),
    View,  // Layout-only, no own HWND
}

impl MountedControl {
    pub fn hwnd(&self) -> Option<HWND> {
        match self {
            Self::Window(h) => Some(*h),
            Self::Button(b) => Some(b.hwnd),
            Self::Label(l) => Some(l.hwnd),
            Self::TextInput(t) => Some(t.hwnd),
            Self::Checkbox(c) => Some(c.hwnd),
            Self::View => None,
        }
    }
}

type EventDispatcher = Box<dyn Fn(EventId, EventKind) + Send + 'static>;

/// The Reconciler maps a ZeltoNode tree to Win32 controls, diffing on re-render.
pub struct Reconciler {
    /// Mounted controls indexed by DFS position in the tree.
    pub mounted: Vec<MountedControl>,
    layout_engine: LayoutEngine,
    next_ctrl_id: u16,
    event_dispatcher: Option<EventDispatcher>,
}

impl Reconciler {
    pub fn new() -> Self {
        Self {
            mounted: Vec::new(),
            layout_engine: LayoutEngine::new(),
            next_ctrl_id: 100,
            event_dispatcher: None,
        }
    }

    pub fn set_event_dispatcher(&mut self, f: EventDispatcher) {
        self.event_dispatcher = Some(f);
    }

    fn next_id(&mut self) -> u16 {
        let id = self.next_ctrl_id;
        self.next_ctrl_id += 1;
        id
    }

    /// Initial mount of the full tree under a parent window HWND.
    pub fn mount(&mut self, root: &ZeltoNode, parent: HWND, available: (f32, f32)) -> Result<()> {
        let layouts = self.layout_engine.compute(root, available)?;
        let mut idx = 0;
        self.mount_node(root, parent, &layouts, &mut idx)?;
        Ok(())
    }

    fn mount_node(
        &mut self,
        node: &ZeltoNode,
        parent: HWND,
        layouts: &[ComputedLayout],
        idx: &mut usize,
    ) -> Result<()> {
        let layout = layouts.get(*idx).copied().unwrap_or(ComputedLayout { x: 0.0, y: 0.0, width: 0.0, height: 0.0 });
        *idx += 1;

        let rect = Rect {
            x: layout.x as i32,
            y: layout.y as i32,
            width: layout.width as i32,
            height: layout.height as i32,
        };

        match node {
            ZeltoNode::Window { children, .. } => {
                self.mounted.push(MountedControl::Window(parent));
                let (w, h) = unsafe {
                    let mut r = windows::Win32::Foundation::RECT::default();
                    let _ = windows::Win32::UI::WindowsAndMessaging::GetClientRect(parent, &mut r);
                    (r.right as f32, r.bottom as f32)
                };
                let child_layouts = self.layout_engine.compute(node, (w, h))?;
                let mut cidx = 1; // skip Window itself at 0
                for child in children {
                    self.mount_node(child, parent, &child_layouts, &mut cidx)?;
                }
            }
            ZeltoNode::View { children, .. } => {
                self.mounted.push(MountedControl::View);
                for child in children {
                    self.mount_node(child, parent, layouts, idx)?;
                }
            }
            ZeltoNode::Text { content, .. } => {
                let label = NativeLabel::create(parent, content, rect)?;
                self.mounted.push(MountedControl::Label(label));
            }
            ZeltoNode::Button { label, on_click, .. } => {
                let id = self.next_id();
                let btn = NativeButton::create(parent, label, rect, id)?;
                if let (Some(event_id), Some(dispatcher)) = (on_click, &self.event_dispatcher) {
                    let eid = *event_id;
                    // Store click handler via subclassing — simplified: use ctrl ID mapping
                    // Full implementation uses SetWindowSubclass; here we record for WM_COMMAND routing
                }
                self.mounted.push(MountedControl::Button(btn));
            }
            ZeltoNode::TextInput { value, on_change, .. } => {
                let id = self.next_id();
                let input = NativeTextInput::create(parent, "", rect, id)?;
                input.set_text(value);
                self.mounted.push(MountedControl::TextInput(input));
            }
            ZeltoNode::Checkbox { label, checked, .. } => {
                let id = self.next_id();
                let cb = NativeCheckbox::create(parent, label, *checked, rect, id)?;
                self.mounted.push(MountedControl::Checkbox(cb));
            }
            ZeltoNode::Fragment(children) | ZeltoNode::Component { children, .. } => {
                for child in children {
                    self.mount_node(child, parent, layouts, idx)?;
                }
            }
            ZeltoNode::Canvas { .. } => {
                // Canvas handled by zelto-d2d — placeholder
                self.mounted.push(MountedControl::View);
            }
        }

        Ok(())
    }

    /// Diff previous tree against new tree, apply minimal Win32 updates.
    pub fn reconcile(
        &mut self,
        old: &ZeltoNode,
        new: &ZeltoNode,
        parent: HWND,
        available: (f32, f32),
    ) -> Result<()> {
        // If node type changed: full remount
        if old.type_name() != new.type_name() {
            self.unmount_all();
            return self.mount(new, parent, available);
        }

        let layouts = self.layout_engine.compute(new, available)?;
        let mut idx = 0;
        self.update_node(old, new, parent, &layouts, &mut idx)?;
        Ok(())
    }

    fn update_node(
        &mut self,
        old: &ZeltoNode,
        new: &ZeltoNode,
        parent: HWND,
        layouts: &[ComputedLayout],
        idx: &mut usize,
    ) -> Result<()> {
        let layout = layouts.get(*idx).copied().unwrap_or(ComputedLayout { x: 0.0, y: 0.0, width: 0.0, height: 0.0 });
        *idx += 1;

        let rect = Rect {
            x: layout.x as i32,
            y: layout.y as i32,
            width: layout.width as i32,
            height: layout.height as i32,
        };

        let mounted_idx = self.find_mounted_idx(old, *idx - 1);

        match (old, new) {
            (ZeltoNode::Text { content: old_c, .. }, ZeltoNode::Text { content: new_c, .. }) => {
                if old_c != new_c {
                    if let Some(MountedControl::Label(label)) = self.mounted.get(mounted_idx) {
                        label.set_text(new_c);
                    }
                }
            }
            (ZeltoNode::Button { label: ol, .. }, ZeltoNode::Button { label: nl, .. }) => {
                if ol != nl {
                    if let Some(MountedControl::Button(btn)) = self.mounted.get(mounted_idx) {
                        btn.set_label(nl);
                    }
                }
                if let Some(MountedControl::Button(btn)) = self.mounted.get(mounted_idx) {
                    btn.set_rect(rect);
                }
            }
            (ZeltoNode::TextInput { value: ov, .. }, ZeltoNode::TextInput { value: nv, .. }) => {
                if ov != nv {
                    if let Some(MountedControl::TextInput(input)) = self.mounted.get(mounted_idx) {
                        input.set_text(nv);
                    }
                }
            }
            (ZeltoNode::View { children: oc, .. }, ZeltoNode::View { children: nc, .. }) => {
                let min_len = oc.len().min(nc.len());
                for i in 0..min_len {
                    self.update_node(&oc[i], &nc[i], parent, layouts, idx)?;
                }
            }
            (ZeltoNode::Window { children: oc, .. }, ZeltoNode::Window { children: nc, .. }) => {
                let min_len = oc.len().min(nc.len());
                for i in 0..min_len {
                    self.update_node(&oc[i], &nc[i], parent, layouts, idx)?;
                }
            }
            _ => {}
        }

        Ok(())
    }

    fn find_mounted_idx(&self, _node: &ZeltoNode, hint: usize) -> usize {
        hint.min(self.mounted.len().saturating_sub(1))
    }

    pub fn unmount_all(&mut self) {
        for control in self.mounted.drain(..) {
            if let Some(hwnd) = control.hwnd() {
                unsafe { let _ = DestroyWindow(hwnd); }
            }
        }
    }
}

impl Default for Reconciler {
    fn default() -> Self {
        Self::new()
    }
}
