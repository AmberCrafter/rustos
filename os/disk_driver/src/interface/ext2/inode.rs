use std::{cell::RefCell, rc::Rc};

use bitflags::bitflags;
use byteorder::{ByteOrder, LittleEndian};

use crate::encode_little_endian;

#[derive(Debug, Default)]
pub struct InodeTable(Vec<Rc<RefCell<Inode>>>);

impl InodeTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, index: usize) -> Option<&Rc<RefCell<Inode>>> {
        self.0.get(index)
    }

    pub fn append(&mut self, inode: Inode) {
        self.0.push(Rc::new(RefCell::new(inode)));
    }
}

pub struct InodeTableIter<'a> {
    index: usize,
    table: &'a InodeTable,
}

impl<'a> Iterator for InodeTableIter<'a> {
    type Item = &'a Rc<RefCell<Inode>>;
    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        self.table.get(self.index - 1)
    }
}

impl<'a> IntoIterator for &'a InodeTable {
    type Item = &'a Rc<RefCell<Inode>>;
    type IntoIter = InodeTableIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        InodeTableIter {
            index: 0,
            table: self,
        }
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Inode {
    pub mode: u16,
    pub uid: u16,
    pub size: u32,
    pub atime: u32,
    pub ctime: u32,
    pub mtime: u32,
    pub dtime: u32,
    pub gid: u16,
    pub links_count: u16,
    pub blocks: u32,
    pub flags: u32,
    pub osd1: u32,
    pub block: [u32; 15],
    pub generation: u32,
    pub file_acl: u32,
    pub dir_acl: u32,
    pub faddr: u32,
    pub osd2: [u8; 12],
}

impl Default for Inode {
    fn default() -> Self {
        Self {
            mode: 0,
            uid: 0,
            size: 0,
            atime: 0,
            ctime: 0,
            mtime: 0,
            dtime: 0,
            gid: 0,
            links_count: 0,
            blocks: 0,
            flags: 0,
            osd1: 0,
            block: [0; 15],
            generation: 0,
            file_acl: 0,
            dir_acl: 0,
            faddr: 0,
            osd2: [0; 12],
        }
    }
}

impl Inode {
    pub fn new(
        mode: u16,
        uid: u16,
        size: u32,
        atime: u32,
        ctime: u32,
        mtime: u32,
        dtime: u32,
        gid: u16,
        links_count: u16,
        blocks: u32,
        flags: u32,
        osd1: u32,
        block: [u32; 15],
        generation: u32,
        file_acl: u32,
        dir_acl: u32,
        faddr: u32,
        osd2: [u8; 12],
    ) -> Self {
        Self {
            mode,
            uid,
            size,
            atime,
            ctime,
            mtime,
            dtime,
            gid,
            links_count,
            blocks,
            flags,
            osd1,
            block,
            generation,
            file_acl,
            dir_acl,
            faddr,
            osd2,
        }
    }
}

impl From<Inode> for [u8; 128] {
    fn from(inode: Inode) -> Self {
        let mut buf: Vec<u8> = Vec::with_capacity(128);
        for byte in encode_little_endian!(inode.mode, 2) {
            buf.push(byte);
        }
        for byte in encode_little_endian!(inode.uid, 2) {
            buf.push(byte);
        }
        for byte in encode_little_endian!(inode.size, 4) {
            buf.push(byte);
        }
        for byte in encode_little_endian!(inode.atime, 4) {
            buf.push(byte);
        }
        for byte in encode_little_endian!(inode.ctime, 4) {
            buf.push(byte);
        }
        for byte in encode_little_endian!(inode.mtime, 4) {
            buf.push(byte);
        }
        for byte in encode_little_endian!(inode.dtime, 4) {
            buf.push(byte);
        }
        for byte in encode_little_endian!(inode.gid, 2) {
            buf.push(byte);
        }
        for byte in encode_little_endian!(inode.links_count, 2) {
            buf.push(byte);
        }
        for byte in encode_little_endian!(inode.blocks, 4) {
            buf.push(byte);
        }
        for byte in encode_little_endian!(inode.flags, 4) {
            buf.push(byte);
        }
        for byte in encode_little_endian!(inode.osd1, 4) {
            buf.push(byte);
        }
        for block in inode.block {
            for byte in encode_little_endian!(block, 4) {
                buf.push(byte);
            }
        }
        for byte in encode_little_endian!(inode.generation, 4) {
            buf.push(byte);
        }
        for byte in encode_little_endian!(inode.file_acl, 4) {
            buf.push(byte);
        }
        for byte in encode_little_endian!(inode.dir_acl, 4) {
            buf.push(byte);
        }
        for byte in encode_little_endian!(inode.faddr, 4) {
            buf.push(byte);
        }
        for osd2 in inode.osd2 {
            for byte in encode_little_endian!(osd2, 1) {
                buf.push(byte);
            }
        }
        buf.as_chunks::<128>().0[0]
    }
}

bitflags! {
    // https://www.nongnu.org/ext2-doc/ext2.html#i-mode
    // Inode::mode
    pub struct InodeMode: u16 {
        // access rights
        const IXOTH = 1 << 0;
        const IWOTH = 1 << 1;
        const IROTH = 1 << 2;
        const IXGRP = 1 << 3;
        const IWGRP = 1 << 4;
        const IRGRP = 1 << 5;
        const IXUSR = 1 << 6;
        const IWUSR = 1 << 7;
        const IRUSR = 1 << 8;

        // process execution override
        const ISVTX = 1 << 9;
        const ISGID = 1 << 10;
        const ISUID = 1 << 11;

        // file format
        const IFIFO = 1 << 12;
        const IFCHR = 1 << 13;
        const IFDIR = 1 << 14;
        const IFBLK = 1 << 13 | 1 << 14;
        const IFREG = 1 << 15;
        const IFLNK = 1 << 13 | 1 << 15;
        const IFSOCK = 1 << 14 | 1 << 15;
    }
}
