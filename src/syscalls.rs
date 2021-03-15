pub const SYSCALL_WRITE: usize = 64;
pub const SYSCALL_EXIT: usize = 93;
pub const SYSCALL_YIELD: usize = 124;

fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: usize = args[0];
    unsafe {
        asm!(
            "ecall",
            inout("a0") ret,
            in("a1") args[1],
            in("a2") args[2],
            in("a7") id
        )
    }
    ret as isize
}

pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

pub fn sys_exit(exit_code: i32) -> isize {
    syscall(SYSCALL_EXIT, [exit_code as usize, 0, 0])
}

pub fn sys_yield() -> isize {
    syscall(SYSCALL_YIELD, [0, 0, 0])
}