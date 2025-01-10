use kernel::logger;


#[no_mangle]
pub extern "C" fn kmain(mbi: u64) {
    kprintln!("kmain");

    // initialize the logger
    logger::init();
    info!("Logger active");


    //
    // viele andere Schritte
    //


    // Kernel-Prozess mit Idle-Thread erzeugen und im Scheduler registrieren
    scheduler::Scheduler::spawn_kernel();   

    // Apps aus initrd.tar extrahieren
    let opt_apps = multiboot::get_apps_from_tar(mbi);

    // Prozesse mit je einem Thread fuer alle Apps erzeugen & im Scheduler registrieren
    match opt_apps {
        None => kprintln!("No apps found."),
        Some(mut apps) => {
            kprintln!("Found following apps in 'initrd': {:?}", apps);
            // Prozesse fuer alle Apps erzeugen
            loop {
                let app = apps.pop();
                if app.is_none() {
                    break;
                }
                scheduler::Scheduler::spawn(app.unwrap());   

            }
        }
    }

    println!("Welcome to hhuTOSr");

    // Scheduler starten & Interrupts erlauben
    scheduler::Scheduler::schedule();
}
