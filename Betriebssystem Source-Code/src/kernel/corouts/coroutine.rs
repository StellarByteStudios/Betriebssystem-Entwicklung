/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Module: coroutine                                                       ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Descr.: Functions for creating, starting, switching and ending coroutines. ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Autor:  Michael Schoettner, 15.05.2023                                  ║
   ╚═════════════════════════════════════════════════════════════════════════╝
*/
use alloc::{boxed::Box, rc::Rc};
use core::borrow::{Borrow, BorrowMut};
use core::ffi::c_void;
use core::fmt::Debug;
use core::ptr;

use crate::consts::{self, STACK_ENTRY_SIZE, STACK_SIZE};
use crate::devices::{cga, kprint};
use crate::kernel::{cpu, stack};

// Description: Assembly functions in 'coroutine.asm'
extern "C" {
    fn _coroutine_start(stack_ptr: usize);
    fn _coroutine_switch(now_stack_ptr: *mut usize, then_stack: usize);
}

/**
   Description: Meta data for a coroutine
*/
#[repr(C)]
pub struct Coroutine {
    cid: usize,
    stack_ptr: usize,      // stack pointer to saved context
    stack: stack::Stack, // memory for stack
    entry: extern "C" fn(*mut Coroutine),
    next: *mut Coroutine,
}

impl Coroutine {
    /**
       Description: Create new coroutine
    */
    pub fn new(my_cid: usize, my_entry: extern "C" fn(*mut Coroutine)) -> Box<Coroutine> {
        let my_stack = stack::Stack::new(STACK_SIZE);
        
        // = = = = = Keine Ahnung wie ich meine Stack ausgeben soll...
        //kprintln!("==== My Stack: {:?}", my_stack);
        //my_stack.fmt("");
        let my_stack_ptr = my_stack.end_of_stack();

        let mut corout = Box::new(Coroutine {
            cid: my_cid,
            stack_ptr: my_stack_ptr,
            stack: my_stack,
            entry: my_entry,
            next: ptr::null_mut(),
        });

        corout.coroutine_prepare_stack();
        corout
    }

    /**
       Description: Start coroutine `cor`
    */
    pub fn start(cor: *mut Coroutine) {
        unsafe{
            // = = = = = = Stackpointer ist bisher immer 0 gewesen
            //kprint!("Coroutine in Start function. Stack_ptr: {:#x}\n", (*cor).stack_ptr);
            _coroutine_start((*cor).stack_ptr as usize);
        }
    }
    
    /**
       Description: Switch from `now` to next coroutine
    */
    pub fn switch2next(now: *mut Coroutine) {
       unsafe{
            //kprintln!("Es wird geswitched");
            //let then = (*now).get_next();
            //let then_stack_pt = (*then).stack_ptr as usize;
            //let now_stack_pt = (*now).stack_ptr.borrow_mut();
            //kprintln!("Now: {:#x}; Then {:#x}", now as usize, then as usize);
            //kprintln!("In Variablen: Now: {:#x}; Then {:#x}", now_stack_pt, then_stack_pt);

            //Self::debug_stop();
            //_coroutine_switch(now_stack_pt, then_stack_pt);
            _coroutine_switch((*now).stack_ptr.borrow_mut(), (*(*now).get_next()).stack_ptr as usize)
       }
    }

    /**
       Description: Return raw pointer to self
    */
    pub fn get_raw_pointer(&mut self) -> *mut Coroutine {
        self
    }

    /**
       Description: Return coroutine id of `cor_object`
    */
    pub fn get_cid(cor_object: *const Coroutine) -> usize {
        unsafe { (*cor_object).cid }
    }

    /**
       Description: Set next pointer of coroutine `self`
    */
    pub fn set_next(&mut self, nxt: *mut Coroutine) {
        self.next = nxt;
    }

    pub fn get_next(&self) -> *mut Coroutine {
        self.next
    }

    /**
      Description: Prepare the stack of a newly created coroutine. It is used to \
                   switch the stack and return to the 'kickoff' function.  \
                   The prepared stack is used in '_coroutine_start' to start the first coroutine.\
                   Starting all other coroutines is done in '_coroutine_switch' where the \
                   prepared stack is used to kickoff a coroutine.
    */
    fn coroutine_prepare_stack(&mut self) {
        let faddr = kickoff as *const ();
        let object: *const Coroutine = self;
        let sp: *mut u64 = self.stack_ptr as *mut u64;

        // The stack should look like a function of a thread was called with one
        // parameter "object" (raw pointer to the Thread struct)
        unsafe {
            *sp = 0x131155 as u64; // dummy return address

            *sp.offset(-1) = faddr as u64; // address of 'kickoff'

            // save all registers on stack
            *sp.offset(-2) = 0; // r8
            *sp.offset(-3) = 0; // r9
            *sp.offset(-4) = 0; // r10
            *sp.offset(-5) = 0; // r11
            *sp.offset(-6) = 0; // r12
            *sp.offset(-7) = 0; // r13
            *sp.offset(-8) = 0; // r14
            *sp.offset(-9) = 0; // r15

            *sp.offset(-10) = 0; // rax
            *sp.offset(-11) = 0; // rbx
            *sp.offset(-12) = 0; // rcx
            *sp.offset(-13) = 0; // rdx

            *sp.offset(-14) = 0; // rsi
            *sp.offset(-15) = object as u64; // rdi -> 1. param. fuer 'kickoff'
            *sp.offset(-16) = 0; // rbp
            *sp.offset(-17) = 0x2; // rflags (IE = 0); interrupts disabled

            // Zum Schluss speichern wir den Zeiger auf den zuletzt belegten
            // Eintrag auf dem Stack in 'context'. Daruber gelangen wir in
            // Coroutine_start an die noetigen Register
            self.stack_ptr = self.stack_ptr - (consts::STACK_ENTRY_SIZE * 17);
        }

        /*
              println!("Prepared Stack: top-address = {:x}", self.stack.get_data() as u64);
              unsafe {
                 println!("  {:x}: {:x}  // dummy raddr", sp as u64, *(sp) as u64);
                 println!("  {:x}: {:x}  // *object", sp.offset(-15) as u64, *(sp.offset(-15)) as u64);
                 println!("  {:x}: {:x}  // kickoff", sp.offset(-1) as u64, *(sp.offset(-1)) as u64);
                 println!("  {:x}: last used ", sp.offset(-17) as u64);
                 println!("");
                 println!("  self.context = {:x}  // context", self.context);
              }
              loop {}
        */
    }
}

/**
   Description: Called indirectly by using the prepared stack in '_coroutine_start' and '_coroutine_switch'
*/
#[no_mangle]
pub extern "C" fn kickoff(object: *mut Coroutine) {
    //kprintln!("kickoff");
    cpu::enable_int(); // interrupts are disabled during coroutine start
    unsafe {
        ((*object).entry)(object);
    }
    loop {}

}
