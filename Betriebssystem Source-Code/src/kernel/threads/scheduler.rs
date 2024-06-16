/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Module: scheduler                                                       ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Descr.: A basic round-robin scheduler for cooperative threads.          ║
   ║         No priorties supported.                                         ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Autor:  Michael Schoettner, 15.05.2023                                  ║
   ╚═════════════════════════════════════════════════════════════════════════╝
*/

use alloc::boxed::Box;
use core::ptr::{self, null};
use core::sync::atomic::AtomicUsize;
use spin::Mutex;

use crate::devices::cga;
use crate::kernel::cpu;
use crate::kernel::threads::thread;
use crate::mylib::queue;

use super::thread::Thread;

static THREAD_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub fn next_thread_id() -> usize {
    THREAD_ID_COUNTER.fetch_add(1, core::sync::atomic::Ordering::SeqCst)
}

pub static SCHEDULER: Mutex<Scheduler> = Mutex::new(Scheduler::new());

/**
 Description: Return callers thread ID
*/
pub fn get_active_tid() -> usize {
    thread::Thread::get_tid(SCHEDULER.lock().active)
}

pub struct Scheduler {
    active: *mut thread::Thread,
    ready_queue: queue::Queue<Box<thread::Thread>>, // auf die CPU wartende Threads
    inizialized: bool
}

unsafe impl Send for Scheduler {}

impl Scheduler {
    /**
     Description: Create the scheduler
    */
    pub const fn new() -> Self {
        Scheduler {
            active: ptr::null_mut(),
            ready_queue: queue::Queue::new(),
            inizialized: false
        }
    }

    /**
     Description: Start the scheduler. Called only once from 'startup'
    */
    pub fn schedule() {
        let ie = cpu::disable_int_nested();
        kprintln!("Die Queue zum Start {}", SCHEDULER.lock().ready_queue);
        let next_thread = SCHEDULER.lock().ready_queue.dequeue();
        if let Some(that) = next_thread {
            // convert 'next_thread' into raw pointer.
            // Prevents Rust from deleting it too early but we need to manually call 'drop' later
            let raw = Box::into_raw(that);

            // set active reference in SCHEDULER
            SCHEDULER.lock().active = raw;

            // set Scheduler to inizialized
            SCHEDULER.lock().inizialized = true;

            cpu::enable_int_nested(ie);

            // and start this thread
            thread::Thread::start(raw);
        } else {
            panic!("Panic: no thread, cannot start scheduler");
        }
    }

    /**
        Description: Register new thread in ready queue

        Parameters: \
               `that` thread to be registered
    */
    pub fn ready(that: Box<thread::Thread>) {
        SCHEDULER.lock().ready_queue.enqueue(that);
    }

    /**
        Description: Calling thread terminates. Scheduler switches to next thread.
                     (The thread terminating is not in the ready queue.)
    */
    pub fn exit() {

        // Interrupts disablen
        let ie = cpu::disable_int_nested();
        
        let old_active: *mut Thread = SCHEDULER.lock().active;
        kprintln!("Exit Thread {}", Thread::get_tid(old_active));
        kprintln!("Die Queue zum exit {}", SCHEDULER.lock().ready_queue);
        
        // Get next thread from ready queue
        let next = SCHEDULER.lock().ready_queue.dequeue();
        if next.is_none() {
            panic!("Cannot exit thread as there is no other thread to run!");
        }

        // Start next thread
        if let Some(nx) = next {
            let raw = Box::into_raw(nx);
            SCHEDULER.lock().active = raw;
            cpu::enable_int_nested(ie);
            thread::Thread::start(raw);
        }
    }

    /**
        Description: Yield cpu and switch to next thread
    */
    pub fn yield_cpu() {
        let ie = cpu::disable_int_nested();
        // Nachschauen ob der Scheduler überhaupt initialisiert ist
        if !SCHEDULER.lock().inizialized{
            cpu::enable_int_nested(ie);
            return;
        }


        // Aktuel aktiven Thread abspeichern
        let old_active: *mut Thread = SCHEDULER.lock().active;

        // Für den Fall dass durch einen Interupt ein threadwechsel Stattfindet
        // obwohl noch keine Threads angelegt sind
        if old_active.is_null(){
            kprintln!("Yield obwohl noch kein Thread aktiv");
            return;
        }

        // Den aktuellen Thread wieder in die Warteschlange packen
        let old_active_box: Box<Thread> = unsafe{ Box::from_raw(old_active)};
        SCHEDULER.lock().ready_queue.enqueue(old_active_box);
        

        // Nächsten Thread holen
        let next_thread: Option<Box<Thread>> = SCHEDULER.lock().ready_queue.dequeue();

        // Ist das was vernünftiges?
        if next_thread.is_none(){
            panic!("Kein Valider Thread aus Ready-Queue bekommen");
        }

        // Threads switchen
        let next_thread_box = Box::into_raw(next_thread.unwrap());
        SCHEDULER.lock().active = next_thread_box;

        cpu::enable_int_nested(ie);
        Thread::switch(old_active, next_thread_box);
    }

    /**
        Description: Kill thread with given thread id. The thread will just be
                     removed from the ready queue.

        Parameters: \
               `tokill_tid` id of the thread to be killed. Calling thread cannot kill itself.
    */
    pub fn kill(tokill_tid: usize) {
        kprintln!("Killing Thread: {}", tokill_tid);
        kprintln!("Die Queue zum des Kills {}", SCHEDULER.lock().ready_queue);


        // Threadmaske erzeugen um remove gut zu benutzten
        let dummy_thread: Box<Thread> = Thread::new(tokill_tid, Self::dummy_thread_function);

        // Thread löschen
        SCHEDULER.lock().ready_queue.remove(dummy_thread);

        kprintln!("Queue after kill: {}", SCHEDULER.lock().ready_queue);

    }

    // Dummyfunktion die nichts macht
    extern "C" fn dummy_thread_function(thread: *mut Thread){ }
}
 
