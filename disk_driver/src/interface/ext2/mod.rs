pub mod bitmap;
pub mod directory;
pub mod group;
pub mod group_descriptor;
pub mod inode;
pub mod status;
pub mod superblock;

pub const BLOCK_SIZE: usize = 1024;
pub const BOOTSECTOR_SIZE: usize = 1024;
pub const SUPERBLOCK_SIZE: usize = BLOCK_SIZE;
pub const GROUPDESCRIPTOR_SIZE: usize = 32;
pub const INODE_INDEX_START: usize = 1;
pub const ROOT_INODE_INDEX: usize = 2;

use core::sync::atomic::{AtomicUsize, Ordering::Relaxed};
use std::collections::HashMap;

pub use bitmap::Bitmap;
pub use directory::{Dentry, DentryMap};
pub use group::{Group, GroupTable};
pub use group_descriptor::{BlockGroupDescriptor, BlockGroupDescriptorTable};
pub use inode::{Inode, InodeTable};
pub use superblock::SuperBlock;

#[allow(unused)]
use super::vector_wrap;
use super::{FileSystem, FileID};
#[allow(unused)]
use super::{
    little_endian_wrap_16, little_endian_wrap_32, little_endian_wrap_64, little_endian_wrap_8,
};
use crate::Disk;
use crate::interface::ext2::directory::DentryFiletype;
use crate::{little_endian, vector};

pub struct Ext2 {
    disk: Disk, // remove this in the future, which will be wrap when new instance
    pub boot_sector: [u8; BOOTSECTOR_SIZE],
    pub superblock: SuperBlock,
    pub groupdescriptortable: BlockGroupDescriptorTable,
    pub grouptable: GroupTable,
    pub dentrymap: DentryMap,
}

impl Ext2 {
    pub fn new(disk: Disk) -> Self {
        Self {
            disk,
            boot_sector: [0; BOOTSECTOR_SIZE],
            superblock: SuperBlock::default(),
            groupdescriptortable: BlockGroupDescriptorTable::new(),
            grouptable: GroupTable::new(),
            dentrymap: DentryMap::new(),
        }
    }

    pub fn read_bootsector(&mut self) {
        for (i, &byte) in self.disk[0..BOOTSECTOR_SIZE].iter().enumerate() {
            self.boot_sector[i] = byte;
        }
    }

    pub fn read_superblock(&mut self) {
        // println!("Read superblock");
        let base = BOOTSECTOR_SIZE;

        let superblock = SuperBlock::new(
            little_endian!(self.disk, base, 4),
            little_endian!(self.disk, base + 4, 4),
            little_endian!(self.disk, base + 8, 4),
            little_endian!(self.disk, base + 12, 4),
            little_endian!(self.disk, base + 16, 4),
            little_endian!(self.disk, base + 20, 4),
            little_endian!(self.disk, base + 24, 4),
            little_endian!(self.disk, base + 28, 4),
            little_endian!(self.disk, base + 32, 4),
            little_endian!(self.disk, base + 36, 4),
            little_endian!(self.disk, base + 40, 4),
            little_endian!(self.disk, base + 44, 4),
            little_endian!(self.disk, base + 48, 4),
            little_endian!(self.disk, base + 52, 2),
            little_endian!(self.disk, base + 54, 2),
            little_endian!(self.disk, base + 56, 2),
            little_endian!(self.disk, base + 58, 2),
            little_endian!(self.disk, base + 60, 2),
            little_endian!(self.disk, base + 62, 2),
            little_endian!(self.disk, base + 64, 4),
            little_endian!(self.disk, base + 68, 4),
            little_endian!(self.disk, base + 72, 4),
            little_endian!(self.disk, base + 76, 4),
            little_endian!(self.disk, base + 80, 2),
            little_endian!(self.disk, base + 82, 2),
            little_endian!(self.disk, base + 84, 4),
            little_endian!(self.disk, base + 88, 2),
            little_endian!(self.disk, base + 90, 2),
            little_endian!(self.disk, base + 92, 4),
            little_endian!(self.disk, base + 96, 4),
            little_endian!(self.disk, base + 100, 4),
            vector!(self.disk, base + 104, 16),
            vector!(self.disk, base + 120, 16),
            vector!(self.disk, base + 136, 64),
            little_endian!(self.disk, base + 200, 4),
            little_endian!(self.disk, base + 204, 1),
            little_endian!(self.disk, base + 205, 1),
            little_endian!(self.disk, base + 206, 2),
            vector!(self.disk, base + 208, 16),
            little_endian!(self.disk, base + 224, 4),
            little_endian!(self.disk, base + 228, 4),
            little_endian!(self.disk, base + 232, 4),
            [
                little_endian!(self.disk, base + 236, 4),
                little_endian!(self.disk, base + 240, 4),
                little_endian!(self.disk, base + 244, 4),
                little_endian!(self.disk, base + 248, 4),
            ],
            little_endian!(self.disk, base + 252, 1),
            vector!(self.disk, base + 253, 3),
            little_endian!(self.disk, base + 256, 4),
            little_endian!(self.disk, base + 260, 4),
            vector!(self.disk, base + 264, 760),
        );
        self.superblock = superblock;
    }

