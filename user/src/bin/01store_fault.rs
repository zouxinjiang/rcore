#![no_std]
#![no_main]

#[macro_use]
extern crate user;

#[unsafe(no_mangle)]
fn main() -> i32 {
    println!("info test store fault, we will insert an invalid store operation...");
    println!("kernel should kill this app!");
    unsafe {
        core::ptr::null_mut::<u8>().write_volatile(0);
    }
    0
}
