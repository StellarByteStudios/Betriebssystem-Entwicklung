/
// Verwaltungsstruktur fuer einen Thread
#[repr(C)]
pub struct Thread {
    pub pid: u64, // Zu welchem Prozess gehoert dieser Prozess
    pub tid: u64,
    is_kernel_thread: bool,
    pml4_addr: PhysAddr, // Einstieg in die Seitentabellen
    old_rsp0: u64,       // letzter genutzter Stackeintrag, Kernel-Stack
    // der User-Stack-Ptr. wird auto. durch die Hardware gesichert
    user_stack: Box<stack::Stack>,
    kernel_stack: Box<stack::Stack>,
    entry: extern "C" fn(),
}

impl Thread {
 
    // Neuen Thread anlegen
    //    mypml4_addr:    Adressraum, zu dem dieser Thread gehoert
    //    mypid:          ProzessID, zu welchem Prozess der Thread gehoert
    //    kernel_thread:  Kernel oder User-Thread
    pub fn new(myentry: extern "C" fn(), mypml4_addr: PhysAddr, mypid: u64, kernel_thread: bool) -> Box<Thread> {

        /*
         * Hier muss Code eingefuegt werden
         */
         
        threadobj.prepare_kernel_stack();

        threadobj
    }

}
