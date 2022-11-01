// Defile VFS interface

use std::convert::AsRef;
use std::path::Path;

use std::fmt::Debug;
use std::io::{Read, Result, Seek, Write};

pub trait VFS {
    type PATH: VPath;
    type FILE: VFile;
    type METADATA: VMetadata;

    fn path<T>(&self, path: T) -> Self::PATH
    where
        T: Into<String>;
}

pub trait VPath: Clone + Debug {
    type FS: VFS;

    fn open(&self) -> Result<<Self::FS as VFS>::FILE>;
    fn create(&self) -> Result<<Self::FS as VFS>::FILE>;
    fn append(&self) -> Result<<Self::FS as VFS>::FILE>;

    fn mkdir(&self) -> Result<()>;
    fn parent(&self) -> Option<Self>;
    fn file_name(&self) -> Option<String>;
    fn extension(&self) -> Option<String>;
    fn push<'a, T>(&mut self, path: T)
    where
        T: Into<&'a str>;
    fn exits(&self) -> bool;

    fn metadata(&self) -> Result<<Self::FS as VFS>::METADATA>;
    fn read_dir(&self) -> Result<Box<dyn Iterator<Item = String> + 'static>>;
}

pub trait VFile: Read + Write + Seek + Debug {}
impl<T> VFile for T where T: Read + Write + Seek + Debug {}

pub trait VMetadata {
    fn is_dir(&self) -> bool;
    fn is_file(&self) -> bool;
    fn len(&self) -> u64;
}