    pub fn read_groupdescriptortable(&mut self) {
        let base = BOOTSECTOR_SIZE + SUPERBLOCK_SIZE;
        // let first_block = self.superblock.first_data_block;
        // let end_block = little_endian!(self.disk, base, 4);
        // println!("Firstblock: {:?}", first_block);
        // println!("Endblock: {:?}", end_block);

        // let gd_nums = (end_block - first_block) as usize * BLOCK_SIZE / GROUPDESCRIPTOR_SIZE;
        let gd_nums =
            (self.superblock.blocks_count / self.superblock.blocks_per_group) as usize + 1;
        for i in 0..gd_nums {
            let gd = BlockGroupDescriptor::new(
                little_endian!(self.disk, base + i * GROUPDESCRIPTOR_SIZE + 0, 4),
                little_endian!(self.disk, base + i * GROUPDESCRIPTOR_SIZE + 4, 4),
                little_endian!(self.disk, base + i * GROUPDESCRIPTOR_SIZE + 8, 4),
                little_endian!(self.disk, base + i * GROUPDESCRIPTOR_SIZE + 12, 2),
                little_endian!(self.disk, base + i * GROUPDESCRIPTOR_SIZE + 14, 2),
                little_endian!(self.disk, base + i * GROUPDESCRIPTOR_SIZE + 16, 2),
                little_endian!(self.disk, base + i * GROUPDESCRIPTOR_SIZE + 18, 2),
                vector!(self.disk, base + i * GROUPDESCRIPTOR_SIZE + 20, 12),
            );
            self.groupdescriptortable.append(gd);
        }
    }

    pub fn load_group(&mut self) {
        for gd in self.groupdescriptortable.into_iter() {
            self.grouptable.append(Group {
                block_bitmap: Bitmap::from(vector!(
                    self.disk,
                    gd.block_bitmap as usize * BLOCK_SIZE,
                    BLOCK_SIZE
                )),
                inode_bitmap: Bitmap::from(vector!(
                    self.disk,
                    gd.inode_bitmap as usize * BLOCK_SIZE,
                    BLOCK_SIZE
                )),
                inode_table: self.load_inode(gd),
            })
        }

        // for group in self.grouptable.into_iter() {
        //     println!("group: {:?}", group);
        // }
    }

