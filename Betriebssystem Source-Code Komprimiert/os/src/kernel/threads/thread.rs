/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Module: thread                                                          ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Descr.: Functions for creating, starting, switching and ending threads. ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Autor:  Michael Schoettner, 11.06.2024                                  ║
   ╚═════════════════════════════════════════════════════════════════════════╝
*/
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;

use crate::consts;
use crate::devices::cga;
use crate::kernel::cpu;
use crate::kernel::paging::frames::pf_alloc;
use crate::kernel::paging::pages;
use crate::kernel::paging::physical_addres::PhysAddr;
use crate::kernel::threads::scheduler;
use crate::kernel::threads::stack;
use crate::mylib::queue::Link;

// Diese Funktionen sind in 'thread.asm'
extern "C" {
    fn _thread_kernel_start(old_rsp0: u64);
    fn _thread_user_start(old_rsp0: u64);
    fn _thread_switch(now_rsp0: *mut u64, then_rsp0: u64, then_rsp0_end: u64, then_pml4: u64);
}

// Diese Funktion (setzt den Kernel-Stack im TSS) ist in 'boot.asm'
extern "C" {
    fn _tss_set_rsp0(old_rsp0: u64);
}

// Verwaltungsstruktur fuer einen Thread
#[repr(C)]
pub struct Thread {
    tid: usize,
    is_kernel_thread: bool,
    pml4_addr: PhysAddr, // Einstieg in die Seitentabellen
    old_rsp0: u64,       // letzter genutzter Stackeintrag im Kernel-Stack
    // der User-Stack-Ptr. wird auto. durch die Hardware gesichert

    // Speicher fuer den User-Stack
    user_stack: Box<stack::Stack>, // Speicher fuer den User-Stack

    kernel_stack: Box<stack::Stack>, // Speicher fuer den Kernel-Stack
    entry: extern "C" fn(),

    // Name und Argumente
    args: Vec<String>,
    name: String,
}

impl Thread {
    // Interne Funktion die die Threads wirklich erstellt
    fn internal_new(
        myentry: extern "C" fn(),
        kernel_thread: bool,
        thread_name: String,
        my_args: Vec<String>,
    ) -> Box<Thread> {
        // Neue ID erstellen
        let new_tid = scheduler::next_thread_id();

        // Oberste Page-Table anlegen
        //let new_pml4_addr = pf_alloc(1, true);
        // Oberste Page-Table anlegen (mit Kernel initialisiert)
        let new_pml4_addr = pages::pg_init_user_tables();

        vprintln!("======= Neuen Thread angelegt.");
        vprintln!("     Die pml4 liegt bei 0x{:x}", new_pml4_addr.0);

        let my_kernel_stack =
            stack::Stack::new_mapped_stack(consts::STACK_SIZE, true, new_pml4_addr);
        let my_user_stack =
            stack::Stack::new_mapped_stack(consts::STACK_SIZE, false, new_pml4_addr);
        // Es hilft nicht, den Userstack einfach nur als Kernelstack anzulegen. Gibt trotzdem ProtFault.

        // Thread-Objekt anlegen
        let mut threadobj = Box::new(Thread {
            tid: new_tid,
            is_kernel_thread: kernel_thread,
            pml4_addr: new_pml4_addr,
            old_rsp0: 0,
            kernel_stack: my_kernel_stack,
            user_stack: my_user_stack,
            entry: myentry,
            args: my_args,
            name: thread_name,
        });

        threadobj.prepare_kernel_stack();

        return threadobj;
    }

    // Neuen Thread anlegen
    pub fn new(my_tid: usize, myentry: extern "C" fn(), kernel_thread: bool) -> Box<Thread> {
        return Thread::internal_new(myentry, kernel_thread, String::from("Nameless"), Vec::new());
    }

    /*
     * Erwerterung für Name und Args
     */
    /**
       Description: Create new Thread (Mit Args!)
    */
    pub fn new_with_args(
        my_tid: usize,
        myentry: extern "C" fn(),
        kernel_thread: bool,
        thread_name: String,
        my_args: Vec<String>,
    ) -> Box<Thread> {
        return Thread::internal_new(myentry, kernel_thread, thread_name, my_args);
    }

    /**
       Description: Create new Thread (mit Name)
    */
    pub fn new_name(
        my_tid: usize,
        myentry: extern "C" fn(),
        kernel_thread: bool,
        thread_name: String,
    ) -> Box<Thread> {
        return Thread::internal_new(myentry, kernel_thread, thread_name, Vec::new());
    }

