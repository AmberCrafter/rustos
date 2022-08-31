mod context;
mod list;

use spin::{Lazy, Mutex};

use self::list::ContextList;


static CONTEXT: Lazy<Mutex<ContextList>> = Lazy::new(|| {
    Mutex::new(ContextList::new())
});


pub fn init() {
    // init first context
    CONTEXT.lock();
    // CONTEXT.new_context.is_ok();
}