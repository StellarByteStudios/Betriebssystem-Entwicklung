use crate::mylib;

#[no_mangle]
pub extern "C" fn sys_getlastkey() -> u64 {
    let key: u8 = mylib::input::getch();
    return key as u64;
}
