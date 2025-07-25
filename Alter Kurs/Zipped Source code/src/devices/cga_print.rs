/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Module: cga_print                                                       ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Descr.: Implements the macros print! and println! using cga. The macro  ║
   ║         implementation uses a mutex, so they should not be used within  ║
   ║         an interrupt handler!                                           ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Author: Philipp Oppermann, see here:                                    ║
   ║            https://os.phil-opp.com/vga-text-mode/                       ║
   ╚═════════════════════════════════════════════════════════════════════════╝
*/
use crate::devices::cga;
use crate::kernel::cpu;
use core::fmt;
use core::fmt::Write;
use spin::Mutex;

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
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => cga::print_byte(byte),

                // not part of printable ASCII range
                _ => cga::print_byte(0xfe),
            }
        }
        Ok(())
    }
}

// Provide macros like in the 'io' module of Rust
// The $crate variable ensures that the macro also works
// from outside the 'std' crate.
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::devices::cga_print::print(format_args!($($arg)*));
    });
}

macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

// Helper function of print macros (must be public)
pub fn print(args: fmt::Arguments) {
    //let ie = cpu::disable_int_nested();
    WRITER.lock().write_fmt(args).unwrap();
    //cpu::enable_int_nested(ie);
}
