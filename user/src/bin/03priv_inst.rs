
#![no_std]
#![no_main]

#[macro_use]
extern crate user;

use core::arch::asm;

#[unsafe(no_mangle)]
fn main() -> i32 {
    println!("try to execute privileged instruction in U mode");
    println!("kernel should kill this app!");
    unsafe {
        asm!("sret");
    }
    0
}
