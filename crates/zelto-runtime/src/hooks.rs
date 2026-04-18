use std::any::Any;
use std::sync::{Arc, Mutex};

/// Per-component state storage. Each component instance owns one HookStore.
pub struct HookStore {
    state: Vec<Box<dyn Any + Send>>,
    dirty: bool,
    current_slot: usize,
}

impl HookStore {
    pub fn new() -> Self {
        Self {
            state: Vec::new(),
            dirty: false,
            current_slot: 0,
        }
    }

    /// Call at the start of each component render to reset slot cursor.
    pub fn begin_render(&mut self) {
        self.current_slot = 0;
        self.dirty = false;
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// useState — returns (current_value_clone, setter).
    /// On first call: initializes with `init`. On subsequent calls: returns stored value.
    pub fn use_state<T: Any + Send + Clone + 'static>(
        &mut self,
        init: T,
    ) -> StateHandle<T> {
        let slot = self.current_slot;
        self.current_slot += 1;

        if slot >= self.state.len() {
            self.state.push(Box::new(init));
        }

        let value = self.state[slot]
            .downcast_ref::<T>()
            .expect("useState type mismatch")
            .clone();

        StateHandle { slot, value }
    }

    /// Update a state slot. Returns true if value changed.
    pub fn set_state<T: Any + Send + Clone + PartialEq + 'static>(
        &mut self,
        slot: usize,
        new_value: T,
    ) -> bool {
        if let Some(boxed) = self.state.get_mut(slot) {
            if let Some(current) = boxed.downcast_ref::<T>() {
                if *current == new_value {
                    return false;
                }
            }
            *boxed = Box::new(new_value);
            self.dirty = true;
            true
        } else {
            false
        }
    }

    /// Read raw state slot for effect dependency checks.
    pub fn read_slot<T: Any + Clone + 'static>(&self, slot: usize) -> Option<T> {
        self.state.get(slot)?.downcast_ref::<T>().cloned()
    }
}

impl Default for HookStore {
    fn default() -> Self {
        Self::new()
    }
}

/// Returned from `use_state`. Caller uses `.value` to read, `.slot` to set.
#[derive(Debug, Clone)]
pub struct StateHandle<T: Clone> {
    pub slot: usize,
    pub value: T,
}

/// Effect registration — tracks deps to decide re-run.
pub struct Effect {
    pub deps: Vec<Box<dyn Any + Send>>,
    pub cleanup: Option<Box<dyn FnOnce() + Send>>,
}
