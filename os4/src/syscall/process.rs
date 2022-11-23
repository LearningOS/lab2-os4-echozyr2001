//! Process management syscalls

use crate::config::MAX_SYSCALL_NUM;
use crate::mm::PageTable;
use crate::mm::{PhysAddr, VirtAddr};
use crate::task::{
    current_user_token, exit_current_and_run_next, get_task_info, suspend_current_and_run_next,
    TaskStatus,
};
use crate::timer::get_time_us;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

#[derive(Clone, Copy)]
pub struct TaskInfo {
    pub status: TaskStatus,
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    pub time: usize,
}

pub fn sys_exit(exit_code: i32) -> ! {
    info!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

// YOUR JOB: 引入虚地址后重写 sys_get_time
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    let virt_addr = VirtAddr(_ts as usize);
    if let Some(phys_addr) = vaddr2paddr(virt_addr) {
        let _us = get_time_us();
        let kernel_ts = phys_addr.0 as *mut TimeVal;
        unsafe {
            *kernel_ts = TimeVal {
                sec: _us / 1_000_000,
                usec: _us % 1_000_000,
            };
        }
        0
    } else {
        -1
    }
}

fn vaddr2paddr(virt_addr: VirtAddr) -> Option<PhysAddr> {
    let offset = virt_addr.page_offset();
    let vpn = virt_addr.floor();
    let ppn = PageTable::from_token(current_user_token())
        .translate(vpn)
        .map(|entry| entry.ppn()); // 查表
    if let Some(ppn) = ppn {
        Some(PhysAddr::combine(ppn, offset))
    } else {
        println!("Vaddr2Paddr() fail!");
        None
    }
}

// CLUE: 从 ch4 开始不再对调度算法进行测试~
pub fn sys_set_priority(_prio: isize) -> isize {
    -1
}

// YOUR JOB: 扩展内核以实现 sys_mmap 和 sys_munmap
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    -1
}

pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    -1
}

// YOUR JOB: 引入虚地址后重写 sys_task_info
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    if let Some(phys_addr) = vaddr2paddr(VirtAddr(ti as usize)) {
        let ti = ti as ti = get_task_info();
        0
    } else {
        -1
    }
}