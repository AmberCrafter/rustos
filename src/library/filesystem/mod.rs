pub mod ext2;
pub mod file_descriptor;
pub mod vfs;
pub mod flags;
pub mod stat;
pub mod inode;
pub mod upnode;

use alloc::boxed::Box;

use self::{flags::{Mode, OpenFlags}, file_descriptor::FileDescriptor};
use super::syscall::error::Errno;


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FsId(u32);

pub trait FileSystem {
    fn fsid(&self) -> FsId;
    fn initialize(&self) -> bool;
    fn is_read_only(&self) -> bool;

    fn open(&self, path: &'static str, mode: Mode, flags: OpenFlags) -> Result<Box<dyn FileDescriptor>, Errno>;
    fn mkdir(&self, path: & str, mode: Mode) -> Result<(), Errno>;
    fn rmdir(&self, path: & str) -> Result<(), Errno>;


    fn flush(&self);
}

impl From<u32> for FsId {
    fn from(fsid: u32) -> Self {
        FsId(fsid)
    }
}

