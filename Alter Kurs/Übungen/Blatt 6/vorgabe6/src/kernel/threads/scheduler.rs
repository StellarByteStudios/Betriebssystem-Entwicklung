
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


impl Scheduler {

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
  
       /* Hier muss Code eingefuegt werden */
       
	  }
}
