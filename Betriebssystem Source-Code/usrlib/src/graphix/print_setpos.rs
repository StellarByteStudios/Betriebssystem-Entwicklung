use core::{fmt, fmt::Write};

use spin::Mutex;

use crate::kernel::syscall::user_api::{
    usr_get_screen_width, usr_graphical_print_pos, usr_hello_world_print,
};

// The global writer that can used as an interface from other modules
// It is threadsafe by using 'Mutex'
pub static WRITER: Mutex<Writer> = Mutex::new(Writer {
    cursor_x: 0,
    cursor_y: 0,
});

// Defining a Writer for writing formatted strings to the CGA screen
pub struct Writer {
    cursor_x: u64,
    cursor_y: u64,
}

// Implementation of the 'core::fmt::Write' trait for our Writer
// Required to output formatted strings
// Requires only one function 'write_str'
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        usr_graphical_print_pos(self.cursor_x, self.cursor_y, s.as_ptr(), s.len() as u64);
        self.update_pos(s.len() as u64);
        return Ok(());
    }
}

impl Writer {
    pub fn update_pos(&mut self, len: u64) {
        // Linewrap
        let max_witdh: u64 = usr_get_screen_width() / 10;

        if self.cursor_x + len > max_witdh {
            self.cursor_y = self.cursor_y + 1;
        }
        self.cursor_x = (self.cursor_x + len) % max_witdh;
    }
}

// Provide macros like in the 'io' module of Rust
// The $crate variable ensures that the macro also works
// from outside the 'std' crate.
#[macro_export]
macro_rules! print_setpos {
    ($x:expr, $y:expr, $($arg:tt)*) => {
        $crate::graphix::print_setpos::printer_set_pos($x, $y);
        $crate::graphix::print_setpos::print_with_pos(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! println_setpos {
    /*
    ($x:expr, $y:expr, $fmt:expr) => (print_setpos!($x, $y, concat!($fmt, "\n")));
    ($x:expr, $y:expr, $fmt:expr, $($arg:tt)*) => (print_setpos!($x, $y, concat!($fmt, "\n"), $($arg)*));*/
    // No additional arguments
    ($x:expr, $y:expr, $fmt:expr) => {
        $crate::graphix::print_setpos::printer_set_pos($x, $y);
        $crate::graphix::print_setpos::print_with_pos(format_args!(concat!($fmt, "\n")));
    };

    // With additional arguments
    ($x:expr, $y:expr, $fmt:expr, $($arg:tt)*) => {
        $crate::graphix::print_setpos::printer_set_pos($x, $y);
        $crate::graphix::print_setpos::print_with_pos(format_args!(concat!($fmt, "\n"), $($arg)*));
    };
}

// Helper function of print macros (must be public)
pub fn print_with_pos(args: fmt::Arguments) {
    usr_hello_world_print(52);
    // String ausgeben
    // TODO: !! Bei diesem Aufruf gibt es einen Page-Fault
    WRITER.lock().write_fmt(args).unwrap();
    usr_hello_world_print(53);
}

pub fn printer_set_pos(x: u64, y: u64) {
    usr_hello_world_print(51);
    // Writer nimmt nicht mehrere Argumente, deswegen umständlich
    let mut writer = WRITER.lock();

    // Cursor Position setzen
    writer.cursor_x = x;
    writer.cursor_y = y;

    drop(writer);
}