    fn load_inode(&self, gd: &BlockGroupDescriptor) -> InodeTable {
        let base = gd.inode_table as usize * BLOCK_SIZE;
        let inode_size = self.superblock.inode_size as usize;
        // let inode_nums = (self.superblock.inodes_per_group - gd.free_inodes_count as u32) as usize;
        let inode_nums = self.superblock.inodes_per_group as usize;

        let mut indoe_table = InodeTable::new();
        indoe_table.append(Inode::default());  // insert a empty inode

        for i in 0..inode_nums {
            let inode = Inode::new(
                little_endian!(self.disk, base + i * inode_size + 0, 2),
                little_endian!(self.disk, base + i * inode_size + 2, 2),
                little_endian!(self.disk, base + i * inode_size + 4, 4),
                little_endian!(self.disk, base + i * inode_size + 8, 4),
                little_endian!(self.disk, base + i * inode_size + 12, 4),
                little_endian!(self.disk, base + i * inode_size + 16, 4),
                little_endian!(self.disk, base + i * inode_size + 20, 4),
                little_endian!(self.disk, base + i * inode_size + 24, 2),
                little_endian!(self.disk, base + i * inode_size + 26, 2),
                little_endian!(self.disk, base + i * inode_size + 28, 4),
                little_endian!(self.disk, base + i * inode_size + 32, 4),
                little_endian!(self.disk, base + i * inode_size + 36, 4),
                [
                    little_endian!(self.disk, base + i * inode_size + 40, 4),
                    little_endian!(self.disk, base + i * inode_size + 44, 4),
                    little_endian!(self.disk, base + i * inode_size + 48, 4),
                    little_endian!(self.disk, base + i * inode_size + 52, 4),
                    little_endian!(self.disk, base + i * inode_size + 56, 4),
                    little_endian!(self.disk, base + i * inode_size + 60, 4),
                    little_endian!(self.disk, base + i * inode_size + 64, 4),
                    little_endian!(self.disk, base + i * inode_size + 68, 4),
                    little_endian!(self.disk, base + i * inode_size + 72, 4),
                    little_endian!(self.disk, base + i * inode_size + 76, 4),
                    little_endian!(self.disk, base + i * inode_size + 80, 4),
                    little_endian!(self.disk, base + i * inode_size + 84, 4),
                    little_endian!(self.disk, base + i * inode_size + 88, 4),
                    little_endian!(self.disk, base + i * inode_size + 92, 4),
                    little_endian!(self.disk, base + i * inode_size + 96, 4),
                ],
                little_endian!(self.disk, base + i * inode_size + 100, 4),
                little_endian!(self.disk, base + i * inode_size + 104, 4),
                little_endian!(self.disk, base + i * inode_size + 108, 4),
                little_endian!(self.disk, base + i * inode_size + 112, 4),
                vector!(self.disk, base + i * inode_size + 116, 12),
            );
            indoe_table.append(inode);
        }
        indoe_table
    }

    pub fn load_root_dentry(&mut self) {
        let inode = self.grouptable.get(0).unwrap().inode_table.get(2).unwrap();
        let mut data = Vec::new();
        for block_num in inode.block {
            if block_num==0 {continue;}
            let tmp = self.cursor(block_num as usize, BLOCK_SIZE);
            for &value in tmp {
                data.push(value);
            }
        }
        self.load_dentrymap(None, &data);
    }

    pub fn load_dentrymap(&mut self, root: Option<&mut Dentry>, data: &Vec<u8>) {
        // crate::format_print(data);
        let mut base = 0;
        // let mut dentrymap = Some(self.dentrymap);
        // if root!="/" {
        //     if let Ok(paths) = self.parse_path(root) {
        //         for p in paths.iter() {
        //             if p.len()==0 {continue;}
        //             if let Some(next) = dentrymap.as_ref().unwrap().get(p) {
        //                 dentrymap = next.dentrymap;
        //             }
        //         }
        //     }
        // }

        let mut dentrymap = if root.is_none() {
            Some(&mut self.dentrymap)
        } else {
            root.unwrap().dentrymap.as_mut()
        };

        while base < data.len() {
            // read inode index
            let inode_index = little_endian!(data, base + 0, 4);
            let rec_len = little_endian!(data, base + 4, 2);
            let name_len = little_endian!(data, base + 6, 1);
            let file_type = little_endian!(data, base + 7, 1);
            let name = String::from_utf8(data[base + 8..base + 8 + name_len as usize].to_vec())
                .unwrap_or_default();

            let dentry = Dentry::new(inode_index, rec_len, name_len, file_type, name);
            base += rec_len as usize;

            // println!("dentry: {:#?}", dentry);

            match dentrymap.as_mut().unwrap().append(dentry) {
                Ok(()) => {},
                Err(err) => println!("[Error] {:?}", err),
            }
        }
    }

