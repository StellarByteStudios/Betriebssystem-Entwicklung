use core::{fmt, fmt::Write};

use spin::Mutex;

use crate::{
    devices::{graphical::graphic_console_printer, serial},
    kernel::cpu,
};

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
        graphic_console_printer::print_string(s);
        return Ok(());
    }
}

// Provide macros like in the 'io' module of Rust
// The $crate variable ensures that the macro also works
// from outside the 'std' crate.
macro_rules! vprint {
    ($($arg:tt)*) => ({
        $crate::devices::graphical::vprint::vprint(format_args!($($arg)*));
    });
}

macro_rules! vprintln {
    ($fmt:expr) => (vprint!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (vprint!(concat!($fmt, "\n"), $($arg)*));
}

// Helper function of print macros (must be public)
pub fn vprint(args: fmt::Arguments) {
    WRITER.lock().write_fmt(args).unwrap();
}
