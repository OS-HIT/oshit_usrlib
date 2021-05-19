#![no_std]
#![feature(asm)]
#![feature(linkage)]
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod usr_panic;
mod syscalls;

pub use syscalls::{
    sys_write,
    sys_read, 
    sys_exit, 
    sys_yield,
    sys_uname,
    sys_time,
    sys_exec,
    sys_fork,
    sys_waitpid,
    sys_pipe,
    sys_close,
    read,
    wait,
    UTSName,
    TMS,
    FD_STDIN,
    FD_STDOUT
};
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