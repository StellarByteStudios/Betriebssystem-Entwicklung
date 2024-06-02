use core::alloc::Allocator;
use crate::kernel::allocator;
use crate::mylib::input::wait_for_return;

use crate::kernel::corouts::coroutine;
use alloc::boxed::Box;
use coroutine::Coroutine;


extern "C" fn coroutine_loop_entry(myself: *mut coroutine::Coroutine) {
    loop{
        kprint!("I am routine {}", Coroutine::get_cid(myself));
        Coroutine::switch2next(myself);
    }

}

pub fn run() {

    allocator::dump_free_list();
    wait_for_return();

    // Anlegen aller Koroutinen
    let mut corot1  = Coroutine::new(1, coroutine_loop_entry);
    let mut corot2  = Coroutine::new(2, coroutine_loop_entry);

    // Zyklisches Verketten aller Koroutinen
    corot1.set_next(corot2.as_mut());
    corot2.set_next(corot1.as_mut());

    allocator::dump_free_list();
    wait_for_return();

    // Start der ersten Koroutine
    Coroutine::start(corot1.as_mut());
}
