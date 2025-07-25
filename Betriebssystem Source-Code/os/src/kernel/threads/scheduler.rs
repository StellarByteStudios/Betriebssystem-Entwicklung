/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Module: scheduler                                                       ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Descr.: A basic round-robin scheduler for cooperative threads.          ║
   ║         No priorties supported.                                         ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Autor:  Michael Schoettner, HHU, 14.6.2024                              ║
   ╚═════════════════════════════════════════════════════════════════════════╝
*/
use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec::Vec,
};
use core::{ptr, ptr::null_mut, sync::atomic::AtomicUsize};

use spin::Mutex;

use crate::{
    boot::appregion::AppRegion,
    devices::cga,
    kernel::{
        cpu,
        paging::physical_addres::PhysAddr,
        processes::process_handler::create_fresh_process,
        threads::{idle_thread, queue::Queue, scheduler, thread, thread::Thread},
    },
};

static THREAD_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub fn next_thread_id() -> usize {
    THREAD_ID_COUNTER.fetch_add(1, core::sync::atomic::Ordering::SeqCst)
}

pub static SCHEDULER: Mutex<Scheduler> = Mutex::new(Scheduler::new());

/**
 Description: Return callers thread ID
*/
pub fn get_active_tid() -> usize {
    return get_active().tid;
}

pub fn get_active_pid() -> usize {
    // get_active Methode einfach ersetzt und reinkopiert
    let active_pid;
    let irq = cpu::disable_int_nested();
    unsafe {
        let active = SCHEDULER.lock().active;
        active_pid = (*active).pid;
    }
    cpu::enable_int_nested(irq);

    return active_pid;
}

/**
   Threads beenden
*/
pub fn exit_current_thread() {
    Scheduler::exit()
}

/**
 Description: Get active thread (used before calling 'block')
*/
pub fn get_active() -> Box<thread::Thread> {
    let act;
    let irq = cpu::disable_int_nested();
    unsafe {
        let a = SCHEDULER.lock().active;
        act = Box::from_raw(a);
    }
    cpu::enable_int_nested(irq);
    act
}

/*****************************************************************************
 * Funktion:        spawn_kernel                                             *
 *---------------------------------------------------------------------------*
 * Beschreibung:    Kernel-Prozess mit Idle-Thread erzeugen und im Scheduler *
 *                  registrieren.                                            *
 *****************************************************************************/
pub fn spawn_kernel() {
    // Neuen Prozess anlegen
    let idle_pid = create_fresh_process("Idle-Prozess");

    // Idle-Thread mit Pid anleggen
    let idle_thread = idle_thread::init(idle_pid);

    // Thread dem Scheduler geben
    Scheduler::ready(idle_thread);
}

/*****************************************************************************
 * Funktion:        spawn                                                    *
 *---------------------------------------------------------------------------*
 * Beschreibung:    Einen neuen Prozess mit dem Haupt-Thread erzeugen und    *
 *                  im Scheduler registrieren.                               *
 *                                                                           *
 * Parameter:       app    Code-Image fuer den neuen Prozess                 *
 *****************************************************************************/
pub fn spawn_app(app: AppRegion, args: Vec<String>) {
    // Neuen Prozess anlegen
    let new_pid = create_fresh_process(app.file_name.as_str());

    // Idle-Thread mit Pid anleggen
    let new_app_thread = thread::Thread::new_app_thread(app, new_pid, &args);

    // Thread dem Scheduler geben
    Scheduler::ready(new_app_thread);
}

/**
 Description: Set initialized flag
*/
pub fn set_initialized() {
    SCHEDULER.lock().initialized = true;
}

pub struct Scheduler {
    active: *mut thread::Thread,
    ready_queue: Queue<Box<thread::Thread>>, // auf die CPU wartende Threads
    next_thread_id: u64,
    initialized: bool,
}

// Notwendig, da sonst der Compiler 'SCHEDULER' als nicht akzeptiert
unsafe impl Send for Scheduler {}

impl Scheduler {
    // Scheduler mit Ready-Queue anlegen
    pub const fn new() -> Self {
        Scheduler {
            active: ptr::null_mut(),
            next_thread_id: 0,
            ready_queue: Queue::new(),
            initialized: false,
        }
    }

