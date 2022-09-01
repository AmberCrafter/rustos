use super::{FileSystem, file_descriptor::FileDescriptor};

pub struct Ext2Fs {}

impl FileSystem for Ext2Fs {
    fn fsid(&self) -> super::FsId {
        todo!()
    }

    fn is_read_only(&self) -> bool {
        todo!()
    }

    fn mkdir(&self, path: &str, mode: super::flags::Mode) -> Result<(), crate::library::syscall::error::Errno> {
        todo!()
    }

    fn open(&self, path: &str, mode: super::flags::Mode, flags: super::flags::OpenFlags) -> Result<FileDescriptor, crate::library::syscall::error::Errno> {
        todo!()
    }

    fn rmdir(&self, path: &str) -> Result<(), crate::library::syscall::error::Errno> {
        todo!()
    }

    fn flush(&self) {
        todo!()
    }
}