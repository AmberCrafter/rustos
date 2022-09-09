use alloc::{rc::Rc, boxed::Box};
use spin::Mutex;

use super::inode::Inode;

pub struct Upnode {
    pub parent: Option<Rc<Mutex<Upnode>>>,
    pub inode: Rc<Mutex<Box<dyn Inode>>>,
}

impl Upnode {
    pub fn from(parent: Rc<Mutex<Upnode>>, inode: Rc<Mutex<Box<dyn Inode>>>) -> Self {
        Self {
            parent: Some(parent.clone()),
            inode: inode.clone(),
        }
    }
}