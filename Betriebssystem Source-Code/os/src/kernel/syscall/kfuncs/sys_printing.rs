use core::{ptr, slice, str};

use crate::{
    devices::graphical::graphic_console_printer::{print_string, print_string_on_position},
    kernel::{cpu, shell::shell_logic, threads::scheduler},
};

#[no_mangle]
pub extern "C" fn sys_hello_world() {
    kprintln!(
        "Hello World from user thread tid={}",
        scheduler::get_active_tid()
    );
}

#[no_mangle]
pub extern "C" fn sys_hello_world_print(arg0: usize) {
    kprintln!("Hello World with Argument={}", arg0);
}

#[no_mangle]
pub extern "C" fn sys_print_apps() {
    // Funktion aus der Shell aufrufen
    shell_logic::print_all_apps();
}

#[no_mangle]
pub extern "C" fn sys_kernel_print(buff: *const u8, len: usize) -> usize {
    // Fehlerabfrage
    if buff.is_null() || len == 0 {
        return 1;
    }

    unsafe {
        // Aus dem Buffer ein Slice machen
        let byte_slice = slice::from_raw_parts(buff, len);

        // Slice zu &str convertieren
        let string = str::from_utf8(byte_slice).unwrap();

        kprint!("{}", string);
    }

    return 0;
}

#[no_mangle]
pub extern "C" fn sys_graphical_print(buff: *const u8, len: usize) -> usize {
    // Fehlerabfrage
    if buff.is_null() || len == 0 {
        return 1;
    }

    unsafe {
        // Aus dem Buffer ein Slice machen
        let byte_slice = slice::from_raw_parts(buff, len);

        // Slice zu &str convertieren
        let string = str::from_utf8(byte_slice).unwrap();

        print_string(string);
    }

    return 0;
}

#[no_mangle]
pub extern "C" fn sys_graphical_print_pos(
    x: usize,
    y: usize,
    buff: *const u8,
    len: usize,
) -> usize {
    // Fehlerabfrage
    if buff.is_null() || len == 0 {
        return 1;
    }

    unsafe {
        // Aus dem Buffer ein Slice machen
        let byte_slice = slice::from_raw_parts(buff, len);

        // Slice zu &str convertieren
        let string = str::from_utf8(byte_slice).unwrap();

        let e = cpu::disable_int_nested();
        print_string_on_position(x as u64, y as u64, string);
        cpu::enable_int_nested(e);
    }

    return 0;
}
