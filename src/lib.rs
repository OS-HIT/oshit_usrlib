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
    clear_bss();
    sys_exit(main());
    panic!("unreachable after sys_exit!");
}

#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    panic!("Cannot find main!");
}

fn clear_bss() {
    extern "C" {
        fn start_bss();
        fn end_bss();
    }
    unsafe{
        for addr in start_bss as usize..end_bss as usize {
            (addr as *mut u8).write_volatile(0); 
        }
    }
}