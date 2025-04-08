use core::sync::atomic::{AtomicUsize, Ordering};
use crate::gprintln;

static CALLS: AtomicUsize = AtomicUsize::new(0);
pub fn calculate_fibonacci_rec(x: u64) -> u64 {
    let times_called = CALLS.fetch_add(1, Ordering::SeqCst);
    //if times_called % 10 == 0 {
    //    gprintln!("Fibonacci call called {} times", times_called);
    //}
    
    //return 42;
    if x <= 2 {
        return 1;
    }
    return calculate_fibonacci_rec(x - 1) + calculate_fibonacci_rec(x - 2);
}