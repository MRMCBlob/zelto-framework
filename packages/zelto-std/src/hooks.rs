use zelto_runtime::hooks::{HookStore, StateHandle};

/// Re-export use_state with a more ergonomic signature.
/// In the full compiler, this will be emitted automatically from TSX `useState` calls.
pub fn use_state<T: std::any::Any + Send + Clone + 'static>(
    store: &mut HookStore,
    init: T,
) -> StateHandle<T> {
    store.use_state(init)
}

/// Simplified useEffect — full dep-tracking in Phase 3.
pub fn use_effect<F: FnOnce() -> Option<Box<dyn FnOnce() + Send>> + 'static>(
    _store: &mut HookStore,
    _f: F,
    _deps: &[u64],
) {
    // TODO Phase 3: track deps hash, run on change, call cleanup on unmount
}
