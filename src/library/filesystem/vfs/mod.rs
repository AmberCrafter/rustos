use alloc::sync::Arc;
use alloc::{collections::LinkedList, boxed::Box};
use spin::Mutex;

use crate::library::syscall::error::Errno;

use super::flags::{MountFlags, Mode, OpenFlags};

use super::{FileSystem, FsId, file_descriptor::FileDescriptor};

// pub mod flags;
// mod mount;
// pub mod test_fs;


pub fn init() {}

pub struct Vfs {
    fsid: FsId,
    mounts: Mutex<LinkedList<Mount>>,
}

struct Mount {
    path: &'static str,
    file_system: Arc<Box<dyn FileSystem>>,
}

impl Mount {
    fn new(path: &'static str, file_system: Box<dyn FileSystem>) -> Self {
        Self {
            path, 
            file_system: Arc::new(file_system)
        }
    }
}

impl Vfs {
    pub fn new(fsid: FsId) -> Self {
        Self { fsid, mounts: Mutex::new(LinkedList::new()) }
    }

    pub fn mount(&self, path: &'static str, file_system: Box<dyn FileSystem>, _flags: MountFlags) -> Result<(), Errno> {
        self.mounts.lock().push_back(Mount::new(path, file_system));
        Ok(())
    }

    pub fn mount_count(&self) -> usize {
        self.mounts.lock().len()
    }

    pub fn unmount(&mut self) -> Result<(), Errno> {
        Ok(())
    }

    fn find_file_system_for_path(&self, path: &str) -> Option<Arc<Box<dyn FileSystem>>> {
        self.mounts
            .lock()
            .iter()
            .find(|&mount| path.starts_with(mount.path))
            .take()
            .map(|m| m.file_system.clone())
            .take()
    }
}

impl FileSystem for Vfs {
    fn fsid(&self) -> FsId {
        self.fsid
    }

    fn is_read_only(&self) -> bool {
        false
    }

    fn open(&self, path: &str, mode: Mode, flags: OpenFlags) -> Result<FileDescriptor, Errno> {
        match self.find_file_system_for_path(path) {
            Some(fs) => fs.open(path, mode, flags),
            None => Err(Errno::ENOENT),
        }
    }

    fn mkdir(&self, path: &str, mode: Mode) -> Result<(), Errno> {
        match self.find_file_system_for_path(path) {
            Some(fs) => fs.mkdir(path, mode),
            None => Err(Errno::ENOENT),
        }
    }

    fn rmdir(&self, path: &str) -> Result<(), Errno> {
        match self.find_file_system_for_path(path) {
            Some(fs) => fs.rmdir(path),
            None => Err(Errno::ENOENT),
        }
    }

    fn flush(&self) {
        todo!()
    }
}
