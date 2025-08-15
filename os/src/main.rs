#![no_std]
#![no_main]

use core::arch::global_asm;
use log::*;

mod lang_items;
mod sbi;
#[macro_use]
mod console;
pub mod batch;
mod logging;
mod sync;
pub mod syscall;
pub mod trap;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));


/// ckear bss segment
fn clear_bss() {
    unsafe extern "C" {
        safe fn sbss();
        safe fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe{(a as *mut u8).write_volatile(0)}
    });

}


#[unsafe(no_mangle)]
pub fn rust_main() -> ! {
    unsafe extern "C" {
        safe fn stext();
        safe fn etext();
        safe fn srodata();
        safe fn erodata();
        safe fn sdata();
        safe fn edata();
        safe fn sbss();
        safe fn ebss();
        safe fn boot_stack_lower_bound();
        safe fn boot_stack_top();
    }
    clear_bss();
    logging::init();
    println!("[kernel] hello world");
    trace!("[kernel] .text [{:#x}, {:#x}]", stext as usize, etext as usize);
    debug!("[kernel] .rodata [{:#x}, {:#x}]", srodata as usize, erodata as usize);
    info!("[kernel] .data [{:#x}, {:#x}]", sdata as usize, edata as usize);
    warn!("[kernel] boot_stack top=bottom={:#x} lower_bound= {:#x}", boot_stack_top as usize, boot_stack_lower_bound as usize);
    error!("[kernel] .bss [{:#x}, {:#x}]", sbss as usize, ebss as usize);
    trap::init();
    batch::init();
    batch::run_next_app();
}
