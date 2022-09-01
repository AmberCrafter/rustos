use core::sync::atomic::Ordering::SeqCst;
use crate::library::context::{ContextId, CURRENT_CONTEXT_ID};

use super::error::Result;

pub fn getpid() -> Result<ContextId> {
    Ok(ContextId::from(CURRENT_CONTEXT_ID.load(SeqCst)))
}