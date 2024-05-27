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

    // Anlegen aller Koroutinen
    let mut corot1  = Coroutine::new(1, coroutine_loop_entry);
    let mut corot2  = Coroutine::new(2, coroutine_loop_entry);

    // Zyklisches Verketten aller Koroutinen
    corot1.set_next(corot2.as_mut());
    corot2.set_next(corot1.as_mut());

    // Start der ersten Koroutine
    Coroutine::start(corot1.as_mut());
}
