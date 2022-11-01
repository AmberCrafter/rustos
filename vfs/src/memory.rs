use std::{
    cmp,
    collections::HashMap,
    io::{Read, Result, Seek, SeekFrom, Write, Error, ErrorKind},
    ops::{Deref, DerefMut},
    sync::{Arc, RwLock},
};

use crate::{VMetadata, VFS, VPath};

pub type Filename = String;

#[derive(Debug, Clone)]
pub struct DataHandle(Arc<RwLock<Vec<u8>>>);

impl DataHandle {
    fn new() -> Self {
        Self(Arc::new(RwLock::new(Vec::new())))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum NodeKind {
    Directory,
    File,
}

pub struct MemoryMetadata {
    kind: NodeKind,
    len: usize,
}

impl VMetadata for MemoryMetadata {
    fn is_dir(&self) -> bool {
        self.kind == NodeKind::Directory
    }
    fn is_file(&self) -> bool {
        self.kind == NodeKind::File
    }
    fn len(&self) -> u64 {
        self.len as u64
    }
}

#[derive(Debug)]
struct FsNode {
    kind: NodeKind,
    pub children: HashMap<String, FsNode>,
    pub data: DataHandle,
}

impl FsNode {
    pub fn new_directory() -> Self {
        Self {
            kind: NodeKind::Directory,
            children: HashMap::new(),
            data: DataHandle::new(),
        }
    }

    pub fn new_file() -> Self {
        Self {
            kind: NodeKind::File,
            children: HashMap::new(),
            data: DataHandle::new(),
        }
    }

    fn metadata(&mut self) -> MemoryMetadata {
        MemoryMetadata {
            kind: self.kind.clone(),
            len: self.data.0.read().unwrap().len(),
        }
    }
}

#[derive(Debug)]
pub struct MemoryFSImpl {
    root: FsNode,
}

pub type MemoryFSHandle = Arc<RwLock<MemoryFSImpl>>;

#[derive(Debug)]
pub struct MemoryFS {
    handle: MemoryFSHandle,
}

impl MemoryFS {
    pub fn new() -> Self {
        Self {
            handle: Arc::new(RwLock::new(MemoryFSImpl {
                root: FsNode::new_directory(),
            })),
        }
    }
}

impl Default for MemoryFS {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct MemoryFile {
    pub data: DataHandle,
    pub pos: usize,
}

impl Read for MemoryFile {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let n = (&self.data.0.write().unwrap().deref()[self.pos..]).read(buf)?;
        self.pos += n;
        Ok(n)
    }
}

impl Write for MemoryFile {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let mut guard = self.data.0.write().unwrap();
        let vec = guard.deref_mut();

        // cursor
        let pos = self.pos as u64;
        let len = vec.len() as u64;
        let amt = pos.saturating_sub(len);

        vec.resize((len + amt) as usize, 0);
        {
            let pos = pos as usize;
            let space = vec.len() - pos;
            let (left, right) = buf.split_at(cmp::min(space, buf.len()));
            vec[pos..pos + left.len()].clone_from_slice(left);
            vec.extend_from_slice(right);
        }

        self.pos = pos as usize + buf.len();
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl Seek for MemoryFile {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        let pos = match pos {
            SeekFrom::Start(n) => {
                self.pos = n as usize;
                return Ok(n);
            }
            SeekFrom::End(n) => self.data.0.read().unwrap().len() as i64 + n,
            SeekFrom::Current(n) => self.pos as i64 + n,
        };

        if pos < 0 {
            Err(Error::new(ErrorKind::InvalidInput, "Invalid seek to a negative position"))
        } else {
            self.pos = pos as usize;
            Ok(self.pos as u64)
        }
    }
}

#[derive(Debug, Clone)]
pub struct MemoryPath {
    pub path: Filename,
    fs: MemoryFSHandle
}

impl MemoryPath {
    pub fn new(fs: &MemoryFSHandle, path: Filename) -> Self {
        Self {
            path,
            fs: fs.clone()
        }
    }
    fn with_node<R, F>(&self, f:F) -> Result<R>
    where
        F: FnOnce(&mut FsNode) -> R
    {
        let root = &mut self.fs.write().unwrap().root;
        let mut components: Vec<&str> = self.path.split('/').collect();
        components.reverse();
        components.pop();
        traverse_with(root, &mut components, f)
    }
    pub fn decompose_path(&self) -> (Option<String>, String) {
        let mut split = self.path.rsplitn(2, '/');
        if let Some(mut filename) = split.next() {
            if let Some(mut parent) = split.next() {
                if parent.is_empty() {
                    parent = "/";
                }
                if filename.is_empty() {
                    filename = parent;
                    return (None, filename.to_owned());
                }
                return (Some(parent.to_owned()), filename.to_owned());
            }
        }
        (None, self.path.clone())
    }
}

fn traverse_with<R, F>(node: &mut FsNode, components: &mut Vec<&str>, f:F) -> Result<R>
where
    F: FnOnce(&mut FsNode) -> R
{
    if let Some(component) = components.pop() {
        if component.is_empty() {
            return traverse_with(node, components, f);
        }
        let entry = node.children.get_mut(component);
        if let Some(directory) = entry {
            traverse_with(directory, components, f)
        } else {
            Err(
                Error::new(
                    ErrorKind::Other,
                    format!("File not found: {:?}", component)
                )
            )
        }
    } else {
        Ok(f(node))
    }
}

fn traverse_mkdir(node: &mut FsNode, components: &mut Vec<&str>) -> Result<()> {
    if let Some(component) = components.pop() {
        let directory = &mut node.children
            .entry(component.to_owned())
            .or_insert_with(FsNode::new_directory);
        traverse_mkdir(directory, components)
    } else {
        Ok(())
    }
}

impl VPath for MemoryPath {
    type FS = MemoryFS;
    fn open(&self) -> Result<MemoryFile> {
        let datahandle = self.with_node(
            |node| node.data.clone()
        ).unwrap();
        Ok(
            MemoryFile { data: datahandle, pos: 0 }
        )
    }

    fn create(&self) -> Result<MemoryFile> {
        let parent_path = self.parent().unwrap();
        let data = parent_path.with_node(
            |node| {
                let file_node = node.children
                    .entry(self.file_name().unwrap())
                    .or_insert_with(FsNode::new_file);
                file_node.data.clone()
            }
        )?;
        data.0.write().unwrap().clear();
        Ok(MemoryFile {
            data,
            pos: 0
        })
    }

    fn append(&self) -> Result<MemoryFile> {
        let parent_path = self.parent().unwrap();
        let data = parent_path.with_node(
            |node| {
                let file_node = node.children
                    .entry(self.file_name().unwrap())
                    .or_insert_with(FsNode::new_file);
                file_node.data.clone()
            }
        )?;
        let len = data.0.read().unwrap().len();
        Ok(MemoryFile { data, pos: len })
    }
    
    fn parent(&self) -> Option<Self> {
        self.decompose_path().0.map(
            |parent| MemoryPath::new(
                &self.fs.clone(), 
                parent
            )
        )
    }

    fn file_name(&self) -> Option<String> {
        Some(self.decompose_path().1)
    }

    fn extension(&self) -> Option<String> {
        if let Some(name) = self.file_name() {
            // rsplit is reverse array
            let mut spliter = name.rsplit('.');
            let suffix = spliter.next().unwrap();
            if name.len()!=suffix.len() {
                Some(suffix.to_string())
            } else {
                None
            }
        } else {
            None
        }
    }

    fn push<'a, T>(&mut self, path: T)
        where
            T: Into<&'a str> {
        if !self.path.ends_with('/') {
            self.path.push('/');
        }
        self.path.push_str(path.into());
    }

    fn mkdir(&self) -> Result<()> {
        let root = &mut self.fs.write().unwrap().root;
        let mut components: Vec<&str> = self.path.split('/').collect();
        components.reverse();
        components.pop();
        traverse_mkdir(root, &mut components)
    }

    fn exits(&self) -> bool {
        self.with_node(
            |node| ()
        ).is_ok()
    }

    fn metadata(&self) -> Result<MemoryMetadata> {
        self.with_node(FsNode::metadata)
    }

    fn read_dir(&self) -> Result<Box<dyn Iterator<Item = String> + 'static >> {
        self.with_node(
            |node| {
                let children: Vec<String> = node.children.keys().map(
                    |name| {
                        MemoryPath::new(
                            &self.fs,
                            self.path.clone() + "/" + name
                        ).path.as_str().to_string()
                    }
                ).collect();
                Box::new(children.into_iter()) as Box<dyn Iterator<Item = String>>
            }
        )
    }
}

