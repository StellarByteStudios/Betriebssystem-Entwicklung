//use crate::devices::pit;

use crate::kernel::syscall::user_api::usr_get_systime;

pub fn delay(ticks: u64) {
    let start_time = usr_get_systime();
    let mut actual_time;

    loop {
        actual_time = usr_get_systime();
        if actual_time - start_time > ticks {
            break;
        }
    }
}