    pub fn get_data(&self, idx_group: usize, idx_inode: usize) -> Vec<u8> {
        let inode = self.grouptable.get(idx_group).unwrap().inode_table.get(idx_inode).unwrap();
        let mut buffer = Vec::new();
        for b in inode.block {
            if b==0 {continue;}
            let base = b as usize * BLOCK_SIZE;
            for &byte in self.disk[base..base+BLOCK_SIZE].iter() {
                buffer.push(byte);
            }
        }
        buffer
    }

    pub fn cursor(&self, block_num: usize, data_size: usize) -> &[u8] {
        // SuperBlock::rev_level / minor_rev_level => format version
        // data size: Inode::blocks (512 bytes base) * blocksize = Inode::blocks * 512
        let base = block_num * BLOCK_SIZE;
        &self.disk[base..base + data_size]
    }

    pub fn test(&self) {
        let tmp = vector!(self.disk, 1024, 4);
        println!("tmp: {:?}", tmp);
    }
}

pub struct VFS {
    disk: Ext2,
    map: HashMap<FileID, Dentry>
}

impl VFS {
    pub fn new(disk: Ext2) -> Self {
        Self { disk, map: HashMap::new() }
    }
}


impl FileSystem for VFS {
    fn open(&mut self, path: &str) -> Option<FileID> {
        static FILE_ID: AtomicUsize = AtomicUsize::new(0);
        let fid = FileID(FILE_ID.fetch_add(1, Relaxed));

        self.list_dir(path);
        println!("{:#?}", self.disk.dentrymap);
        if let Ok(paths) = self.parse_path(path) {

        } else {
            println!("[Error] Path not exist: {:?}", path);
            return None;
        }

        Some(fid)
    }

    fn read(&self, file_id: FileID) -> &[u8] {
        todo!()
    }

    fn write(&mut self, file_id: FileID, ctx: &[u8]) {
        todo!()
    }

    fn create(&mut self, path: &str) {
        todo!()
    }

    fn list_dir(&mut self, path: &str) {
        let paths = self.parse_path(path);
        match paths {
            Err(e) => println!("[Error]: {:?}", e),
            Ok(paths) => {
                // println!("Path: {:?}", val);
                // let root_inode = self.grouptable.get(0).unwrap().inode_table.get(2).unwrap();
                // println!("Table: {:?}", root_inode);
                if paths[0] == "/" {
                    let mut dentrymap = self.disk.dentrymap.clone();
                    for p in paths[1..].iter() {
                        if p.len()==0 {continue;}
                        if let Some(dentry) = dentrymap.get_mut(p) {
                            // if dentry.dentrymap.is_some() {
                            //     dentrymap = dentry.dentrymap.as_ref().unwrap().clone();
                            // } else {
                            //     println!("[Error]: Path not found: {:?}", path);
                            //     return;
                            // }
                            let inode_filetype = &dentry.file_type;
                            if inode_filetype==&DentryFiletype::DirecotryFile {
                                let data = self.disk.get_data(0, dentry.inode_index as usize);
                                self.disk.load_dentrymap(Some(dentry), &data)
                            }
                        } else {
                            println!("[Error]: Entry not found: {:?}", path);
                            return;
                        }
                    }
                    // println!("dentrymap: {:#?}", dentrymap);
                }
            }
        }
    }
}
