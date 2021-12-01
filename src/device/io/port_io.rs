use std::sync::Mutex;

use crate::device::io::map::IOMap;

lazy_static! {
    pub static ref MAPS: Mutex<Vec<IOMap>> = Mutex::new(Vec::new());
}

pub const PORT_IO_SPACE_MAX: usize = 65536;

pub fn add_physical_io_map(
    name: String,
    low: usize,
    len: usize,
    space: Vec<u8>,
    callback: fn(u32, i32, bool),
) {
    let map = IOMap::new(name, low, low+len-1, space, callback);
    MAPS.lock().unwrap().push(map.clone());
    dbg!(map);
}
