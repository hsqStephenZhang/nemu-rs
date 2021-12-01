use crate::{memory::{host_read, host_write}, device::io::port_io::PORT_IO_SPACE_MAX};

#[derive(Debug, Clone)]
pub struct IOMap {
    pub name: String,
    pub low: usize,
    pub high: usize,
    pub space: Vec<u8>,
    pub callback: fn(u32, i32, bool),
}

impl IOMap {
    #[inline]
    pub fn inside(&self, addr: usize) -> bool {
        addr >= self.low && addr <= self.high
    }
}

impl IOMap {
    pub fn new(
        name: String,
        low: usize,
        high: usize,
        space: Vec<u8>,
        callback: fn(u32, i32, bool),
    ) -> Self {
        Self {
            name,
            low,
            high,
            space,
            callback,
        }
    }
}

impl IOMap {
    pub fn read(&self, addr: *const u8, len: usize) -> i32 {
        assert!(len >= 1 && len <= 8);
        let offset = addr as u32 - self.low as u32;
        let f = self.callback;
        f(offset, len as i32, false);
        let addr = (self.space.as_ptr() as u32 + offset) as *const u8;
        return host_read(addr, len);
    }
    pub fn write(&self, addr: *const u8, len: usize, data: i32) {
        assert!(len >= 1 && len <= 8);
        let offset = addr as u32 - self.low as u32;
        let f = self.callback;
        let addr = (self.space.as_ptr() as u32 + offset) as *mut u8;
        host_write(addr, len, data);
        f(offset, len as i32, true);
    }

    // bus interface
    pub fn mmio_read(&self, addr: *const u8, len: usize) -> i32 {
        self.read(addr, len)
    }

    pub fn mmio_write(&self, addr: *const u8, len: usize, data: i32) {
        self.write(addr, len, data);
    }

    // device interface
    pub fn physical_io_read(&self, addr: *const u8, len: usize) -> i32 {
        assert!(addr  as usize+ len - 1 < PORT_IO_SPACE_MAX);
        self.read(addr, len)
    }

    pub fn physical_io_write(&self, addr: *const u8, len: usize, data: i32) {
        assert!(addr  as usize+ len - 1 < PORT_IO_SPACE_MAX);
        self.write(addr, len, data);
    }
}

pub fn find_mapid_by_addr(maps: &Vec<IOMap>, addr: usize) -> Option<usize> {
    for (index, map) in maps.iter().enumerate() {
        if map.inside(addr) {
            return Some(index);
        }
    }
    None
}

pub fn fetch_mmio_map(maps: &Vec<IOMap>, addr: usize) -> Option<&IOMap> {
    find_mapid_by_addr(maps, addr).map(|index| &maps[index])
}
