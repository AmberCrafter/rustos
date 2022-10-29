use super::{ Bitmap, InodeTable };

#[derive(Debug, Default)]
#[repr(C)]
pub struct Group {
    pub block_bitmap: Bitmap,
    pub inode_bitmap: Bitmap,
    pub inode_table: InodeTable,
}

impl Group {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct GroupTable(Vec<Group>);

impl GroupTable {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn append(&mut self, gd: Group) {
        self.0.push(gd);
    }

    pub fn get(&self, idx: usize) -> Option<&Group> {
        self.0.get(idx)
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut Group> {
        self.0.get_mut(idx)
    }
}

pub struct GroupTableIter<'a> {
    index: usize,
    table: &'a GroupTable,
}

impl<'a> Iterator for GroupTableIter<'a> {
    type Item = &'a Group;
    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        self.table.get(self.index - 1)
    }
}

impl<'a> IntoIterator for &'a GroupTable {
    type Item = &'a Group;
    type IntoIter = GroupTableIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        GroupTableIter {
            index: 0,
            table: self,
        }
    }
}