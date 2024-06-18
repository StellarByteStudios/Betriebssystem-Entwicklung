use crate::kernel::threads::scheduler::{self, Scheduler};
use crate::kernel::threads::thread::Thread;
use alloc::boxed::Box;
use crate::devices::cga;


pub fn init() -> usize {
    // ID holen
    let thread_id: usize = scheduler::next_thread_id();
    // Thread anlegen
    let coop_thread_loop: Box<Thread> = Thread::new(thread_id, thread_loop_entry);
    // Thread beim Scheduler registrieren
    scheduler::Scheduler::ready(coop_thread_loop);
    // ID zurückgeben
    return thread_id;
}


#[no_mangle]
extern "C" fn thread_loop_entry(myself: *mut Thread) {

    // Wie viele Threads laufen vorher? Müssen aus rechnung raus
    let prev_thread_count: usize = 2;

    // Counter parallelen Hochzählen
    let mut counter: usize = 0;
    loop {
        // Ausgabe auf dem Bildschirm
        print_thread_text_on_screen(Thread::get_tid(myself) - prev_thread_count, counter);

        // Hochzählen des Counters
        counter += 1;

        // Weitergeben an nächsten Thread 
        //Scheduler::yield_cpu();
    }
}


// Gibt einen Counter auf dem Screen aus
// Geht aufgrund der Breite nur für 3 Coroutinen
fn print_thread_text_on_screen(id: usize, counter: usize) {
    // X-Position berechnen
    let x_pos = 5 + (id * 25) as u32;

    // Y-Position festsetzen
    let y_pos = 10;

    // X-Position setzen nach id
    cga::setpos(x_pos, y_pos);

    // Counter Ausgeben
    print!("Thread [{}] : {:8}", id, counter);
}
