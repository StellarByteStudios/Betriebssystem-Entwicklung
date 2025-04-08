
use core::ptr;


pub fn set_initialized() {
    SCHEDULER.lock().initialized = true;
}

pub struct Scheduler {
    active: *mut thread::Thread,
    ready_queue: queue::Queue<Box<thread::Thread>>, // auf die CPU wartende Threads
    initialized: bool,
}


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
        Description: Check if we can switch from the current running thread to another one. \
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

      /* Hier muss Code eingefuegt werden */
    }   
}
