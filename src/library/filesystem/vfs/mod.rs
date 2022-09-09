use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use spin::Mutex;

use crate::library::syscall::error::Errno;

use self::mount::Mount;

use super::flags::{Mode, MountFlags, OpenFlags};

use super::{file_descriptor::FileDescriptor, FileSystem, FsId};

pub mod mount;
pub mod test_fs;

pub fn init() {}

pub struct Vfs {
    fsid: FsId,
    mounts: Mutex<BTreeMap<&'static str, Mount>>,
}

impl Vfs {
    pub fn new(fsid: FsId) -> Self {
        Self {
            fsid,
            mounts: Mutex::new(BTreeMap::<&str, Mount>::new()),
        }
    }

    pub fn mount(
        &self,
        path: &'static str,
        file_system: Box<dyn FileSystem>,
        _flags: MountFlags,
    ) -> Result<(), Errno> {
        match self
            .mounts
            .lock()
            .insert(path, Mount::new(path, file_system))
        {
            None => Ok(()),
            Some(_) => Err(Errno::EINVAL),
        }
    }

    pub fn mount_count(&self) -> usize {
        self.mounts.lock().len()
    }

    pub fn unmount(&self, path: &str) -> Result<(), Errno> {
        match self.mounts.lock().remove(path) {
            None => Err(Errno::EINVAL),
            Some(_) => Ok(()),
        }
    }

    fn find_file_system_for_path(&self, path: &str) -> Option<Arc<Box<dyn FileSystem>>> {
        self.mounts
            .lock()
            .iter()
            .find(|&(p, _)| path.starts_with(p))
            .map(|(_, m)| m.file_system.clone())
    }
}

impl FileSystem for Vfs {
    fn fsid(&self) -> FsId {
        self.fsid
    }

    fn is_read_only(&self) -> bool {
        false
    }

    fn initialize(&self) -> bool {
        true
    }

    fn open(
        &self,
        path: &'static str,
        mode: Mode,
        flags: OpenFlags,
    ) -> Result<Box<dyn FileDescriptor>, Errno> {
        match self.find_file_system_for_path(path) {
            Some(fs) => fs.clone().open(path, mode, flags),
            None => Err(Errno::ENOENT),
        }
    }

    fn mkdir(&self, path: &str, mode: Mode) -> Result<(), Errno> {
        match self.find_file_system_for_path(path) {
            Some(fs) => fs.clone().mkdir(path, mode),
            None => Err(Errno::ENOENT),
        }
    }

    fn rmdir(&self, path: &str) -> Result<(), Errno> {
        match self.find_file_system_for_path(path) {
            Some(fs) => fs.clone().rmdir(path),
            None => Err(Errno::ENOENT),
        }
    }

    fn flush(&self) {
        todo!()
    }
}
