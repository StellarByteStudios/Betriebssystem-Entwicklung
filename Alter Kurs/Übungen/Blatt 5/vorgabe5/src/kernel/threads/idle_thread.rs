
#[no_mangle]
extern "C" fn idle_thread_entry(myself: *mut thread::Thread) {

   scheduler::set_initialized();

   // ....
}

