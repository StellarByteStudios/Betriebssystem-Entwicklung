
#[no_mangle]
extern "C" fn synced_loop_thread_entry(myself: *mut thread::Thread) {

   /* Hier muss Code eingefuegt werden */

    loop {

        /* Hier muss Code eingefuegt werden */

        cga::setpos(5 + (my_tid-1) * 20, 10);
        delay::delay(1);
        println!("Loop [{}] : {}", my_tid, cnt);

        cnt = cnt + 1;

        delay::delay(10);
    }
}

pub fn init() {

   /* Hier muss Code eingefuegt werden */

}
