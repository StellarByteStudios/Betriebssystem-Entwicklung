

#[no_mangle]
pub extern "C" fn kmain(mbi: u64) {
    // ...

    // separate compilierte App suchen
    let app_region = multiboot::get_apps(mbi);
    kprintln!("kmain, app: {:?}", app_region);

    // ...

    // Idle-Thread eintragen
    let idle_thread = Thread::new(idle_thread::idle_thread_entry, true);
    scheduler::Scheduler::ready(idle_thread);

    // Thread fuer eine App erzeugen & im Scheduler registrieren

    /*
     * Hier muss Code eingefuegt werden
     */
     
     
    // Scheduler starten & Interrupts erlauben
    scheduler::Scheduler::schedule();
}

