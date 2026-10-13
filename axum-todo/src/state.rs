use std::{
    collections::HashMap,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
};

use tokio::sync::RwLock;

use crate::models::Todo;

#[derive(Clone)]
pub struct AppState {
    pub(crate) todos: Arc<RwLock<HashMap<u64, Todo>>>,
    next_id: Arc<AtomicU64>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            todos: Arc::new(RwLock::new(HashMap::new())),
            next_id: Arc::new(AtomicU64::new(1)),
        }
    }
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn allocate_id(&self) -> Option<u64> {
        self.next_id
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |current| {
                current.checked_add(1)
            })
            .ok()
    }
}
