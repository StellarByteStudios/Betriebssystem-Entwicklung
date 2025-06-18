/*****************************************************************************
 *                                                                           *
 *                  s y s c a l l _ d i s p a t c h e r                      *
 *                                                                           *
 *---------------------------------------------------------------------------*
 * Beschreibung:    Alle Systemaufrufe landen vom Assembler-Coder hier und   *
 *                  werden anhand der Funktionsnummerund der Funktions-      *
 *                  tabelle weitergeleitet.                                  *
 *                                                                           *
 * Autor:           Stefan Lankes, RWTH Aachen                               *
 *                  Michael Schoettner, 23.10.2024, modifiziert              *
 *****************************************************************************/
use core::arch::{asm, naked_asm};

use usrlib::kernel::syscall::NUM_SYSCALLS;

use crate::kernel::{
    syscall,
    syscall::kfuncs::{
        sys_call_not_implemented::sys_call_not_implemented,
        sys_dump_vmas::sys_dump_vmas,
        sys_getlastkey::sys_getlastkey,
        sys_mmap_heap_space::sys_mmap_heap_space,
        sys_paint_picture_on_pos::sys_paint_picture_on_pos,
        sys_printing::{
            sys_graphical_print, sys_graphical_print_pos, sys_hello_world, sys_hello_world_print,
            sys_kernel_print,
        },
        sys_simple_getter::{sys_get_screen_witdh, sys_get_systime},
        sys_thread_process_management::{
            sys_exit_process, sys_exit_thread, sys_getpid, sys_gettid, sys_kill_process,
            sys_read_process_name,
        },
    },
};
use crate::kernel::syscall::kfuncs::sys_music::sys_play_song_by_notes;

// Anzahl an Systemaufrufen
// Muss mit NO_SYSCALLS in 'kernel/syscall/syscalls.asm' konsistent sein!
#[no_mangle] // No mangle, weil ich versuche die Variable direkt in Assembler zu benutzen
pub static NO_SYSCALLS: usize = NUM_SYSCALLS;
pub const NO_SYSCALLS_CONST: usize = NUM_SYSCALLS; // NUM_SYSCALLS aus der userlib

extern "C" {
    fn _init_syscalls();
}

// IDT-Eintrag fuer Systemaufrufe einrichten (in 'syscalls.asm')
pub fn init() {
    unsafe {
        _init_syscalls();
    }
}

#[no_mangle]
pub static SYSCALL_FUNCTABLE: SyscallFuncTable = SyscallFuncTable::new();

#[repr(align(64))]
#[repr(C)]
pub struct SyscallFuncTable {
    handle: [*const usize; NO_SYSCALLS_CONST],
}

impl SyscallFuncTable {
    pub const fn new() -> Self {
        SyscallFuncTable {
            handle: [
                sys_hello_world as *const _,
                sys_hello_world_print as *const _,
                sys_getlastkey as *const _,

                sys_gettid as *const _,
                sys_getpid as *const _,
                sys_read_process_name as *const _,

                sys_get_systime as *const _,
                sys_get_screen_witdh as *const _,

                sys_mmap_heap_space as *const _,

                sys_exit_thread as *const _,
                sys_exit_process as *const _,
                sys_kill_process as *const _,

                sys_dump_vmas as *const _,
                sys_graphical_print as *const _,
                sys_graphical_print_pos as *const _,
                sys_paint_picture_on_pos as *const _,

                sys_kernel_print as *const _,
                sys_call_not_implemented as *const _,
                sys_call_not_implemented as *const _,
                sys_call_not_implemented as *const _,

                sys_play_song_by_notes as *const _,
            ],
        }
    }
}

unsafe impl Send for SyscallFuncTable {}
unsafe impl Sync for SyscallFuncTable {}

/*****************************************************************************
 * Funktion:        syscall_disp                                             *
 *---------------------------------------------------------------------------*
 * Beschreibung:    Wenn ein System-Aufruf ueber int 0x80 ausgeloest wurde   *
 *                  ruft der Assembler-Handler '_syscall_handler' diese      *
 *                  Rust-Funktion auf. Das Sichern und Wiederherstellen der  *
 *                  Register wird schon in Assembler erledigt.               *
 *****************************************************************************/
#[naked]
#[no_mangle]
pub unsafe extern "C" fn syscall_disp() {
    naked_asm!(
						"call [{syscall_functable}+8*rax]",
						"ret",
      syscall_functable = sym SYSCALL_FUNCTABLE);
}

/*****************************************************************************
 * Funktion:        syscall_abort                                            *
 *---------------------------------------------------------------------------*
 * Beschreibung:    Falls eine unbekannte Funktionsnummer verwendet wurde,   *
 *                  ruft der Assembler-Code diese Funktion auf, um eine      *
 *                  panic auszuloesen.                                       *
 *****************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn syscall_abort() {
    let sys_no: u64;

    asm!(
        "mov {}, rax", out(reg) sys_no
    );

    panic!("Systemaufruf mit Nummer {} existiert nicht!", sys_no);
}
