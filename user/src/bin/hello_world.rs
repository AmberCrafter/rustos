#![no_std]
#![no_main]

#[macro_use]
extern crate user;

#[no_mangle]
unsafe fn main() -> i32 {
    println!("Hello world!");
    0
}