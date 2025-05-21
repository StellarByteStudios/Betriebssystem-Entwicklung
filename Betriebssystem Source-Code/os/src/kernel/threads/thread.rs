/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Module: thread                                                          ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Descr.: Functions for creating, starting, switching and ending threads. ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Autor:  Michael Schoettner, 11.06.2024                                  ║
   ╚═════════════════════════════════════════════════════════════════════════╝
*/
use alloc::{boxed::Box, string::String, vec::Vec};
use core::{fmt, mem::transmute};

use usrlib::kernel::runtime::environment::USER_SPACE_ENV_START;
use x86_64::registers::control::Cr3;

use crate::{
    boot::appregion::AppRegion,
    consts,
    consts::USER_CODE_VM_START,
    devices::cga,
    kernel::{
        cpu,
        paging::{
            frames::pf_alloc,
            pages,
            pages::{pg_mmap_user_environment, PageTable},
            physical_addres::PhysAddr,
        },
        processes::process_handler,
        threads::{scheduler, stack},
    },
};

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
    pub pid: usize, // Zu welchem Prozess gehört dieser Thread
    pub tid: usize,

    is_kernel_thread: bool,
    pml4_addr: PhysAddr, // Einstieg in die Seitentabellen
    old_rsp0: u64,       // letzter genutzter Stackeintrag im Kernel-Stack
    // der User-Stack-Ptr. wird auto. durch die Hardware gesichert
    user_stack: Box<stack::Stack>,   // Speicher fuer den User-Stack
    kernel_stack: Box<stack::Stack>, // Speicher fuer den Kernel-Stack

    entry: extern "C" fn(), // Einstiegsfunktion

    // Name und Argumente
    args: Vec<String>,
    name: String,
}

impl Thread {
    // Getter für die PageTable
    pub fn get_pml4_address(&self) -> PhysAddr {
        let return_page_address = self.pml4_addr.raw();

        let return_page_phys_add = PhysAddr::new(return_page_address);
        return return_page_phys_add;
    }

