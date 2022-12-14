use alloc::vec::Vec;

use crate::library::syscall::error::Errno;

pub trait Inode {
    fn read_bytes(&self, _offset: usize, _buffer: Vec<u8>) -> Result<usize, Errno>;
    fn write_bytes(&self, _offset: usize, _buffer: Vec<u8>) -> Result<usize, Errno>;

    fn size(&self) -> usize;
}