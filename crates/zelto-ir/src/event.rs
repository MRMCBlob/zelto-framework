/// Unique ID for event handler registrations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EventId(pub u64);

#[derive(Debug, Clone, PartialEq)]
pub enum EventKind {
    Click,
    Change(String),
    KeyPress(u32),
    Focus,
    Blur,
    Resize { width: u32, height: u32 },
    Close,
}
