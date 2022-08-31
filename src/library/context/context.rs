use core::sync::atomic::{AtomicU64, Ordering::Relaxed};



#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ContextId(u64);

impl ContextId {
    pub(crate) fn new() -> Self {
        static CONTEXT_ID: AtomicU64 = AtomicU64::new(0);
        Self(CONTEXT_ID.fetch_add(1, Relaxed))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Runnable,
    Blocked,
    Stopped(usize),
    Exited(usize),
}

pub struct Context {
    pub id: ContextId,
    pub status: Status,
    pub running: bool
}

impl Context {
    pub(crate) fn new() -> Self {
        Self {
            id: ContextId::new(),
            status: Status::Runnable,
            running: false
        }
    }
}