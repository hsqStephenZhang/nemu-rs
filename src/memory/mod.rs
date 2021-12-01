pub mod dram;

pub const CONFIG_MSIZE: usize = 0x2000000;
pub const CONFIG_MBASE: usize = 0x0;

pub const PHYSICAL_MEM: [u8; CONFIG_MSIZE] = [0; CONFIG_MSIZE];

pub fn host_read(addr: *const u8, size: usize) -> i32 {
    return unsafe {
        match size {
            1 => *addr as i32,
            2 => *(addr as *const i16) as i32,
            4 => *(addr as *const i32),
            _ => {
                panic!("read size wrong:{}", size)
            }
        }
    };
}

pub fn host_write(addr: *mut u8, size: usize, data: i32) {
    unsafe {
        match size {
            1 => *addr = data as u8,
            2 => *(addr as *mut u16) = data as u16,
            4 => *(addr as *mut u32) = data as u32,
            _ => {
                panic!("read size wrong:{}", size)
            }
        }
    };
}

pub fn in_physical_mem(addr: *const u8) -> bool {
    let a = addr as usize;
    return a >= CONFIG_MBASE && a <= (CONFIG_MSIZE + CONFIG_MBASE);
}

#[allow(unused_variables)]
pub fn physical_addr_read(addr: *const u8, size: usize) -> i32 {
    let a = addr as usize;
    if in_physical_mem(addr) {
        host_read(addr, size)
    } else {
        panic!(
            "addr out of memory bound,addr:{:X},bound:{:X}-{:X}",
            addr as usize,
            CONFIG_MBASE,
            CONFIG_MBASE + CONFIG_MSIZE
        );
    }
}
#[allow(unused_variables)]
pub fn physical_addr_write(addr: *mut u8, size: usize, data: i32) {
    let a = addr as usize;
    if in_physical_mem(addr) {
        host_write(addr, size, data)
    } else {
        panic!(
            "addr out of memory bound,addr:{:X},bound:{:X}-{:X}",
            addr as usize,
            CONFIG_MBASE,
            CONFIG_MBASE + CONFIG_MSIZE
        );
    }
}

#[inline(always)]
pub fn vaddr_read(addr: *const u8, size: usize) -> i32 {
    return physical_addr_read(addr, size);
}

#[inline(always)]
pub fn vaddr_write(addr: *mut u8, size: usize, data: i32) {
    return physical_addr_write(addr, size, data);
}
