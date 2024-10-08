use crate::devices::cga;
use crate::devices::cga_print;

pub fn run() {
    let mut x: u32 = 0;

    // endless printing numbers 1-10 at fixed position
    loop {


        cga::setpos(0, 6);
        for i in 0..9 {
            println!("{}", i);
        }


        // delay  ...
        for i in 0..10000 {
            x += 1;
        }
    }
}
