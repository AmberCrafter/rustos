use alloc::{collections::BTreeMap, rc::Rc};
use spin::Mutex;

use super::{ContextId, Context};

pub struct ContextList {
    map: BTreeMap<ContextId, Rc<Mutex<Context>>>
}

impl ContextList {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new()
        }
    }

    pub fn new_context(&mut self) -> Result<&Rc<Mutex<Context>>, ()> {
        let context = Context::new();
        let context_id = context.id;
        assert!(!self.map.contains_key(&context_id));
        assert!(self.map.insert(context_id, Rc::new(Mutex::new(context))).is_none());
        Ok(self.map.get(&context_id).expect("unable to insert context"))
    }
}