pub mod ext2;
use std::collections::HashMap;
use core::sync::atomic::AtomicUsize;

use crate::Disk;
use self::ext2::Inode;


// pub struct FileSystem {
//     pwd: Vec<String>,
//     file_table: HashMap<FileID, Inode>,
// }

pub trait FileSystem {
    fn create(&mut self, path: &str); // directory: /..../  ;  file: /....
    fn open(&mut self, path: &str) -> Option<FileID>;
    fn read(&self, file_id: FileID) -> &[u8];
    fn write(&mut self, file_id: FileID, ctx: &[u8]);
    fn list_dir(&mut self, path: &str);

    fn parse_path(&self, path: &str) -> Result<Vec<String>, FileSystemErr> {
        let mut res = Vec::new();
        if path.starts_with("/") {
           res.push("/".to_string());
        } else {
            return Err(FileSystemErr::InvalidPath);
        }
        for p in path.split('/') {
            if p.len()!=0 {
                res.push(p.to_string());
            }
        }

        Ok(res)
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct FileID(usize);

#[derive(Debug)]
#[repr(u8)]
pub enum FileSystemErr {
    InvalidPath,
}

#[doc(hidden)]
#[allow(unused)]
fn little_endian_wrap_8(var: &Disk, base: usize, nums: usize) -> u8 {
    let mut res = 0;
    for i in 0..nums {
        res += (var[base + i] as u8) << i * 8;
    }
    res
}

#[doc(hidden)]
#[allow(unused)]
fn little_endian_wrap_16(var: &Disk, base: usize, nums: usize) -> u16 {
    let mut res = 0;
    for i in 0..nums {
        res += (var[base + i] as u16) << i * 8;
    }
    res
}

#[doc(hidden)]
#[allow(unused)]
fn little_endian_wrap_32(var: &Disk, base: usize, nums: usize) -> u32 {
    let mut res = 0;
    for i in 0..nums {
        res += (var[base + i] as u32) << i * 8;
    }
    res
}

#[doc(hidden)]
#[allow(unused)]
fn little_endian_wrap_64(var: &Disk, base: usize, nums: usize) -> u64 {
    let mut res = 0;
    for i in 0..nums {
        res += (var[base + i] as u64) << i * 8;
    }
    res
}

#[macro_export]
macro_rules! little_endian {
    ($var:expr, $base:expr, 1) => {
        little_endian_wrap_8(&$var, $base, 1)
    };

    ($var:expr, $base:expr, 2) => {
        little_endian_wrap_16(&$var, $base, 2)
    };

    ($var:expr, $base:expr, 4) => {
        little_endian_wrap_32(&$var, $base, 4)
    };

    ($var:expr, $base:expr, 8) => {
        little_endian_wrap_64(&$var, $base, 8)
    };
}

#[doc(hidden)]
#[allow(unused)]
fn vector_wrap(var: &Disk, base: usize, nums: usize) -> Vec<u8> {
    let mut res = vec![0; nums];
    for (i, &byte) in var[base..(base + nums)].iter().enumerate() {
        res[i] = byte;
    }
    res
}

#[macro_export]
macro_rules! vector {
    ($var:expr, $base:expr, $nums:expr ) => {
        // [0; $nums].copy_from_slice(&vector_wrap(&$var, $base, $nums))
        // vector_wrap(&$var, $base, $nums).as_slice().to_owned()
        {
            let mut tmp = [0; $nums];
            tmp.copy_from_slice(&vector_wrap(&$var, $base, $nums));
            tmp
        }
    };
}
