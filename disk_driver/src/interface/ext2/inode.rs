use bitflags::bitflags;

#[derive(Debug, Default)]
pub struct InodeTable(Vec<Inode>);

impl InodeTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, index: usize) -> Option<&Inode> {
        self.0.get(index)
    }

    pub fn append(&mut self, inode: Inode) {
        self.0.push(inode);
    }
}

pub struct InodeTableIter<'a> {
    index: usize,
    table: &'a InodeTable,
}

impl<'a> Iterator for InodeTableIter<'a> {
    type Item = &'a Inode;
    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        self.table.get(self.index - 1)
    }
}

impl<'a> IntoIterator for &'a InodeTable {
    type Item = &'a Inode;
    type IntoIter = InodeTableIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        InodeTableIter {
            index: 0,
            table: self,
        }
    }
}

#[derive(Debug)]
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
