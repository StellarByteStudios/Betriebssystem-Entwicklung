/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Module: pic                                                             ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Descr.: The PIC allows to enable or disable IRQs. This determines       ║
   ║         whether an interruption from a device is forwarded to the cpu   ║
   ║         at all. Even then, activation of the interrupt routine which is ║
   ║         registered in the IDT only occurs if the processor is ready to  ║ 
   ║         respond to interrupts. This depends on the Interrupt Enable IE  ║
   ║         bit in the RFLAGS register. This can be controlled using        ║
   ║         function in the 'cpu.rs' module.                                ║   
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Author: Michael Schoetter, Univ. Duesseldorf, 7.3.2022                  ║
   ╚═════════════════════════════════════════════════════════════════════════╝
*/
use crate::kernel::cpu as cpu;


// IRQ-Nummern von Geraeten
pub const IRQ_TIMER: u32    = 0;     // Programmable Interrupt Timer (PIT)
pub const IRQ_KEYBOARD: u32 = 1;     // Tastatur


const PIC_IMR1: u16   = 0x21;    // interrupt mask register von PIC 1
const PIC_IMR2: u16   = 0xa1;    // interrupt mask register von PIC 2


/**
 Description:
    Enables an IRQ to be to be forwarded to the processor by the PIC. 
    To enable interrupt handling, additionally call `cpu::enable_int()`

 Parameters: \
   `irq` irq to be enabled
*/
pub fn allow (irq: u32) {

   /* Hier muss Code eingefuegt werden */

}


/**
 Description:
    Disables an IRQ to be to be forwarded to the processor by the PIC. 

 Parameters: \
   `irq` irq to be disabled
*/
pub fn forbid (irq: u32) {

    /* Hier muss Code eingefuegt werden */

}


/**
 Description:
    Returns the state (enabled/disabled) in the PIC for the given `irq`
    
 Parameters: \
   `irq` the irq which status is to be checked
    
 Parameters: \
   `true` irq is disabled \
   `false` irq is enabled
*/
pub fn status (irq: u32) -> bool {

    /* Hier muss Code eingefuegt werden */
    return false;

}
 
