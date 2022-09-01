mod list;

use core::sync::atomic::{AtomicU64, Ordering::{Relaxed, SeqCst}};
use spin::{Lazy, Mutex};

use self::list::ContextList;

static CONTEXT: Lazy<Mutex<ContextList>> = Lazy::new(|| {
    Mutex::new(ContextList::new())
});

// current not work
// #[thread_local]
pub static CURRENT_CONTEXT_ID: Lazy<AtomicU64> = Lazy::new(|| {
   AtomicU64::default()
});

// lazy_static! {
//     #[thread_local]
//     pub static ref CURRENT_CONTEXT_ID: AtomicU64 = AtomicU64::default();
// }


pub fn init() {
    // init first context
    let id = CONTEXT.lock()
        .new_context()
        .expect("[Error] unable to initialize first context")
        .lock()
        .id;
    CURRENT_CONTEXT_ID.store(id.0, SeqCst);
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ContextId(u64);

impl ContextId {
    pub(crate) fn new() -> Self {
        static CONTEXT_ID: AtomicU64 = AtomicU64::new(0);
        Self(CONTEXT_ID.fetch_add(1, Relaxed))
    }

    pub(crate) fn from(id: u64) -> Self {
        ContextId(id)
    }
}

impl From<ContextId> for usize {
    fn from(ctx_id: ContextId) -> Self {
        usize::from(ctx_id)
    }
}

impl From<ContextId> for u64 {
    fn from(ctx_id: ContextId) -> Self {
        ctx_id.0
    }
}

impl From<ContextId> for i32 {
    fn from(ctx_id: ContextId) -> Self {
        ctx_id.0 as i32
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