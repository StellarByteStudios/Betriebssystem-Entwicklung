
// Pruefen, ob wir in einem Grafikmodus sind
// Falls ja setzen der Infos in VGA
fn check_graphics_mode(mbi: u64) -> bool {
    unsafe {
        let ptr = mbi;

        let flags = *(mbi as *mut u32);

        // 12 Bit in Flags zeigt an, ob Framebuffer-Infos vorhanden sind
        if flags & 0x1000 == 0 {
            return false;
        }

        let addr = *((mbi + 88) as *mut u64);
        let pitch = *((mbi + 96) as *mut u32);
        let width = *((mbi + 100) as *mut u32);
        let height = *((mbi + 104) as *mut u32);
        let bpp = *((mbi + 108) as *mut u8);
        vga::VGA::init(addr, pitch, width, height, bpp);
    }
    true
}


#[no_mangle]
pub extern "C" fn startup(mbi: u64) {

    // init allocator

    // Multiboot-Infos für Grafik auslesen, falls vorhanden
    check_graphics_mode(mbi);

    // init interrupts
    
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kprintln!("Panic: {}", info);
    //	kprintln!("{:?}", Backtrace::new());
    loop {}
}
