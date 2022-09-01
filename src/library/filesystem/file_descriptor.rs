// use alloc::vec::Vec;

// use crate::library::syscall::error::Errno;

// use super::stat::Stat;

// pub enum Seek {
//     Set,
//     Cur,
//     End
// }

// pub trait FileDescriptor {
//     fn is_readalbe(&self) -> bool;
//     fn is_writable(&self) -> bool;

//     fn seek(&mut self, _buffer: Vec<u8>) -> Result<usize, Errno>;
//     fn write(&mut self, _buffer: Vec<u8>) -> Result<usize, Errno>;
//     fn stat(&self) -> Result<Stat, Errno>;

//     fn absolute_path(&self) -> &str;
// }

pub struct FileDescriptor {}
