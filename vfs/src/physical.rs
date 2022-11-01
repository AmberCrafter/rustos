// using the standard library to build a file system
// stdlib should provide
// - Metadata
// - Path Operation (PathBuf)
// 
// 

use std::{fs::{Metadata, File, OpenOptions, DirBuilder, ReadDir, self, DirEntry}, path::{PathBuf, Path}, io::Result};

use crate::{VMetadata, VPath, VFS};



pub struct PhysicalFS{}
impl VFS for PhysicalFS {
    type PATH = PathBuf;
    type FILE = File;
    type METADATA = Metadata;
    fn path<T>(&self, path: T) -> PathBuf
        where
            T: Into<String> {
        PathBuf::from(path.into())
    }
}

impl VMetadata for Metadata {
    fn is_dir(&self) -> bool {
        self.is_dir()
    }
    fn is_file(&self) -> bool {
        self.is_file()
    }
    fn len(&self) -> u64 {
        self.len()
    }
}

impl VPath for PathBuf {
    type FS = PhysicalFS;
    fn open(&self) -> Result<File> {
        File::open(self)
    }
    fn create(&self) -> Result<File> {
        File::create(self)
    }
    fn append(&self) -> Result<File> {
        OpenOptions::new()
            .write(true)
            .append(true)
            .open(self)
    }
    fn mkdir(&self) -> Result<()> {
        DirBuilder::new()
            .recursive(true)
            .create(self)
    }
    fn parent(&self) -> Option<Self> {
        <Path>::parent(self).map(|path| path.to_path_buf())
    }
    fn file_name(&self) -> Option<String> {
        <Path>::file_name(self).map(|name| name.to_string_lossy().into_owned())
    }
    fn push<'a, T>(&mut self, path: T)
        where
            T: Into<&'a str> {
        <PathBuf>::push(self, path.into());
    }
    fn exits(&self) -> bool {
        <Path>::exists(self)
    }
    fn metadata(&self) -> Result<<Self::FS as crate::VFS>::METADATA> {
        <Path>::metadata(self)
    }
    fn read_dir(&self) -> Result<Box<dyn Iterator<Item = String> + 'static >> {
        // fs::read_dir(path) -> Result<ReadDir, Error>
        // ReadDir -> Result<DirEntry, Error>
        let read_dir = fs::read_dir(self.as_path())?;
        let iter = read_dir.map(|entry| entry.unwrap().path().to_string_lossy().to_string());
        Ok(Box::new(iter))
    }

}

#[cfg(test)]
mod tests {
    use std::io::{Read, Result};
    use std::path::PathBuf;

    use super::*;
    use VPath;

    #[test]
    fn read_file() {
        let path = PathBuf::from("Cargo.toml");
        let mut file = path.open().unwrap();
        let mut string: String = "".to_owned();
        file.read_to_string(&mut string).unwrap();
        assert!(string.len() > 10);
        assert!(path.exists());
        assert!(path.metadata().unwrap().is_file());
        assert!(PathBuf::from(".").metadata().unwrap().is_dir());
    }

    #[test]
    fn pareant() {
        let src = PathBuf::from("./src");
        let parent = PathBuf::from(".");
        assert_eq!(src.parent().unwrap(), parent);
        assert_eq!(PathBuf::from("/").parent(), None);
    }

    #[test]
    fn read_dir() {
        let src = PathBuf::from("./src");
        let entries: Vec<String> = src.read_dir().unwrap().collect();
        println!("{:#?}", entries);
    }
}