#![no_std]
#![feature(asm)]
#![feature(linkage)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

extern crate alloc;

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

use buddy_system_allocator::LockedHeap;
use alloc::vec::Vec;

const USER_HEAP_SIZE: usize = 32768;
static mut HEAP_SPACE: [u8; USER_HEAP_SIZE] = [0; USER_HEAP_SIZE];
#[global_allocator]
static HEAP: LockedHeap = LockedHeap::empty();

#[alloc_error_handler]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Heap allocation error, layout = {:?}", layout);
}

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start(argc: usize, argv: usize, envp: usize) -> ! {
    unsafe {
        HEAP.lock()
            .init(HEAP_SPACE.as_ptr() as usize, USER_HEAP_SIZE);
    }
    let mut args: Vec<&'static str> = Vec::new();
    for i in 0..argc {
        let str_start = unsafe {
            ((argv + i * core::mem::size_of::<usize>()) as *const usize).read_volatile()
        };
        let len = (0usize..).find(|i| unsafe {
            ((str_start + *i) as *const u8).read_volatile() == 0
        }).unwrap();

        args.push(
            core::str::from_utf8(unsafe {
                core::slice::from_raw_parts(str_start as *const u8, len)
            }).unwrap()
        );
    }

    let mut envs: Vec<&'static str> = Vec::new();
    for i in 0.. {
        let str_start = unsafe {
            ((envp + i * core::mem::size_of::<usize>()) as *const usize).read_volatile()
        };
        if str_start == 0 {break;};
        let len = (0usize..).find(|i| unsafe {
            ((str_start + *i) as *const u8).read_volatile() == 0
        }).unwrap();
        envs.push(
            core::str::from_utf8(unsafe {
                core::slice::from_raw_parts(str_start as *const u8, len)
            }).unwrap()
        );
    }
    sys_exit(main(argc, args.as_slice(), envs.as_slice()));
    panic!("unreachable after sys_exit!");
}

#[linkage = "weak"]
#[no_mangle]
fn main(_: usize, _: &[&str], _: &[&str]) -> i32 {
    panic!("Cannot find main!");
}