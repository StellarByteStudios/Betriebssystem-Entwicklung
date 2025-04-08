
pub mod pic;
pub mod isr;
pub mod intdispatcher;

// function in 'interrupts.asm'
extern "C" { fn _init_interrupts(); }

// init everything related to interrupt handling
pub fn init() {

    // setup IDT and PIC (in 'interrupts.asm')
    unsafe {
       _init_interrupts();
    }
    
    // initialize the Rust interrupt dispatcher
    intdispatcher::init();
}
