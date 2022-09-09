pub mod superblock;

use alloc::boxed::Box;

use crate::library::syscall::error::Errno;

use super::{FileSystem, file_descriptor::FileDescriptor, flags::{OpenFlags, Mode}, FsId};

pub struct Ext2FileSystem {
    fsid: FsId
}

impl FileSystem for Ext2FileSystem {
    fn fsid(&self) -> super::FsId {
        self.fsid
    }

    fn is_read_only(&self) -> bool {
        false
    }
    
    fn initialize(&self) -> bool {
        true
    }

    fn mkdir(&self, path: &str, mode: Mode) -> Result<(), Errno> {
        todo!()
    }

    fn open(&self, path: &'static str, mode: Mode, flags: OpenFlags) -> Result<Box<dyn FileDescriptor>, Errno> {
        todo!()
    }

    fn rmdir(&self, path: &str) -> Result<(), Errno> {
        todo!()
    }

    fn flush(&self) {
        todo!()
    }
}