    /**
     Description: Start the scheduler. Called only once from 'startup'
    */
    pub fn schedule() {
        let next_thread = SCHEDULER.lock().ready_queue.dequeue();
        if let Some(that) = next_thread {
            // convert 'next_thread' into raw pointer.
            // Prevents Rust from deleting it too early but we need to manually call 'drop' later
            let raw = Box::into_raw(that);

            // set active reference in SCHEDULER
            SCHEDULER.lock().active = raw;

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

    pub fn print_ready_queue() {
        vprintln!("{}", SCHEDULER.lock().ready_queue);
    }
    pub fn kill_thread_with_pid(pid: usize) {
        // Idlethread nicht killen
        if pid == 0 {
            return;
        }

        // Scheduler locken
        let mut sched = SCHEDULER.lock();

        // Referenz auf ready Queue
        let queue: &mut Queue<Box<Thread>> = &mut sched.ready_queue;

        // Neue Queue mit den übrigen Threads
        let mut temp_queue = Queue::new();

        // Durch alle Threads durchgehen
        while let Some(thread) = queue.dequeue() {
            if thread.pid == pid {
                // Thread nicht wieder hinzugefügt
            } else {
                temp_queue.enqueue(thread);
            }
        }

        // Ready Queue mit neu befüllten queue überschreiben
        *queue = temp_queue;
    }

    /**
    Description: Calling thread terminates. Scheduler switches to next thread.
                 (The thread terminating is not in the ready queue.)
    */

    pub fn exit() {
        // Get next thread from ready queue
        let next = SCHEDULER.lock().ready_queue.dequeue();
        if next.is_none() {
            panic!("Cannot exit thread as there is no other thread to run!");
        }

        // Start next thread
        if let Some(nx) = next {
            let raw = Box::into_raw(nx);
            SCHEDULER.lock().active = raw;
            thread::Thread::start(raw);
        }
    }

    /**
        Description: Yield cpu and switch to next thread
    */
    pub fn yield_cpu() {
        // Get next thread from ready queue
        let next = SCHEDULER.lock().ready_queue.dequeue();
        if next.is_none() {
            return;
        }

        let that = SCHEDULER.lock().active;

        // Re-insert current thread into ready queue
        let bx;
        unsafe {
            // convert raw-Pointer back to Box<Thread>
            bx = Box::from_raw(that);
        }
        SCHEDULER.lock().ready_queue.enqueue(bx);

        // Switch thread
        if let Some(nx) = next {
            let raw = Box::into_raw(nx);
            SCHEDULER.lock().active = raw;
            thread::Thread::switch(that, raw);
        }
    }

    /**
        Description: Kill thread with given thread id. The thread will just be
                     removed from the ready queue.

        Parameters: \
               `tokill_tid` id of the thread to be killed. Calling thread cannot kill itself.
    */
    pub fn kill(tokill_tid: usize) -> bool {
        // Threadmaske erzeugen um remove gut zu benutzten
        let dummy_thread: Box<thread::Thread> = thread::Thread::new_name(
            tokill_tid,
            Self::dummy_thread_function,
            false,
            "Dummy-Thread".to_string(),
        );

        // Thread löschen
        let success: bool = SCHEDULER.lock().ready_queue.remove(dummy_thread);

        // War das Löschen erfolgreich?
        return success;
    }
    // Dummyfunktion die nichts macht
    extern "C" fn dummy_thread_function() {}

    /**
        Description: This function is only called from the ISR of the PIT. \
                     Check if we can switch from the current running thread to another one. \
                     If doable prepare everything and return raw pointers to current and next thread. \
                     The switching of threads is done from within the ISR of the PIT, in order to \
                     release the lock of the scheduler.

        Return: \
               `(current,next)` current thread, next thread (to switch to)
    */
    pub fn prepare_preempt(&mut self) -> (*mut thread::Thread, *mut thread::Thread) {
        // If the scheduler is not initialized, we abort
        if self.initialized == false {
            return (ptr::null_mut(), ptr::null_mut());
        }

        // Check if there is a thread in the ready queue, if not we abort
        let next = self.ready_queue.dequeue();
        if next.is_none() {
            return (ptr::null_mut(), ptr::null_mut());
        }

        // If we are here, we can preempt

        // Insert the current running thread into the ready qeueue
        let current = self.active;
        unsafe {
            self.ready_queue.enqueue(Box::from_raw(current));
        }

        // Set active thread in scheduler and return (current, next)
        if let Some(nx) = next {
            let raw_next = Box::into_raw(nx);
            self.active = raw_next;
            (current, raw_next)
        } else {
            panic!("prepare_preempt failed.");
        }

        // Interrupts werden in Thread_switch in thread.asm wieder zugelassen
        //
    }
}
