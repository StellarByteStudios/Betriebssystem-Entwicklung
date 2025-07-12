use crate::{consts::USER_CODE_HEAP_START, kernel::paging::pages};

#[no_mangle]
pub extern "C" fn sys_mmap_heap_space(pid: usize, size: usize) -> usize {
    // mmap aufrufen
    let error_code = pages::pg_mmap_user_heap(pid, USER_CODE_HEAP_START, size );

    // Fehlerprüfung
    if error_code == 1 {
        return 0;
    }

    return USER_CODE_HEAP_START;
}
