use core::alloc::Allocator;
use crate::devices::cga;
use crate::kernel::allocator;
use crate::mylib::input::wait_for_return;

use crate::kernel::corouts::coroutine;
use alloc::boxed::Box;
use coroutine::Coroutine;


extern "C" fn coroutine_loop_entry(myself: *mut coroutine::Coroutine) {
    // Counter parallelen Hochzählen
    let mut counter: usize = 0;
    loop{
        // Ausgabe auf dem Bildschirm
        print_coroutine_text_on_screen(Coroutine::get_cid(myself), counter);
        //kprintln!("I am routine {}", Coroutine::get_cid(myself));

        // Hochzählen des Counters
        counter += 1;

        // Weitergeben an nächste Coroutine
        Coroutine::switch2next(myself);
    }

}

pub fn run() {

    //allocator::dump_free_list();
    //wait_for_return();

    cga::clear();

    // Anlegen aller Koroutinen
    let mut corot1  = Coroutine::new(0, coroutine_loop_entry);
    let mut corot2  = Coroutine::new(1, coroutine_loop_entry);
    let mut corot3  = Coroutine::new(2, coroutine_loop_entry);

    // Zyklisches Verketten aller Koroutinen
    corot1.set_next(corot2.as_mut());
    corot2.set_next(corot3.as_mut());
    corot3.set_next(corot1.as_mut());


    allocator::dump_free_list();
    wait_for_return();
    cga::clear();

    // Start der ersten Koroutine
    Coroutine::start(corot1.as_mut());
}


// Gibt einen Counter auf dem Screen aus
// Geht aufgrund der Breite nur für 3 Coroutinen
fn print_coroutine_text_on_screen(id: usize, counter: usize){

    
    // X-Position berechnen
    let x_pos = 5 + (id * 25) as u32;

    // Y-Position festsetzen
    let y_pos = 10;

    // X-Position setzen nach id 
    cga::setpos(x_pos, y_pos);


    // Counter Ausgeben
    print!("Loop [{}] : {:8}", id, counter);

}