    // Starten des 1. Kernel-Threads (rsp0 zeigt auf den praeparierten Stack)
    // Wird vom Scheduler gerufen, wenn dieser gestartet wird.
    // Alle anderen Threads werden mit 'switch' angestossen
    pub fn start(now: *mut Thread) {
        kprintln!("======= In Kernal Start wird ausgeführt.");
        unsafe {
            kprintln!(
                "     Die pml4 liegt bei 0x{:x}",
                now.as_ref().unwrap().pml4_addr.0
            );
            pages::pg_set_cr3(now.as_ref().unwrap().pml4_addr); // Adressraum setzen
            _thread_kernel_start((*now).old_rsp0);
        }
    }

    // Umschalten von Thread 'now' auf Thread 'then'
    /*  Parameterreihenfolge
        1. rdi
        2. rsi
        3. rdx
        4. rcx
        5. r8
        6. r9
    */
    pub fn switch(now: *mut Thread, then: *mut Thread) {
        unsafe {
            _thread_switch(
                &mut (*now).old_rsp0,
                (*then).old_rsp0,
                (*then).kernel_stack.stack_end() as u64,
                (*now).pml4_addr.0,
            );
        }
    }

    //
    // Kernel-Stack praeparieren, fuer das Starten eines Threads im Ring 0
    // (wird in '_thread_kernel_start' und '_thread_switch' genutzt)
    // Im Wesentlichen wird hiermit der Stack umgeschaltet und durch
    // einen Ruecksprung die Funktion 'kickoff_kernel_thread' angesprungen.
    //
    // Die Interrupt werden nicht aktiviert.
    //
    fn prepare_kernel_stack(&mut self) {
        let kickoff_kernel_addr = kickoff_kernel_thread as *const ();
        let object: *const Thread = self;

        // sp0 zeigt ans Ende des Speicherblocks, passt somit
        let sp0: *mut u64 = self.kernel_stack.stack_end();

        // Stack initialisieren. Es soll so aussehen, als waere soeben die
        // die Funktion '_thread_kernel_start' aufgerufen worden. Diese
        // Funktion hat als Parameter den Zeiger "object" erhalten.
        // Da der Aufruf "simuliert" wird, kann fuer die Ruecksprung-
        // adresse in 'kickoff_kernel_addr' nur ein unsinniger Wert eingetragen
        // werden. Die Funktion 'kickoff_kernel_addr' muss daher dafuer sorgen,
        // dass diese Adresse nie benoetigt, sie darf also nicht zurueckspringen,
        // sonst kracht's.
        unsafe {
            *sp0 = 0x00DEAD00 as u64; // dummy Ruecksprungadresse

            *sp0.offset(-1) = kickoff_kernel_addr as u64; // Adresse von 'kickoff_kernel_thread'

            // Nun sichern wir noch alle Register auf dem Stack
            *sp0.offset(-2) = 2; // rflags (IOPL=0, IE=0)
            *sp0.offset(-3) = 0; // r8
            *sp0.offset(-4) = 0; // r9
            *sp0.offset(-5) = 0; // r10
            *sp0.offset(-6) = 0; // r11
            *sp0.offset(-7) = 0; // r12
            *sp0.offset(-8) = 0; // r13
            *sp0.offset(-9) = 0; // r14
            *sp0.offset(-10) = 0; // r15

            *sp0.offset(-11) = 0; // rax
            *sp0.offset(-12) = 0; // rbx
            *sp0.offset(-13) = 0; // rcx
            *sp0.offset(-14) = 0; // rdx

            *sp0.offset(-15) = 0; // rsi
            *sp0.offset(-16) = object as u64; // rdi -> 1. Param. fuer 'kickoff_kernel_thread'
            *sp0.offset(-17) = 0; // rbp

            // Zum Schluss speichern wir den Zeiger auf den zuletzt belegten
            // Eintrag auf dem Stack in 'rsp0'. Daruber gelangen wir in
            // _thread_kernel_start an die noetigen Register
            self.old_rsp0 = (sp0 as u64) - (8 * 17); // aktuellen Stack-Zeiger speichern
        }
    }

