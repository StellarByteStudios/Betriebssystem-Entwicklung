use crate::devices::keyboard;

// Wartet bis neue Tastatureingabe kommt
pub fn getchar() -> u8 {
    let mut k: u8;

    loop {
        k = keyboard::get_lastkey();
        if k != 0 {
            break;
        }
    }
    k
}
