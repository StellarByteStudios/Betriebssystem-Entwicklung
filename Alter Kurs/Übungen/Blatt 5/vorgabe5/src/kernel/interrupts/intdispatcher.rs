
use core::sync::atomic::{AtomicUsize, Ordering};

/**
 Description:
    This function is the main interrupt dispatcher in Rust.
    It is called from `interrupts.asm`

 Parameters: \
   `vector` vector number of interrupt
*/
#[no_mangle]
pub extern "C" fn int_disp(vector: u32) {
    if is_initialized() == false {
        panic!("int_disp called but INT_VECTORS not initialized.");
    }

    // 'report' calls registered ISR
    if report(vector as usize) == false {
        kprint!("Panic: unexpected interrupt nr = {}", vector);
        kprint!(" - processor halted.");
        cpu::halt();
    }
}

static mut INT_VECTORS: Option<IntVectors> = None;
static INT_VECTORS_INITIALIZED: AtomicUsize = AtomicUsize::new(0);

// used in 'int_disp' to check if interrupt dispatching tables has been initialized
fn is_initialized() -> bool {
    let v = INT_VECTORS_INITIALIZED.load(Ordering::SeqCst);
    if v == 0 {
        return false;
    }
    return true;
}

/**
 Description:
    Initializing the ISR map with MAX_VEC_NUM default ISRs.
    Specific ISRs can be overwritten by calling `assign`.
*/
pub fn init() {
    kprintln!("INT_VECTORS: init");
    unsafe {
        INT_VECTORS = Some(IntVectors { map: Vec::new() });

        for _ in 0..MAX_VEC_NUM {
            INT_VECTORS
                .as_mut()
                .unwrap()
                .map
                .push(Box::new(isr::Default));
        }
    }
    INT_VECTORS_INITIALIZED.store(1, Ordering::SeqCst);
}

