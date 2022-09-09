#[derive(Debug)]
pub struct Dentry {
    pub inode_index: u32,
    rec_len: u16,
    name_len: u8,
    pub file_type: u8,
    pub name: Vec<u8>,
}

impl Default for Dentry {
    fn default() -> Self {
        Self {
            inode_index: 0,
            rec_len: 0,
            name_len: 0,
            file_type: 0,
            name: Vec::new(),
        }
    }
}

#[derive(Debug, Default)]
pub struct DentryTable(Vec<Dentry>);

impl DentryTable {
    pub fn new() -> Self {
        Self( Vec::new() )
    }
    pub fn append(&mut self, dentry: Dentry) {
        self.0.push(dentry);
    }
    pub fn get(&self, index: usize) -> Option<Dentry> {
        self.0.get(index)
    }
}

pub struct DentryTableIter<'a> {
    index: usize,
    table: &'a DentryTable
}

impl<'a> Iterator for DentryTableIter<'a> {
    type Item = &'a Dentry;
    fn next(&mut self) -> Option<Self::Item> {
        self.index+=1;
        self.table.get(self.index-1)
    }
}

impl<'a> IntoIterator for &'a DentryTable {
    type Item = &'a Dentry;
    type IntoIter = DentryTableIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        DentryTableIter {
            index: 0,
            table: self
        }
    }
}