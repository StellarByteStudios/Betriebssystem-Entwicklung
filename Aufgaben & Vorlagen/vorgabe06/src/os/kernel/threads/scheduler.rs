


pub fn get_active_pid() -> u64 {
    let pid;
    let irq = cpu::disable_int_nested();
    unsafe {
        pid = Thread::get_pid(SCHEDULER.as_mut().unwrap().get_active());
    }
    cpu::enable_int_nested(irq);
    pid
}


impl Scheduler {

    /*****************************************************************************
     * Funktion:        spawn_kernel                                             *
     *---------------------------------------------------------------------------*
     * Beschreibung:    Kernel-Prozess mit Idle-Thread erzeugen und im Scheduler *
     *                  registrieren.                                            *
     *****************************************************************************/
    pub fn spawn_kernel() {
			
        /*
         * Hier muss Code eingefuegt werden
         */

    }

    /*****************************************************************************
     * Funktion:        spawn                                                    *
     *---------------------------------------------------------------------------*
     * Beschreibung:    Einen neuen Prozess mit dem Haupt-Thread erzeugen und    *
     *                  im Scheduler registrieren.                               *
     *                                                                           *
     * Parameter:       app    Code-Image fuer den neuen Prozess                 *
     *****************************************************************************/
    pub fn spawn(app: AppRegion) {

        /*
         * Hier muss Code eingefuegt werden
         */

    }

}
