use std::io::{Write, Seek};
use std::{fs::File, io::Read};

use anyhow::Result;
use disk_driver::format_print;
use disk_driver::{
    interface::{
        ext2::{Ext2, VFS},
        FileSystem,
    },
    Disk,
};

use byteorder::{ByteOrder, LittleEndian};

fn main() -> Result<()> {
    let path = "disk_driver/ext2fs_01/disk.img";
    // let mut f = File::open(path)?;
    let mut f = File::options().read(true).write(true).open(path)?;

    let mut disk: Disk = vec![0; 1024 * 1024];
    f.read_exact(&mut disk);

    let mut fs = Ext2::new(disk);
    // fs.read_bootsector();
    fs.read_superblock();
    fs.read_groupdescriptortable();
    fs.load_group();
    fs.load_root_dentry();

    // fs.test();

    // println!("fs sb: {:?}", fs.superblock);
    // println!("fs gdt: {:#?}", fs.groupdescriptortable);
    // println!("fs gt: {:?}", fs.grouptable);
    // println!("fs it: {:?}", fs.grouptable.get(0).unwrap().inode_table.get(12).unwrap());

    // // test file structure
    // let nums = 16;
    // for i in 0..nums {
    //     println!("fs inode {:02x}: {:?}", i+1, fs.grouptable.get(0).unwrap().inode_table.get(i).unwrap());
    // }

    // // show directory
    // let block_num = 24;
    // println!("\n block number: {:}", block_num);
    // let cur = fs.cursor(block_num, 2 * 512);
    // format_print(cur);

    // let block_num = 40;
    // println!("\n block number: {:}", block_num);
    // let cur = fs.cursor(block_num, 2 * 512);
    // format_print(cur);

    // println!("Root: {:#?}", fs.dentrymap);

    let mut vfs = VFS::new(fs);

    // test filesystem
    // vfs.list_dir("./foo/bar.txt");
    // vfs.list_dir("/foo/bar.txt");
    // vfs.list_dir("/lost+found/");
    // vfs.list_dir("/folder1");

    let fid = vfs.open("/folder1/file1_1.txt").expect("File not exist");
    println!("{:?}", fid);
    let data = vfs.read(fid);
    println!("{:?}", data);
    println!("{:?}", String::from_utf8(data).unwrap());

    // let mut ctx = [0_u8; 1024];
    // // [102, 105, 108, 101, 32, 49, 45, 49, 32, 99, 111, 110, 116, 101, 120, 116, 46, 10]
    // for (i, &byte) in "Write by my vfs.\nHello world.\n".as_bytes().iter().enumerate() {
    // // for (i, &byte) in [102, 105, 108, 101, 32, 49, 45, 49, 32, 99, 111, 110, 116, 101, 120, 116, 46, 10].iter().enumerate() {
    //     ctx[i] = byte;
    // }
    // // vfs.disk.write(40, ctx);
    let ctx = "Write by my vfs.\nHello world.\n".as_bytes();

    vfs.write(fid, ctx);

    let data = vfs.read(fid);
    println!("{:?}", data);
    println!("{:?}", String::from_utf8(data).unwrap());

    // vfs.disk.alloc_block(0);

    // println!("Root: {:#?}", vfs.disk.dentrymap);
    // println!("Root: {:#?}", vfs.map);
    // println!("Hello, world!");

    let tmp = vfs.disk.get_inode(0, 14).unwrap().borrow().clone();
    let tmp: [u8; 128] = tmp.into();
    println!("{:?}", tmp);


    flsuh_disk(&mut f, &vfs);


    // let val = 123456789_u32;
    // let mut buf = [0; 4];
    // LittleEndian::write_u32(&mut buf, val);

    // println!("u32: {:x?}", buf);

    Ok(())
}

// fn format_print(data: &[u8]) {
//     for (i, v) in data.iter().enumerate() {
//         if i%16 == 0 {println!();}
//         print!("{:02x} ", v);
//     }
//     println!();
// }


fn flsuh_disk(f: &mut File, vfs: &VFS) {
    f.rewind();
    let res = f.write(vfs.disk.as_slice());
}