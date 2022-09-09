use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Dentry {
    pub inode_index: u32,
    rec_len: u16,
    name_len: u8,
    pub file_type: DentryFiletype,
    pub name: String,
    // this is used to descript the directory under this inode
    pub dentrymap: Option<DentryMap>,
}

impl Default for Dentry {
    fn default() -> Self {
        Self {
            inode_index: 0,
            rec_len: 0,
            name_len: 0,
            file_type: DentryFiletype::Unknown,
            name: String::new(),
            dentrymap: None,
        }
    }
}

impl Dentry {
    pub fn new(
        inode_index: u32,
        rec_len: u16,
        name_len: u8,
        file_type: u8,
        name: String,
    ) -> Self {
        let file_type: DentryFiletype = file_type.into();
        match file_type {
            DentryFiletype::DirecotryFile => Self { inode_index, rec_len, name_len, file_type, name, dentrymap: Some(DentryMap::new()) },
            _ => Self { inode_index, rec_len, name_len, file_type, name, dentrymap: None }
        }
        
    }

    pub fn padding_len(&self) -> usize {
        (self.rec_len - self.name_len as u16 - 8) as usize
    }

    pub fn update_dentrymap(&mut self, dentrymap: DentryMap) {
        self.dentrymap = Some(dentrymap);
    }
}

#[derive(Debug, Default, Clone)]
pub struct DentryMap(BTreeMap<String, Dentry>);

impl DentryMap {
    pub fn new() -> Self {
        Self( BTreeMap::new() )
    }
    pub fn append(&mut self, dentry: Dentry) -> Result<(), DentryMapErr> {
        let name = &dentry.name;
        if name.len()>0 {
            if self.0.contains_key(name) {
                return Err(DentryMapErr::FileExist);
            }
            self.0.insert(name.to_owned(), dentry);
        } else {
            return Err(DentryMapErr::InvalidName);
        }
        Ok(())
    }
    pub fn get(&self, name: &str) -> Option<&Dentry> {
        self.0.get(name)
    }
    pub fn get_mut(&mut self, name: &str) -> Option<&mut Dentry> {
        self.0.get_mut(name)
    }
    pub fn remove(&mut self, name: &str) -> Result<(), DentryMapErr> {
        let status = self.0.remove(name);
        match status {
            Some(_) => Ok(()),
            None => Err(DentryMapErr::FileNotExist)
        }
    }

    pub fn as_ref(&self) -> &Self {
        self
    }
    pub fn as_mut(&mut self) -> &mut Self {
        self
    }
}


#[derive(Debug)]
pub enum DentryMapErr {
    InvalidName,
    FileExist,
    FileNotExist,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum DentryFiletype {
    Unknown = 0,
    RegularFile = 1,
    DirecotryFile = 2,
    CharacterDevice = 3,
    BlockDevice = 4,
    BufferFile = 5,
    SocketFile = 6,
    SymbolicLink = 7,
}

impl From<u8> for DentryFiletype {
    fn from(value: u8) -> Self {
        match value {
            1 => DentryFiletype::RegularFile,
            2 => DentryFiletype::DirecotryFile,
            3 => DentryFiletype::CharacterDevice,
            4 => DentryFiletype::BlockDevice,
            5 => DentryFiletype::BufferFile,
            6 => DentryFiletype::SocketFile,
            7 => DentryFiletype::SymbolicLink,
            _ => DentryFiletype::Unknown,
        }
    }
}
