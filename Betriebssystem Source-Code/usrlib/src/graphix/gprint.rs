use core::{fmt, fmt::Write};

use spin::Mutex;

use crate::kernel::syscall::user_api::usr_graphical_print;

// The global writer that can used as an interface from other modules
// It is threadsafe by using 'Mutex'
pub static WRITER: Mutex<Writer> = Mutex::new(Writer {});

// Defining a Writer for writing formatted strings to the CGA screen
pub struct Writer {}

// Implementation of the 'core::fmt::Write' trait for our Writer
// Required to output formatted strings
// Requires only one function 'write_str'
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        usr_graphical_print(s.as_ptr(), s.len() as u64);
        return Ok(());
    }
}

// Provide macros like in the 'io' module of Rust
// The $crate variable ensures that the macro also works
// from outside the 'std' crate.
#[macro_export]
macro_rules! gprint {
    ($($arg:tt)*) => ({
        $crate::graphix::gprint::gprint(format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! gprintln {
    ($fmt:expr) => {
        $crate::graphix::gprint::gprint(format_args!(concat!($fmt, "\n")));
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::graphix::gprint::gprint(format_args!(concat!($fmt, "\n"), $($arg)*));
    };
}

// Helper function of print macros (must be public)
pub fn gprint(args: fmt::Arguments) {
    //let ie = cpu::disable_int_nested();
    WRITER.lock().write_fmt(args).unwrap();
    //cpu::enable_int_nested(ie);
}
