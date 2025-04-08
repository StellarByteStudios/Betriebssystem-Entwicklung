use crate::devices::pit;


pub fn delay(ticks: u64) {
    let start_time = pit::get_systime();
    let mut actual_time;

    loop {
        actual_time = pit::get_systime();
        if actual_time - start_time > ticks {
            break;
        }
    }
}
