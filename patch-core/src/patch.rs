use std::vec;

use mach2::boolean::boolean_t;
use mach2::kern_return::KERN_SUCCESS;
use mach2::message::mach_msg_type_number_t;
use mach2::port::{mach_port_name_t, mach_port_t};
use mach2::task::{task_resume, task_suspend};
use mach2::traps::{mach_task_self, task_for_pid};
use mach2::vm::{mach_vm_protect, mach_vm_read_overwrite, mach_vm_region, mach_vm_write};
use mach2::vm_prot::{vm_prot_t, VM_PROT_COPY, VM_PROT_EXECUTE, VM_PROT_READ, VM_PROT_WRITE};
use mach2::vm_region::{vm_region_basic_info, VM_REGION_BASIC_INFO_64};
use mach2::vm_types::{mach_vm_address_t, mach_vm_size_t, vm_address_t, vm_offset_t, vm_size_t};
pub struct Patch {
    task: mach_port_name_t,
    protection_addr: mach_vm_address_t,
    protection_size: mach_vm_size_t,
    protection: vm_prot_t,
    base_addr: vm_address_t,
    offset: vm_address_t,
    data: Vec<u8>,
    pub pid: i32,
}

impl Patch {
    pub fn from_name(name: &str) -> Option<Self> {
        match Self::get_pid(name) {
            None => None,
            Some(pid) => Self::new(pid),
        }
    }
    pub fn new(pid: i32) -> Option<Self> {
        let task = unsafe {
            let mut task: mach_port_name_t = 0;
            let res = task_for_pid(mach_task_self(), pid, &mut task);
            if res != KERN_SUCCESS {
                println!("task_for pid error {}", res);
                return None;
            }
            task
        };
        let mut own = Self {
            data: vec![],
            pid,
            offset: 0,
            task,
            protection_addr: 0,
            protection_size: 0,
            protection: 0,
            base_addr: 0,
        };
        own.get_base_address();
        Some(own)
    }
    pub fn get_pid(name: &str) -> Option<i32> {
        use sysinfo::System;
        // Please note that we use "new_all" to ensure that all lists of
        // CPUs and processes are filled!
        let mut sys = System::new_all();
        // First we update all information of our `System` struct.
        sys.refresh_all();
        // Display processes ID, name na disk usage:
        for (pid, process) in sys.processes() {
            let _path = process.exe().unwrap().to_str().unwrap();
            if process.name() == name {
                return Some(pid.as_u32() as i32);
            }
        }
        None
    }
    pub fn write(&mut self, data: Vec<u8>, offset: vm_address_t) {
        // 挂起
        // 保护
        // 写入
        // 恢复保护
        // 继续运行
        self.offset = offset;
        self.data = data;
        self.suspend();
        self.protection();
        self.do_write();
        self.restore_protection();
        self.resuming();
    }
    pub fn read_u64(&mut self, addr: u64, data: &mut u64) -> bool {
        let buffer: [u8; 8] = [0; 8];
        let mut buffer = buffer.to_vec();
        let res = self.read(addr, &mut buffer, 8);
        if res {
            let bytes = &buffer[..8]; // 只取前 8 个字节
            *data = u64::from_le_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ]);
        }
        res
    }
    pub fn read(&mut self, addr: u64, data: &mut Vec<u8>, size: usize) -> bool {
        let mut outsize: mach_vm_size_t = 0;
        let result = unsafe {
            mach_vm_read_overwrite(
                self.task,
                addr as mach_vm_address_t,
                size as mach_vm_size_t,
                data.as_mut_ptr() as mach_vm_address_t,
                &mut outsize,
            )
        };
        if result != KERN_SUCCESS {
            eprintln!("无法读取内存: {:?}", result);
            false
        } else {
            data.truncate(outsize as usize);
            true
        }
    }
    fn do_write(&mut self) {
        let data_ptr: *const u8 = self.data.as_ptr();
        let result = unsafe {
            mach_vm_write(
                self.task,
                self.protection_addr,
                data_ptr as vm_offset_t,
                self.protection_size as mach_msg_type_number_t,
            )
        };
        if result != KERN_SUCCESS {
            eprintln!("无法写入内存: {:?}", result);
        } else {
            println!(
                "地址 0x{:x} 已修改为: {:?}",
                self.protection_addr, self.data
            );
        }
    }
    fn restore_protection(&mut self) {
        let target_addr = (self.base_addr + self.offset) as mach_vm_address_t;
        let protect_result = unsafe {
            mach_vm_protect(
                self.task, // 目标任务
                target_addr,
                self.protection_size, // 大小
                false as boolean_t,   // 不修改最大保护值
                self.protection,      // 设置为可读可写
            )
        };
        if protect_result != KERN_SUCCESS {
            eprintln!("无法恢复protection {:?}", protect_result);
        }
    }
    fn protection(&mut self) {
        self.protection_size = self.data.len() as mach_vm_size_t;
        let target_addr = (self.base_addr + self.offset) as mach_vm_address_t;
        self.protection_addr = target_addr;
        let protect_result = unsafe {
            mach_vm_protect(
                self.task, // 目标任务
                target_addr,
                self.protection_size,         // 大小
                false as boolean_t,           // 不修改最大保护值
                VM_PROT_READ | VM_PROT_WRITE, // 设置为可读可写
            )
        };

        if protect_result != 0 {
            eprintln!("Failed to change memory protection: {}", protect_result);
            let protect_result = unsafe {
                mach_vm_protect(
                    self.task, // 目标任务
                    target_addr,
                    self.protection_size,                        // 大小
                    false as boolean_t,                          // 不修改最大保护值
                    VM_PROT_READ | VM_PROT_WRITE | VM_PROT_COPY, // 设置为可读可写
                )
            };
            if protect_result != KERN_SUCCESS {
                eprintln!(
                    "Failed to change memory protection again: {}",
                    protect_result
                );
            }
        } else {
            println!("Memory protection changed successfully.");
        }
    }
    fn suspend(&mut self) {
        // 暂停目标进程
        let result = unsafe { task_suspend(self.task) };
        if result != KERN_SUCCESS {
            eprintln!("无法暂停目标任务: {:?}", result);
        }
    }
    fn resuming(&mut self) {
        // 恢复目标进程
        let result = unsafe { task_resume(self.task) };
        if result != KERN_SUCCESS {
            eprintln!("无法恢复目标任务: {:?}", result);
        }
    }
    pub fn get_base_address(&mut self) -> Option<vm_address_t> {
        unsafe {
            let mut address: vm_address_t = 0;
            let mut size: vm_size_t = 0;
            let mut info: vm_region_basic_info = std::mem::zeroed();
            let mut info_count = std::mem::size_of_val(&info) as mach_msg_type_number_t;
            let mut object_name: mach_port_t = 0;
            while mach_vm_region(
                self.task,
                &mut address as *mut _ as *mut u64,
                &mut size as *mut _ as *mut u64,
                VM_REGION_BASIC_INFO_64,
                &mut info as *mut _ as *mut i32,
                &mut info_count,
                &mut object_name,
            ) == KERN_SUCCESS
            {
                if info.protection & VM_PROT_EXECUTE != 0 {
                    self.base_addr = address;
                    self.protection = info.protection;
                    return Some(address);
                }
                address += size;
            }
        }
        None
    }
}