    // Interne Funktion die die Threads wirklich erstellt
    fn internal_new(
        myentry: extern "C" fn(),
        kernel_thread: bool,
        process_id: usize,
        thread_name: String,
        my_args: Vec<String>,
    ) -> Box<Thread> {
        // Neue ID erstellen
        let new_tid = scheduler::next_thread_id();

        // PML4 aus Prozess laden
        let process_pml4 = process_handler::get_pml4_address_by_pid(process_id);

        let my_kernel_stack =
            stack::Stack::new_mapped_stack(process_id, consts::STACK_SIZE, true, process_pml4);
        let my_user_stack =
            stack::Stack::new_mapped_stack(process_id, consts::STACK_SIZE, false, process_pml4);

        // Thread-Objekt anlegen
        let mut threadobj = Box::new(Thread {
            pid: process_id,
            tid: new_tid,
            is_kernel_thread: kernel_thread,
            pml4_addr: process_pml4,
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
    pub fn new(
        my_pid: usize,
        process_pml4: PhysAddr,
        myentry: extern "C" fn(),
        kernel_thread: bool,
    ) -> Box<Thread> {
        return Thread::internal_new(
            myentry,
            kernel_thread,
            my_pid,
            String::from("Nameless"),
            Vec::new(),
        );
    }

    /*
     * Erwerterung für Name und Args
     */
    /**
       Description: Create new Thread (Mit Args!)
    */
    pub fn new_with_args(
        my_pid: usize,
        myentry: extern "C" fn(),
        kernel_thread: bool,
        thread_name: String,
        my_args: Vec<String>,
    ) -> Box<Thread> {
        return Thread::internal_new(myentry, kernel_thread, my_pid, thread_name, my_args);
    }

    /**
       Description: Create new Thread (mit Name)
    */
    pub fn new_name(
        my_pid: usize,
        myentry: extern "C" fn(),
        kernel_thread: bool,
        thread_name: String,
    ) -> Box<Thread> {
        return Thread::internal_new(myentry, kernel_thread, my_pid, thread_name, Vec::new());
    }

    // Thread fuer eine App anlegen
    // Hier wird der Code & BSS eingemappt & ein Thread mit eigenem Adressraum erzeugt
    pub fn new_app_thread(app: AppRegion, pid: usize, args: &Vec<String>) -> Box<Thread> {
        // Entry-Thread konvertieren
        let thread_entry = unsafe { transmute::<usize, extern "C" fn()>(USER_CODE_VM_START) };

        let app_thread =
            Self::internal_new(thread_entry, false, pid, app.file_name.clone(), Vec::new());

        // ============ NEU! Environment ============ //
        // Gesammten Speicherplatz für die Argumente berechnen
        let args_size = args.iter().map(|arg| arg.len()).sum::<usize>();

        // Startadresse
        let env_virt_start = USER_SPACE_ENV_START;
        let env_size = args_size;

        // Mappen der Environment im App-Space
        // Startadresse im Virtuellen Adressraum
        kprintln!("--------------- Lege Mapping für Environment an");
        let phys_addr_of_env = pg_mmap_user_environment(pid, env_virt_start, env_size);

        // Aufbauen von argc und argv im Userspace
        // Im ersten Eintrag steht die Anzahl der Argumente
        let argc_phys: *mut usize = phys_addr_of_env.as_mut_ptr::<usize>();
        // Pointer einer pro Element in den Args
        let argv_phys = (phys_addr_of_env.raw() + size_of::<usize>() as u64) as *mut *mut u8;

        // Alle Argumente zum pointer kopieren
        unsafe {
            // Anzahl der Argumente in den Speicher schreiben
            argc_phys.write(args.len() + 1);

            // Physische Startadresse der Environment-Variablen
            // (args.len() + 1) weil davor ja nur die Pointer sind und dannach die Echten Inhalte kommen
            let args_begin_phys = argv_phys.offset((args.len() + 1) as isize).cast::<u8>();
            // Virtuelle Startadresse der Environment-Variablen
            let args_begin_virt = env_virt_start
                + size_of::<usize>() // Feld mit Anzahl Einträge
                + ((args.len() + 1) * size_of::<usize>()); // Platz für die Pointer

            // Programmname als erstes Argument speichern
            let name = app.file_name.clone();
            args_begin_phys.copy_from(name.as_bytes().as_ptr(), name.len());
            args_begin_phys.add(name.len()).write(0); // null-terminieren für den String

            // Pointer auf Anfang des Namens im Eingabearray speichern
            argv_phys.write(args_begin_virt as *mut u8);

            // Wo gehen die nächsten Argumente hin?
            let mut offset = name.len() + 1;

            // Restlichen Argumente kopieren
            for (i, arg) in args.iter().enumerate() {
                // An welche physische Adresse wird das Argument geschrieben
                let target_address = args_begin_phys.add(offset);

                // Den String roh in den Speicher schreiben
                target_address.copy_from(arg.as_bytes().as_ptr(), arg.len());
                target_address.add(arg.len()).write(0); // null-terminieren für den String

                // Pointer auf neue das Argument in unser Array schreiben
                argv_phys
                    .add(i + 1)
                    .write((args_begin_virt + offset) as *mut u8);

                // Offset neu berechnen
                offset += arg.len() + 1;
            }
        }
        // ============  ============ //

        // App-Image mappen
        pages::pg_mmap_user_app(pid, app_thread.pml4_addr, app);

        return app_thread;
    }

    // Starten des 1. Kernel-Threads (rsp0 zeigt auf den praeparierten Stack)
    // Wird vom Scheduler gerufen, wenn dieser gestartet wird.
    // Alle anderen Threads werden mit 'switch' angestossen
    pub fn start(now: *mut Thread) {
        unsafe {
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
                (*then).pml4_addr.0,
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
        let object: *const Thread = self;

        // sp0 zeigt ans Ende des Speicherblocks, passt somit
        let sp0: *mut u64 = self.kernel_stack.stack_end();
        let sp3: *mut u64 = self.user_stack.stack_end();

        unsafe {
            *sp0 = 0x00DEAD00 as u64; // dummy Ruecksprungadresse

            // Nun sichern wir noch alle Register auf dem Stack
            *sp0.offset(-1) = 0b0000_0000_0010_1011; // SS Register (Segment Selector)
                                                     // 15-3 Bit Index in GDT = *Data*/Code? = 5 = 0b101 | 1Bit TI = GDT = 0| 2Bit PrivLevel = 3 = 0b11
            *sp0.offset(-2) = sp3 as u64; // ESP
            *sp0.offset(-3) = 512 + 2; // EFLAGS
            *sp0.offset(-4) = 0b0000_0000_0010_0011; // CS
                                                     //15-3 Bit Index in GDT = Data/*Code*? = 4 = 0b100 | 1Bit TI = GDT = 0| 2Bit PrivLevel = 3 = 0b11
            *sp0.offset(-5) = consts::USER_CODE_VM_START as u64;
            //*sp0.offset(-5) = (*object).entry as u64;

            *sp0.offset(-6) = object as u64; // RDI

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
        // Interrupt-Stackframe bauen
        self.prepare_user_stack();

        // In den Ring 3 schalten -> Aufruf von '_thread_user_start'
        unsafe {
            _thread_user_start(self.old_rsp0);
        }
    }

    pub fn get_tid(thread_object: *const Thread) -> usize {
        unsafe { (*thread_object).tid }
    }
    pub fn get_pid(thread_object: *const Thread) -> usize {
        unsafe { (*thread_object).pid }
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
            "kickoff_thread, tid={}, old_rsp0 = {:x}, is_kernel_thread: {}, pagetable-addres: 0x{:x}, name={}",
            (*object).tid,
            (*object).old_rsp0,
            (*object).is_kernel_thread,
            (*object).pml4_addr.0,
            (*object).name,
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
