use alloc::{boxed::Box, sync::Rc};

use crate::library::filesystem::FileSystem;

pub struct Mount {
    path: &'static str,
    pub file_system: Rc<Box<dyn FileSystem>>,
}

impl Mount {
    pub fn new(path: &'static str, file_system: Box<dyn FileSystem>) -> Self {
        Self {
            path,
            file_system: Rc::new(file_system),
        }
    }
}
