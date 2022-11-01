use core::arch::global_asm;

use alloc::vec::Vec;
use spin::Lazy;

static APP_NAMES: Lazy<Vec<&'static str>> = Lazy::new(|| {
    let num_app = get_app_num();
    extern "C" {
        fn _app_names();
    }
    let mut start = _app_names as usize as *const u8;
    let mut v = Vec::new();
    unsafe {
        for _ in 0..num_app {
            let mut end = start;
            while end.read_volatile() != '\0' as u8 {
                end = end.add(1);
            }
            let slice = core::slice::from_raw_parts(start, end as usize - start as usize);
            let tmp = core::str::from_utf8(slice).unwrap();
            v.push(tmp);
            start = end.add(1);
        }
    }
    v
});

pub fn get_app_num() -> usize {
    extern "C" {
        fn _num_app();
    }
    unsafe { (_num_app as usize as *const usize).read_volatile() }
}

pub fn get_app_data(app_id: usize) -> &'static [u8] {
    extern "C" {
        fn _num_app();
    }
    let num_app_ptr = _num_app as usize as *const usize;
    let num_app = get_app_num();
    let app_start = unsafe { core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1) };
    assert!(app_id < num_app);
    unsafe {
        core::slice::from_raw_parts(
            app_start[app_id] as *const u8,
            app_start[app_id + 1] - app_start[app_id],
        )
    }
}

pub fn get_app_data_by_name(name: &str) -> Option<&'static [u8]> {
    let num_app = get_app_num();
    (0..num_app)
        .find(|&i| APP_NAMES[i] == name)
        .map(|i| get_app_data(i))
}

pub fn list_app() {
    serial_println!("=============================");
    serial_println!("APPS:");
    for app in APP_NAMES.iter() {
        serial_println!("{}", app);
    }
    serial_println!("=============================");
}