    fn prepare_user_stack(&mut self) {
        //kprintln!("Ich bin in prepare user stack angekommen");
        let kickoff_user_addr = kickoff_user_thread as *const ();
        let object: *const Thread = self;

        // sp0 zeigt ans Ende des Speicherblocks, passt somit
        let sp0: *mut u64 = self.kernel_stack.stack_end();
        let sp3: *mut u64 = self.user_stack.stack_end();

        unsafe {
            *sp0 = 0x00DEAD00 as u64; // dummy Ruecksprungadresse

            //*sp0.offset(-1) = kickoff_kernel_addr as u64; // Adresse von 'kickoff_kernel_thread'

            // Nun sichern wir noch alle Register auf dem Stack
            *sp0.offset(-1) = 0b0000_0000_0010_1011; // SS Register (Segment Selector)
                                                     //15-3 Bit Index in GDT = *Data*/Code? = 5 = 0b101 | 1Bit TI = GDT = 0| 2Bit PrivLevel = 3 = 0b11
            *sp0.offset(-2) = sp3 as u64; // ESP                            ===== Ka ob das so richtig ist
            *sp0.offset(-3) = 512 + 2; // EFLAGS                                  ===== Ka wo ich die hernehmen soll
            *sp0.offset(-4) = 0b0000_0000_0010_0011; // CS                  ===== Selbe wie SS nur mit Code?
                                                     //15-3 Bit Index in GDT = Data/*Code*? = 4 = 0b100 | 1Bit TI = GDT = 0| 2Bit PrivLevel = 3 = 0b11
            *sp0.offset(-5) = kickoff_user_addr as u64; // EIP              ===== Adresse für funktion?
                                                        //*sp0.offset(-6) = 0; // ERROR Code                              ===== KA deswegen mal alles auf 0
                                                        //15-3 Bit Index in GDT? | 1Bit TI | 1Bit IDT | 1Bit EXT

            // = = = Wo muss ich object as u64 als parameter für kickoff_kernel_ad
            *sp0.offset(-6) = object as u64; // rdi -> 1. Param. fuer 'kickoff_kernel_thread' ?

            // Zum Schluss speichern wir den Zeiger auf den zuletzt belegten
            // Eintrag auf dem Stack in 'rsp0'. Daruber gelangen wir in
            // _thread_kernel_start an die noetigen Register
            self.old_rsp0 = (sp0 as u64) - (8 * 6); // aktuellen Stack-Zeiger speichern
        }
    }

    //
    // Diese Funktion wird verwendet, um einen Thread vom Ring 0 in den
    // Ring 3 zu versetzen. Dies erfolgt wieder mit einem praeparierten Stack.
    // Hier wird ein Interrupt-Stack-Frame gebaut, sodass beim Ruecksprung
    // mit 'iretq' die Privilegstufe gewechselt wird. Wenn alles klappt
    // landen wir in der Funktion 'kickoff_user_thread' und sind dann im Ring 3
    //
    // In den Selektoren RPL = 3, RFLAGS = IOPL=0, IE=1
    //
    // Die Interrupt werden durch den 'iretq' aktiviert.
    //
    fn switch_to_usermode(&mut self) {
        //kprintln!("Switch to usermode wird ausgeführt");

        // Interrupt-Stackframe bauen
        self.prepare_user_stack();

        //kprintln!("Ich bin durch prepare_user_stack durchgekommen");

        // In den Ring 3 schalten -> Aufruf von '_thread_user_start'
        unsafe {
            _thread_user_start(self.old_rsp0);
        }
    }

    pub fn get_tid(thread_object: *const Thread) -> usize {
        unsafe { (*thread_object).tid }
    }

    pub fn get_raw_pointer(&mut self) -> *mut Thread {
        self
    }
}

// Notwendig, für die Queue-Implementierung im Scheduler
impl PartialEq for Thread {
    fn eq(&self, other: &Self) -> bool {
        self.tid == other.tid
    }
}

// Notwendig, falls wir die Ready-Queue ausgeben moechten
impl fmt::Display for Thread {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.tid)
    }
}

//
// Dies ist die erste Rust-Funktion, die aufgerufen wird, wenn
// ein neuer Thread startet (im Ring 0). Falls dies ein User-Thread
// ist, so wird von hier aus 'switch_to_usermode' gerufen.
//
// Hier sind die Interrupts noch gesperrt.
//
#[no_mangle]
pub extern "C" fn kickoff_kernel_thread(object: *mut Thread) {
    unsafe {
        kprintln!(
            "kickoff_kernel_thread, tid={}, old_rsp0 = {:x}, is_kernel_thread: {}, pagetable-addres: 0x{:x}",
            (*object).tid,
            (*object).old_rsp0,
            (*object).is_kernel_thread,
            (*object).pml4_addr.0,
        );
    }

    // Setzen von rsp0 im TSS
    unsafe {
        _tss_set_rsp0((*object).kernel_stack.stack_end() as u64);
    }

    // Falls dies ein User-Thread ist, schalten wir nun in den User-Mode
    // Der Aufruf kehrt nicht zurueck, schaltet aber IE = 1
    // Es geht anschliessend in 'kickoff_user_thread' weiter
    unsafe {
        if (*object).is_kernel_thread == false {
            (*object).switch_to_usermode();
        } else {
            // Interrupts wieder zulassen
            cpu::enable_int();
            ((*object).entry)();
        }
    }
    loop {}
}

//
// Dies ist die  Rust-Funktion, die aufgerufen wird, wenn ein
// Kernel-Thread (Ring 0) in den Ring 3 versetzt wird
//
#[no_mangle]
pub extern "C" fn kickoff_user_thread(object: *mut Thread) {
    // Einstiegsfunktion des Threads aufrufen

    // Setzen von rsp0 im TSS
    unsafe {
        //_tss_set_rsp0((*object).kernel_stack.stack_end() as u64);
        ((*object).entry)();
    }
    loop {}
}
