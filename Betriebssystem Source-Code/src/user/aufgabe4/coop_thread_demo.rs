use crate::kernel::threads::thread;

#[no_mangle]
extern "C" fn coop_demo_thread_entry(myself: *mut thread::Thread) {

    /* Hier muss Code eingefuegt werden */
}

/**
 Return thread id of created thread
*/
pub fn init() -> usize {
    /* Hier muss Code eingefuegt werden */
    return 0;
}
