use core::time;
use patch_core::Patch;
use std::{
    ffi::{c_char, CStr, CString},
    ptr::{self, NonNull},
    thread::sleep,
};
#[no_mangle]
#[link_section = "__DATA,__mod_init_func"]
pub static INIT_ARRAY: extern "C" fn() = init;

#[repr(C)]
pub struct Person {
    id: i32,
    name: [c_char; 20],
}
impl Person {
    // 从 C 地址创建 Person 结构体引用
    unsafe fn from_raw(ptr: *mut Person) -> NonNull<Person> {
        NonNull::new_unchecked(ptr)
    }

    // 打印 Person 的信息
    pub fn print(&self) {
        unsafe {
            let name_cstr = CStr::from_ptr(self.name.as_ptr());
            println!("ID: {}, Name: {:?}", self.id, name_cstr.to_str().unwrap());
        }
    }
    pub fn set_id(&mut self, new_id: i32) {
        self.id = new_id;
    }
    pub fn set_name(&mut self, new_name: &str) {
        let c_string = CString::new(new_name).expect("CString::new failed");
        let c_str = c_string.as_bytes_with_nul();

        // 确保新字符串长度不超过 name 数组的长度
        if c_str.len() <= self.name.len() {
            unsafe {
                // 直接操作 name 数组
                ptr::copy_nonoverlapping(
                    c_str.as_ptr(),
                    self.name.as_mut_ptr() as *mut u8,
                    c_str.len(),
                );
                // 填充剩余部分
                if c_str.len() < self.name.len() {
                    ptr::write_bytes(
                        self.name.as_mut_ptr().add(c_str.len()),
                        0,
                        self.name.len() - c_str.len(),
                    );
                }
            }
        } else {
            println!("New name is too long.");
        }
    }
}
#[no_mangle]
pub extern "C" fn init() {
    println!("Library initialized!");
    let mut p = Patch::from_name("add").unwrap();
    let base_addr = p.get_base_address().unwrap();
    let p1 = base_addr + 0x2008;
    // 你知道的十六进制地址
    let hex_address: u64 = p1 as u64; // 示例地址
    println!(
        "pid: {}, hex_address: {:?}",
        p.pid, hex_address as *const u64
    );
    // 将十六进制地址转换为指针
    let address: *const u8 = hex_address as *const u8;
    let mut really: u64 = 0;
    unsafe {
        // 从指定地址读取 8 字节数据到 u64 变量
        std::ptr::copy_nonoverlapping(address, &mut really as *mut u64 as *mut u8, 8);
    }
    // 将十六进制地址转换为原始指针
    let c_person_address: *mut Person = (really as *const u64) as *mut Person;
    let mut person: Option<NonNull<Person>> = None;
    unsafe {
        if !c_person_address.is_null() {
            let mut p = Person::from_raw(c_person_address);
            p.as_mut().set_id(100);
            p.as_mut().set_name("hello");
            person = Some(p);
        } else {
            println!("Received a null pointer.");
        }
        loop {
            if let Some(p) = person {
                let p_ref = p.as_ref();
                p_ref.print();
            }
            println!("Library loop running ...!");
            sleep(time::Duration::from_secs(3));
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
}
