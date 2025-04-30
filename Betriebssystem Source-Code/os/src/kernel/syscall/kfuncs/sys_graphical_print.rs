use core::{ptr, slice, str};

use crate::devices::graphical::graphic_console_printer::print_string;

#[no_mangle]
pub extern "C" fn sys_graphical_print(buff: *const u8, len: u64) -> i64 {
    // Fehlerabfrage
    if buff.is_null() || len == 0 {
        return -1;
    }

    unsafe {
        // Aus dem Buffer ein Slice machen
        let byte_slice = slice::from_raw_parts(buff, len as usize);

        // Slice zu &str convertieren
        let string = str::from_utf8(byte_slice).unwrap();

        print_string(string);
    }

    return 0;
}
