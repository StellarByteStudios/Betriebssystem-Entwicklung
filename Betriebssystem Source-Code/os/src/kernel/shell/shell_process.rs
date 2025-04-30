use alloc::{boxed::Box, string::ToString, vec::Vec};

use crate::{
    boot::appregion::AppRegion,
    kernel::{
        processes::process_handler::create_fresh_process,
        shell::{input, shell_logic},
        threads::{scheduler, scheduler::Scheduler, thread, thread::Thread},
    },
    utility::delay,
};

// Statisches Feld, damit der Thread später darauf zugreifen kann
static mut APP_LIST: Vec<AppRegion> = Vec::new();

pub extern "C" fn shell_thread_entry() {
    kprintln!("Shell wird initialisiert");
    unsafe {
        shell_logic::init_keyboardhandler(APP_LIST.clone());
    }

    loop {
        // Char laden
        let c = input::getchar();
        // Char abarbeiten
        shell_logic::handle_keystroke(c);
    }
}

// Thread erstellen
fn init_shell_thread(pid: usize) -> Box<Thread> {
    let shell_thread: Box<Thread> =
        Thread::new_name(pid, shell_thread_entry, true, "Shell-Thread".to_string());

    return shell_thread;
}

// Prozess erstellen
pub fn spawn_shell_process(apps: Vec<AppRegion>) {
    // Apps abspeichern
    unsafe {
        APP_LIST = apps.clone();
    }

    // Neuen Prozess anlegen
    let shell_pid = create_fresh_process("Shell-Prozess");

    // Shell-Thread mit Pid anlegen
    let shell_thread = init_shell_thread(shell_pid);

    // Thread dem Scheduler geben
    Scheduler::ready(shell_thread);
}
