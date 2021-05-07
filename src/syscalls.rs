#![allow(dead_code)]
pub const SYSCALL_GETCWD        : usize = 17;
pub const SYSCALL_DUP           : usize = 23;
pub const SYSCALL_DUP3          : usize = 24;
pub const SYSCALL_MKDIRAT       : usize = 34;
pub const SYSCALL_UNLINKAT      : usize = 35;
pub const SYSCALL_LINKAT        : usize = 37;
pub const SYSCALL_UMOUNT2       : usize = 39;
pub const SYSCALL_MOUNT         : usize = 40;
pub const SYSCALL_CHDIR         : usize = 49;
pub const SYSCALL_OPENAT        : usize = 56;
pub const SYSCALL_CLOSE         : usize = 57;
pub const SYSCALL_PIPE2         : usize = 59;
pub const SYSCALL_GETDENTS64    : usize = 61;
pub const SYSCALL_READ          : usize = 63;
pub const SYSCALL_WRITE         : usize = 64;
pub const SYSCALL_FSTAT         : usize = 80;
pub const SYSCALL_EXIT          : usize = 93;
pub const SYSCALL_NANOSLEEP     : usize = 101;
pub const SYSCALL_SCHED_YIELD   : usize = 124;
pub const SYSCALL_TIMES         : usize = 153;
pub const SYSCALL_UNAME         : usize = 160;
pub const SYSCALL_GETTIMEOFDAY  : usize = 169;
pub const SYSCALL_GETPID        : usize = 172;
pub const SYSCALL_GETPPID       : usize = 173;
pub const SYSCALL_BRK           : usize = 214;
pub const SYSCALL_MUNMAP        : usize = 215;
pub const SYSCALL_CLONE         : usize = 220;
pub const SYSCALL_FORK          : usize = 220;
pub const SYSCALL_EXECVE        : usize = 221;
pub const SYSCALL_EXEC          : usize = 221;
pub const SYSCALL_MMAP          : usize = 222;
pub const SYSCALL_WAIT4         : usize = 260;
pub const SYSCALL_WAITPID       : usize = 260;

pub const FD_STDIN              : usize = 0;
pub const FD_STDOUT             : usize = 1;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct TMS {
    pub tms_utime: u64,
    pub tms_stime: u64,
    pub tms_cutime: u64,
    pub tms_cstime: u64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct UTSName {
    pub sysname     : *const u8,
    pub nodename    : *const u8,
    pub release     : *const u8,
    pub version     : *const u8,
    pub machine     : *const u8,
    pub domainname  : *const u8,
}

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
    syscall(SYSCALL_SCHED_YIELD, [0, 0, 0])
}

pub fn sys_time(tms: *mut TMS) -> isize {
    syscall(SYSCALL_TIMES, [tms as usize, 0, 0])
}

pub fn sys_uname(uts: *mut UTSName) -> isize {
    syscall(SYSCALL_UNAME, [uts as usize, 0, 0])
}

pub fn sys_fork() -> isize {
    syscall(SYSCALL_FORK, [0, 0, 0])
}

pub fn sys_exec(app_name: *const u8) -> isize{
    syscall(SYSCALL_EXEC, [app_name as usize, 0, 0])
}

pub fn sys_waitpid(pid: isize, exit_code_ptr: *mut i32) -> isize{
    syscall(SYSCALL_WAITPID, [pid as usize, exit_code_ptr as usize, 0])
}

pub fn sys_read(fd: usize, buf: *const u8, len: usize) -> isize {
    syscall(SYSCALL_READ, [fd, buf as usize, len])
}

pub fn getbyte() -> u8 {
    let buf: [u8; 1] = [0];
    sys_read(FD_STDIN, &buf[0], 1);
    return buf[0];
}