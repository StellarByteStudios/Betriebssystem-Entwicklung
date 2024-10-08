
fn aufgabe3() {
    cga::clear();
    keyboard_irq_demo::run();
 }
 
#[no_mangle]
pub extern "C" fn startup() {
    kprintln!("OS *** is running ...");

    // init allocator

    // init interrupts

    // register keyboard ISR
   
    // CPU enable ints
   
    //aufgabe1();
    //aufgabe2();
    aufgabe3();

    loop {}
}
