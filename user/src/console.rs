use core::fmt::{self, Write};

use crate::config::SERIAL_PORT;

#[inline(always)]
pub fn putch(ch: u8) {
    unsafe {
        core::ptr::write_volatile(SERIAL_PORT, ch);
    }
}

pub fn print_str(s: &str) {
    for &b in s.as_bytes() {
        putch(b);
    }
}

struct SerialWriter;

impl Write for SerialWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        print_str(s);
        Ok(())
    }
}

pub fn print_fmt(args: fmt::Arguments) {
    SerialWriter.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::console::print_fmt(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ({
        $crate::print!("{}\n", format_args!($($arg)*));
    })
}
