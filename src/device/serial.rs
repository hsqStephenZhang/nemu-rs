use super::io::port_io::add_physical_io_map;

const CHART_OFFSET:u32=0;

pub fn serial_putc(c: u8) {
    print!("{}", c);
}

pub fn serial_io_handler(offset:u32,len:i32,is_write:bool) {
    assert!(len==1);
    match offset {
        CHART_OFFSET=>{
            if is_write{
                serial_putc(1);
            }else{
                panic!("not supported read")
            }
        }
        _ => panic!()
    }
}

// [0,7] is serial output port
pub fn init_serial() {
    add_physical_io_map("serial".into(), 0, 8, vec![0;8], serial_io_handler)
}
