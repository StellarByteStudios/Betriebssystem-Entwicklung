use core::{ptr, slice, str};

use crate::{
    devices::graphical::graphic_console_printer::{print_string, print_string_on_position},
    kernel::{cpu, threads::scheduler},
};

#[no_mangle]
pub extern "C" fn sys_hello_world() {
    kprintln!(
        "Hello World from user thread tid={}",
        scheduler::get_active_tid()
    );
}

#[no_mangle]
pub extern "C" fn sys_hello_world_print(arg0: u64) {
    kprintln!("Hello World with Argument={}", arg0);
}

#[no_mangle]
pub extern "C" fn sys_kernel_print(buff: *const u8, len: u64) -> i64 {
    // Fehlerabfrage
    if buff.is_null() || len == 0 {
        return -1;
    }

    unsafe {
        // Aus dem Buffer ein Slice machen
        let byte_slice = slice::from_raw_parts(buff, len as usize);

        // Slice zu &str convertieren
        let string = str::from_utf8(byte_slice).unwrap();

        kprint!("{}", string);
    }

    return 0;
}


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

#[no_mangle]
pub extern "C" fn sys_graphical_print_pos(x: u64, y: u64, buff: *const u8, len: u64) -> i64 {
    // Fehlerabfrage
    if buff.is_null() || len == 0 {
        return -1;
    }

    unsafe {
        // Aus dem Buffer ein Slice machen
        let byte_slice = slice::from_raw_parts(buff, len as usize);

        // Slice zu &str convertieren
        let string = str::from_utf8(byte_slice).unwrap();

        let e = cpu::disable_int_nested();
        print_string_on_position(x, y, string);
        cpu::enable_int_nested(e);
    }

    return 0;
}
