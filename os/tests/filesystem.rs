#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustos::library::unittest::test_runner)]
#![reexport_test_harness_main = "test_main"]

#![feature(alloc_error_handler)]

extern crate alloc;

use alloc::collections::BTreeMap;
use bootloader::{entry_point, BootInfo};
use rustos::library::syscall::error::Errno;
// use spin::Mutex;
use core::panic::PanicInfo;

use rustos;
#[allow(unused)]
use rustos::{print, println};
#[allow(unused)]
use rustos::{serial_print, serial_println};


use alloc::boxed::Box;
use rustos::library::filesystem::FileSystem;
use rustos::library::filesystem::vfs::test_fs::EmptyFileSystem;
use rustos::library::filesystem::vfs::Vfs;
use rustos::library::filesystem::flags::{MountFlags, OpenFlags, Mode};
use rustos::library::filesystem::FsId;


entry_point!(main);
pub fn main(boot_info: &'static mut BootInfo) -> ! {
    rustos::init(boot_info);
    serial_println!("Hello, this is tests::filesystem");
    test_main();
    rustos::hlt_loop()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rustos::library::handler_panic::kernel_panic::panic_handler(info)
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) ->! {
    rustos::library::handler_panic::kernel_panic::alloc_error_handler(layout)
}

// test case
#[test_case]
fn test_initialize_called() {
    let fs = Box::new(EmptyFileSystem::from(7.into()));

    let mut vfs = Vfs::new(FsId::from(0));
    assert_eq!(0, vfs.mount_count());

    assert!(vfs.mount("/", fs, MountFlags::NONE).is_ok());
}

#[test_case]
fn test_mount_unmount() {
    let fs = Box::new(EmptyFileSystem::from(19.into()));
    let vfs = Vfs::new(FsId::from(0));
    assert_eq!(0, vfs.mount_count());

    assert!(vfs.mount("/", fs, MountFlags::NONE).is_ok());
    assert_eq!(1, vfs.mount_count());

    assert!(vfs.unmount("/").is_ok());
    assert_eq!(0, vfs.mount_count());

    assert_eq!(Err(Errno::EINVAL), vfs.unmount("/"));
    assert_eq!(0, vfs.mount_count());
}

#[test_case]
fn test_unmount_wrong_dir() {
    let fs = Box::new(EmptyFileSystem::from(19.into()));
    let vfs = Vfs::new(FsId::from(0));
    assert_eq!(0, vfs.mount_count());

    assert!(vfs.mount("/", fs, MountFlags::NONE).is_ok());
    assert_eq!(1, vfs.mount_count());

    assert_eq!(Err(Errno::EINVAL), vfs.unmount("/foobar"));
    assert_eq!(1, vfs.mount_count());

    assert!(vfs.unmount("/").is_ok());
    assert_eq!(0, vfs.mount_count());

    assert_eq!(Err(Errno::EINVAL), vfs.unmount("/"));
    assert_eq!(0, vfs.mount_count());
}

#[test_case]
fn test_mount_and_open() {
    let fs = Box::new(EmptyFileSystem::from(2.into()));
    let vfs = Vfs::new(FsId::from(11));

    assert!(vfs.mount("/", fs, MountFlags::NONE).is_ok());

    let f_res = vfs.open("/foo/bar.file", Mode::from(0), OpenFlags::O_RDWR);
    assert!(f_res.is_ok());

    let f = f_res.unwrap();
    assert_eq!("/foo/bar.file", f.absolute_path());
}