impl<'a> From<&'a MemoryPath> for String {
    fn from(path: &'a MemoryPath) -> Self {
        path.path.clone()
    }
}

impl PartialEq for MemoryPath {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl VFS for MemoryFS {
    type PATH = MemoryPath;
    type FILE = MemoryFile;
    type METADATA = MemoryMetadata;
    fn path<T>(&self, path: T) -> Self::PATH
        where
            T: Into<String> {
        MemoryPath::new(&self.handle, path.into())
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::write;

    use super::*;
    #[test]
    fn mkdir() {
        let fs = MemoryFS::new();
        let path = fs.path("/foo/bar/baz");
        assert!(!path.exits(), "Path should not exist");
        path.mkdir().unwrap();
        assert!(path.exits(), "Path should exist");
        assert!(path.metadata().unwrap().is_dir(), "Path should be dir");
        assert!(!path.metadata().unwrap().is_file(), "Path should not be file");
        assert!(path.metadata().unwrap().len()==0, "Path size should be 0");
        println!("{:#?}", path);
    }

    #[test]
    fn read_empty_file() {
        let fs = MemoryFS::new();
        let path = fs.path("/footbar.txt");
        path.create().unwrap();
        let mut file = path.open().unwrap();
        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();
        assert_eq!(buf, "");
    }

    #[test]
    fn write_and_read_file() {
        let fs = MemoryFS::new();
        let path = fs.path("/foobar.txt");
        {
            let mut file = path.create().unwrap();
            write!(file, "Hello world").unwrap();
            write!(file, "!").unwrap();
        }
        // Check Seek
        {
            let mut file = path.open().unwrap();
            let mut buf = String::new();
            file.read_to_string(&mut buf).unwrap();
            assert_eq!(buf, "Hello world!");
        }
        {
            let mut file = path.open().unwrap();
            file.seek(SeekFrom::Start(1)).unwrap();
            write!(file, "a").unwrap();
        }
        {
            let mut file = path.open().unwrap();
            let mut buf = String::new();
            file.read_to_string(&mut buf).unwrap();
            assert_eq!(buf, "Hallo world!");
        }
        {
            let mut file = path.open().unwrap();
            let mut buf = String::new();
            file.seek(SeekFrom::End(-1)).unwrap();
            file.read_to_string(&mut buf).unwrap();
            assert_eq!(buf, "!");
        }
        // Check create a empty file by write
        {
            let file = path.create().unwrap();
        }
        {
            let mut file = path.open().unwrap();
            let mut buf = String::new();
            file.read_to_string(&mut buf).unwrap();
            assert_eq!(buf, "");
        }
    }

    #[test]
    fn append() {
        let fs = MemoryFS::new();
        let path = fs.path("/foobar.txt");
        {
            let mut file = path.append().unwrap();
            write!(file, "Hello").unwrap();
            write!(file, " world").unwrap();
        }
        {
            let mut file = path.open().unwrap();
            let mut buf = String::new();
            file.read_to_string(&mut buf).unwrap();
            assert_eq!(buf, "Hello world");
        }
        {
            let mut file = path.append().unwrap();
            write!(file, "!").unwrap();
        }
        {
            let mut file = path.open().unwrap();
            let mut buf = String::new();
            file.read_to_string(&mut buf);
            assert_eq!(buf, "Hello world!");
        }
    }

    #[test]
    fn push() {
        let fs = MemoryFS::new();
        let mut path = fs.path("/");
        let mut path2 = path.clone();
        assert_eq!(String::from(&path), "/");
        path.push("foo");
        assert_eq!(String::from(&path), "/foo");
        path.push("bar");
        assert_eq!(String::from(&path), "/foo/bar");

        assert_eq!(String::from(&path2), "/");
        path2.push("foo/bar");
        assert_eq!(String::from(&path2), "/foo/bar");
    }

    #[test]
    fn parent() {
        let fs = MemoryFS::new();
        let path = fs.path("/foo");
        let path2 = fs.path("/foo/bar");
        assert_eq!(path2.parent().unwrap(), path);
        assert_eq!(String::from(&path.parent().unwrap()), "/");
        assert_eq!(fs.path("/").parent(), None);
    }

    #[test]
    fn read_dir() {
        let fs = MemoryFS::new();
        let path = fs.path("/foo");
        let path2 = fs.path("/foo/bar");
        let path3 = fs.path("/foo/baz.txt");
        path2.mkdir().unwrap();
        path3.create().unwrap();
        let mut entries = path.read_dir().unwrap().collect::<Vec<_>>();
        entries.sort();
        assert_eq!(entries, vec![
            "/foo/bar".to_owned(),
            "/foo/baz.txt".to_owned(),
        ]);
    }

    #[test]
    fn file_name() {
        let fs = MemoryFS::new();
        let path = fs.path("/foo/bar.txt");
        assert_eq!(path.file_name(), Some("bar.txt".to_string()));
        assert_eq!(path.extension(), Some("txt".to_string()));
        assert_eq!(path.parent().unwrap().extension(), None);
    }
}