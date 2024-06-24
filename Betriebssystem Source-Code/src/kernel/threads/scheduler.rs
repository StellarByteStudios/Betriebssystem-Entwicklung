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
use core::borrow::Borrow;
use core::ptr::{self, null, null_mut, read_unaligned};
use core::sync::atomic::AtomicUsize;
use spin::Mutex;

use crate::devices::cga;
use crate::kernel::cpu;
use crate::kernel::threads::thread;
use crate::mylib::queue;

use super::thread::Thread;






/* ========= Stuff für Thread ID ========= */

static THREAD_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);


pub fn next_thread_id() -> usize {
    THREAD_ID_COUNTER.fetch_add(1, core::sync::atomic::Ordering::SeqCst)
}





/* ========= globales Scheduler objekt ========= */

pub static SCHEDULER: Mutex<Scheduler> = Mutex::new(Scheduler::new());

/**
 Description: Return callers thread ID
*/
pub fn get_active_tid() -> usize {
    thread::Thread::get_tid(SCHEDULER.lock().active)
}

pub fn set_initialized() {
    SCHEDULER.lock().initialized = true;
}




/**
 Description: Prepare the blocking of the calling thread (which is the active thread)
*/
pub fn prepare_block() -> (*mut thread::Thread, *mut thread::Thread) {
    SCHEDULER.lock().prepare_block()
}

/**
 Description: Deblock thread `that`. This will result in putting
              `that` into the ready-queue but no thread switching.
*/
pub fn deblock(that: *mut thread::Thread) {
    unsafe {
        SCHEDULER.lock().ready_queue.enqueue(Box::from_raw(that));
    }
}













/* ========= Implementierung der Scheduler-Klasse ========= */


pub struct Scheduler {
    active: *mut thread::Thread,
    ready_queue: queue::Queue<Box<thread::Thread>>, // auf die CPU wartende Threads
    initialized: bool,
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
            initialized: false,
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
            //SCHEDULER.lock().inizialized = true;

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
        //kprintln!("Exit Thread {}", Thread::get_tid(old_active));
        //kprintln!("Die Queue zum exit {}", SCHEDULER.lock().ready_queue);
        
        // Get next thread from ready queue
        let next: Option<Box<Thread>> = SCHEDULER.lock().ready_queue.dequeue();
        if next.is_none() {
            panic!("Cannot exit thread as there is no other thread to run!");
        }

        // Start next thread
        if let Some(nx) = next {
            let raw: *mut Thread = Box::into_raw(nx);
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
        let next_thread_box: *mut Thread = Box::into_raw(next_thread.unwrap());
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
        //kprintln!("Killing Thread: {}", tokill_tid);
        //kprintln!("Die Queue zum des Kills {}", SCHEDULER.lock().ready_queue);


        // Threadmaske erzeugen um remove gut zu benutzten
        let dummy_thread: Box<Thread> = Thread::new(tokill_tid, Self::dummy_thread_function);

        // Thread löschen
        SCHEDULER.lock().ready_queue.remove(dummy_thread);

        //kprintln!("Queue after kill: {}", SCHEDULER.lock().ready_queue);

    }

    // Dummyfunktion die nichts macht
    extern "C" fn dummy_thread_function(thread: *mut Thread){ }

    /**
        Description: Check if we can switch from the current running thread to another one. \
                     If doable prepare everything and return raw pointers to current and next thread. \
                     The switching of threads is done from within the ISR of the PIT, in order to \
                     release the lock of the scheduler. 

        Return: \
               `(current,next)` current thread, next thread (to switch to)
    */
    pub fn prepare_preempt(&mut self) -> (*mut thread::Thread, *mut thread::Thread) {

        //kprintln!("Queue in Preempts {}", self.ready_queue);
        let ie = cpu::disable_int_nested();
        
        // If the scheduler is not initialized, we abort
        if self.initialized == false {
            cpu::enable_int_nested(ie);
            return (ptr::null_mut(), ptr::null_mut());
        }

        // Aktuell laufenden Thread holen
        let old_active: *mut Thread = self.active;

        // Gucken, gibts überhaut einen nächsten?
        if self.ready_queue.is_empty(){
            cpu::enable_int_nested(ie);
            //return (ptr::null_mut(), ptr::null_mut());
            return (old_active, old_active);
        };

        


        // Nachfolgerthread holen
        let next: Option<Box<Thread>> = self.ready_queue.dequeue();

        // Nochmal testen
        if next.is_none(){
            cpu::enable_int_nested(ie);
            //return (ptr::null_mut(), ptr::null_mut());
            return (old_active, old_active);
        }

        self.ready_queue.enqueue(unsafe{ Box::from_raw(old_active)});

        let next_thread_box: *mut Thread =  Box::into_raw(next.unwrap());
        self.active = next_thread_box;

        cpu::enable_int_nested(ie);

        return (old_active, next_thread_box);
    }   





    /**
        Description: Check if we can switch from the current running thread to another one. \
                     If doable prepare everything and return raw pointers to current and next thread. \
                     The switching of threads is done later by calling 'Thread::switch'. \
                     This function is very similar to `prepare_preempt` except the \
                     current thread is not inserted in the `ready_queue` but returned. \
                     The next thread is removed from the `ready_queue` and `active` is set.

        Return: \
               `(current,next)` current thread, next thread (to switch to)
    */
    pub fn prepare_block(&mut self) -> (*mut thread::Thread, *mut thread::Thread) {
  
        //kprintln!("Queue in Preempts {}", self.ready_queue);
        let ie = cpu::disable_int_nested();
        
        // If the scheduler is not initialized, we abort
        if self.initialized == false {
            cpu::enable_int_nested(ie);
            return (ptr::null_mut(), ptr::null_mut());
        }

        // Aktuell laufenden Thread holen
        let old_active: *mut Thread = self.active;

        // Gucken, gibts überhaut einen nächsten?
        if self.ready_queue.is_empty(){
            cpu::enable_int_nested(ie);
            //return (ptr::null_mut(), ptr::null_mut());
            return (old_active, old_active);
        };

        


        // Nachfolgerthread holen
        let next: Option<Box<Thread>> = self.ready_queue.dequeue();

        // Nochmal testen
        if next.is_none(){
            cpu::enable_int_nested(ie);
            //return (ptr::null_mut(), ptr::null_mut());
            return (old_active, old_active);
        }

        //self.ready_queue.enqueue(unsafe{ Box::from_raw(old_active)});

        let next_thread_box: *mut Thread =  Box::into_raw(next.unwrap());
        self.active = next_thread_box;

        cpu::enable_int_nested(ie);

        return (old_active, next_thread_box);
    } 
}
 
