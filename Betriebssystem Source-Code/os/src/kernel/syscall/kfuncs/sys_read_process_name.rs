use core::{ptr, slice};
use crate::kernel::processes::process;
use crate::kernel::threads::scheduler;

#[no_mangle]
pub extern "C" fn sys_read_process_name(buff: *mut u8, len: u64) -> u64 {
    // Name laden
    let active_pid = scheduler::get_active_pid();
    let process_name = process::get_app_name(active_pid);
    
    
    
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
        let bytes = process_name_owned.as_bytes();             // Create a byte slice
        for i in 0..name_length {
            *buff.offset(i as isize) = bytes[i];
        }
    }

    return name_length as u64;
}
