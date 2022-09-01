use crate::library::{filesystem::{FsId, FileSystem, flags::{Mode, OpenFlags}, file_descriptor::FileDescriptor}, syscall::error::Errno};

pub struct TestFs {
    fsid: FsId,
    init_callback: Option<fn()>,
    open_callback: Option<fn(&str, Mode, OpenFlags)>
}

impl TestFs {
    pub fn from(fsid: FsId) -> Self {
        Self {
            fsid,
            init_callback: None,
            open_callback: None
        }
    }

    pub fn set_initialize_callback(&mut self, callback: fn()) {
        self.init_callback = Some(callback);
    }
}

impl FileSystem for TestFs {
    fn fsid(&self) -> FsId {
        self.fsid
    }

    fn initialize(&self) -> bool {
        true
    }

    fn is_read_only(&self) -> bool {
        false
    }

    fn mkdir(&self, path: &str, mode: Mode) -> Result<(), Errno> {
        todo!()
    }

    fn open(&self, path: &str, mode: Mode, flags: OpenFlags) -> Result<FileDescriptor, Errno> {
        todo!()
    }

    fn rmdir(&self, path: &str) -> Result<(), Errno> {
        todo!()
    }

    fn flush(&self) {}
}