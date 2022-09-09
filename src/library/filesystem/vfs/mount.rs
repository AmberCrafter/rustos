use alloc::{boxed::Box, sync::Arc};

use crate::library::filesystem::FileSystem;

pub struct Mount {
    path: &'static str,
    pub file_system: Arc<Box<dyn FileSystem>>,
}

impl Mount {
    pub fn new(path: &'static str, file_system: Box<dyn FileSystem>) -> Self {
        Self {
            path,
            file_system: Arc::new(file_system),
        }
    }
}
