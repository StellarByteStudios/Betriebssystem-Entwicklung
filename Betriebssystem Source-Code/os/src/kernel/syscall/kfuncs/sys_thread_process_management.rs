use core::{ptr, slice};

use crate::{
    devices::pcspk,
    kernel::{
        processes::process_handler,
        threads::{scheduler, scheduler::Scheduler},
    },
};

// = = Einfache ID getter = = //
#[no_mangle]
pub extern "C" fn sys_getpid() -> usize {
    let pid = scheduler::get_active_pid();
    return pid;
}

#[no_mangle]
pub extern "C" fn sys_gettid() -> usize {
    let tid = scheduler::get_active_tid();
    return tid;
}

// = = Beenden von Threads und Prozessen = = //
#[no_mangle]
pub extern "C" fn sys_exit_thread() {
    scheduler::exit_current_thread();
}

#[no_mangle]
pub extern "C" fn sys_exit_process(){
    let pid = scheduler::get_active_pid();
    kprintln!("exit_process number: {}", pid);
    process_handler::remove_process_by_pid(pid);
    loop {
        Scheduler::yield_cpu();
    }
}

#[no_mangle]
pub extern "C" fn sys_kill_process(pid: usize) {
    process_handler::remove_process_by_pid(pid);
    Scheduler::kill_thread_with_pid(pid);

    // Speaker deaktivieren, falls der ncoh lief beim killen der App
    pcspk::speaker_off();
}

#[no_mangle]
pub extern "C" fn sys_show_threads(){
    Scheduler::print_ready_queue();
}

// = = Läd den Namen des Laufenden Prozess in den Buffer = = //
#[no_mangle]
pub extern "C" fn sys_read_process_name(buff: *mut u8, len: usize) -> usize {
    // Name laden
    let active_pid = scheduler::get_active_pid();
    let process_name = process_handler::get_app_name(active_pid);

    // Ist der Name da?
    if process_name.is_none() {
        return 0;
    }

    // Länge prüfen
    let name_length = process_name.clone().unwrap().len();
    if name_length > len as usize {
        return 0;
    }

    // Schreiben in den übergebenen Buffer
    unsafe {
        // Einzeln bytes kopieren
        let process_name_owned = process_name.clone().unwrap(); // Store the owned value
        let bytes = process_name_owned.as_bytes(); // Create a byte slice
        for i in 0..name_length {
            *buff.offset(i as isize) = bytes[i];
        }
    }

    return name_length;
}
