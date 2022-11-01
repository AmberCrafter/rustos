use super::GROUPDESCRIPTOR_SIZE;

#[derive(Debug)]
#[repr(C)]
pub struct BlockGroupDescriptor {
    pub block_bitmap: u32,
    pub inode_bitmap: u32,
    pub inode_table: u32,
    pub free_blocks_count: u16,
    pub free_inodes_count: u16,
    pub used_dirs_count: u16,
    pub padding: u16,
    pub reserved: [u8; GROUPDESCRIPTOR_SIZE - 20],
}

impl Default for BlockGroupDescriptor {
    fn default() -> Self {
        Self {
            block_bitmap: 0,
            inode_bitmap: 0,
            inode_table: 0,
            free_blocks_count: 0,
            free_inodes_count: 0,
            used_dirs_count: 0,
            padding: 0,
            reserved: [0; GROUPDESCRIPTOR_SIZE - 20],
        }
    }
}

impl BlockGroupDescriptor {
    pub fn new(
        block_bitmap: u32,
        inode_bitmap: u32,
        inode_table: u32,
        free_blocks_count: u16,
        free_inodes_count: u16,
        used_dirs_count: u16,
        padding: u16,
        reserved: [u8; GROUPDESCRIPTOR_SIZE - 20],
    ) -> Self {
        Self {
            block_bitmap,
            inode_bitmap,
            inode_table,
            free_blocks_count,
            free_inodes_count,
            used_dirs_count,
            padding,
            reserved,
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct BlockGroupDescriptorTable(Vec<BlockGroupDescriptor>);

impl BlockGroupDescriptorTable {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn append(&mut self, gd: BlockGroupDescriptor) {
        self.0.push(gd);
    }

    pub fn get(&self, idx: usize) -> Option<&BlockGroupDescriptor> {
        self.0.get(idx)
    }
}

pub struct BlockGroupDescriptorTableIter<'a> {
    index: usize,
    table: &'a BlockGroupDescriptorTable,
}

impl<'a> Iterator for BlockGroupDescriptorTableIter<'a> {
    type Item = &'a BlockGroupDescriptor;
    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        self.table.get(self.index - 1)
    }
}

impl<'a> IntoIterator for &'a BlockGroupDescriptorTable {
    type Item = &'a BlockGroupDescriptor;
    type IntoIter = BlockGroupDescriptorTableIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        BlockGroupDescriptorTableIter {
            index: 0,
            table: self,
        }
    }
}
