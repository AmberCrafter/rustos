use alloc::{collections::BTreeMap, sync::Arc};
use spin::Mutex;

use super::context::{ContextId, Context};

pub struct ContextList {
    map: BTreeMap<ContextId, Arc<Mutex<Context>>>
}

impl ContextList {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new()
        }
    }

    pub fn new_context(&mut self) -> Result<&Arc<Mutex<Context>>, ()> {
        let context = Context::new();
        let context_id = context.id;
        assert!(!self.map.contains_key(&context_id));
        assert!(self.map.insert(context_id, Arc::new(Mutex::new(context))).is_none());

        Ok(self.map.get(&context_id).expect("unable to insert context"))
    }
}