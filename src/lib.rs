#![no_std]
#![feature(asm)]
#![feature(linkage)]
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod usr_panic;
mod syscalls;

pub use syscalls::{sys_write, sys_exit, sys_yield};
pub use console::{print};

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    sys_exit(main());
    panic!("unreachable after sys_exit!");
}

#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    panic!("Cannot find main!");
}