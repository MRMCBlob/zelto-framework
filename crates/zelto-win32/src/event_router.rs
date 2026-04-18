use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zelto_ir::event::{EventId, EventKind};
use windows::Win32::Foundation::HWND;

type EventCallback = Box<dyn Fn(EventKind) + Send + 'static>;

/// Routes Win32 control notifications to Zelto event handlers.
pub struct EventRouter {
    /// Maps control HWND to (EventId, callback)
    handlers: Arc<Mutex<HashMap<isize, Vec<(EventId, EventCallback)>>>>,
    next_id: u64,
}

impl EventRouter {
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(Mutex::new(HashMap::new())),
            next_id: 1,
        }
    }

    pub fn register(&mut self, hwnd: HWND, kind_filter: EventId, cb: EventCallback) {
        let mut map = self.handlers.lock().unwrap();
        map.entry(hwnd.0 as isize).or_default().push((kind_filter, cb));
    }

    pub fn dispatch(&self, hwnd: HWND, kind: EventKind) {
        let map = self.handlers.lock().unwrap();
        if let Some(handlers) = map.get(&(hwnd.0 as isize)) {
            for (_id, cb) in handlers {
                cb(kind.clone());
            }
        }
    }

    pub fn next_event_id(&mut self) -> EventId {
        let id = EventId(self.next_id);
        self.next_id += 1;
        id
    }

    pub fn unregister(&self, hwnd: HWND) {
        let mut map = self.handlers.lock().unwrap();
        map.remove(&(hwnd.0 as isize));
    }
}

impl Default for EventRouter {
    fn default() -> Self {
        Self::new()
    }
}
