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
use core::convert::TryFrom;
use usrlib::kernel::syscall::systemcall::{SystemCall, NUM_SYSCALLS};
use crate::kernel::{
    syscall,
    syscall::kfuncs::{
        sys_call_not_implemented::sys_call_not_implemented,
        sys_dump_vmas::sys_dump_vmas,
        sys_mmap_heap_space::sys_mmap_heap_space,
        sys_music::sys_play_song_by_notes,
        sys_paint_picture_on_pos::{sys_draw_pixel, sys_paint_picture_on_pos},
        sys_printing::{
            sys_graphical_print, sys_graphical_print_pos, sys_hello_world, sys_hello_world_print,
            sys_kernel_print, sys_print_apps,
        },
        sys_shell_and_keys::{sys_activate_shell, sys_deactivate_shell, sys_getlastkey},
        sys_simple_getter::{
            sys_get_datetime, sys_get_screen_witdh, sys_get_systime, sys_get_systime_intervall,
        },
        sys_thread_process_management::{
            sys_exit_process, sys_exit_thread, sys_getpid, sys_gettid, sys_kill_process,
            sys_read_process_name, sys_show_threads,
        },
    },
};

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
                //
                sys_gettid as *const _,
                sys_getpid as *const _,
                sys_read_process_name as *const _,
                //
                sys_get_systime as *const _,
                sys_get_screen_witdh as *const _,
                //
                sys_mmap_heap_space as *const _,
                //
                sys_exit_thread as *const _,
                sys_exit_process as *const _,
                sys_kill_process as *const _,
                //
                sys_dump_vmas as *const _,
                sys_graphical_print as *const _,
                sys_graphical_print_pos as *const _,
                sys_paint_picture_on_pos as *const _,
                //
                sys_kernel_print as *const _,
                sys_print_apps as *const _,
                sys_show_threads as *const _,
                //
                sys_play_song_by_notes as *const _,
                //
                sys_draw_pixel as *const _,
                sys_get_datetime as *const _,
                sys_get_systime_intervall as *const _,
                //
                sys_activate_shell as *const _,
                sys_deactivate_shell as *const _,
            ],
        }
    }
}

unsafe impl Send for SyscallFuncTable {}
unsafe impl Sync for SyscallFuncTable {}



// * GPT Funktionen * //
#[no_mangle]
pub unsafe extern "C" fn syscall_disp_new() {
    let syscall_no: usize;
    let arg1: usize;
    let arg2: usize;
    let arg3: usize;
    let arg4: usize;
    let arg5: usize;
    let arg6: usize;

    asm!(
    "mov {}, rax",
    "mov {}, rdi",
    "mov {}, rsi",
    "mov {}, rdx",
    "mov {}, rcx",
    "mov {}, r8",
    "mov {}, r9",
    out(reg) syscall_no,
    out(reg) arg1,
    out(reg) arg2,
    out(reg) arg3,
    out(reg) arg4,
    out(reg) arg5,
    out(reg) arg6,
    );

    let result = dispatch_syscall(syscall_no, arg1, arg2, arg3, arg4, arg5, arg6);

    asm!("mov rax, {}", in(reg) result);
}

fn dispatch_syscall(
    syscall_no: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
    arg6: usize,
) -> usize {
    match SystemCall::try_from(syscall_no) {
        Ok(SystemCall::HelloWorld) => {
            sys_hello_world();
            0
        }

        Ok(SystemCall::HelloWorldWithPrint) => {
            sys_hello_world_print(arg1);
            0
        }

        Ok(SystemCall::GetLastKey) => sys_getlastkey(),

        Ok(SystemCall::GetCurrentThreadID) => sys_gettid(),

        Ok(SystemCall::GetCurrentProcessID) => sys_getpid(),

        Ok(SystemCall::GetCurrentProcessName) => sys_read_process_name(arg1 as *mut u8, arg2),

        Ok(SystemCall::GetSystime) => sys_get_systime(),

        Ok(SystemCall::GetScreenWidth) => sys_get_screen_witdh(),

        Ok(SystemCall::MMapHeapSpace) => sys_mmap_heap_space(arg1, arg2),

        Ok(SystemCall::ExitThread) => {
            sys_exit_thread();
            0
        }

        Ok(SystemCall::ExitProcess) => {
            sys_exit_process();
            0
        }

        Ok(SystemCall::KillProcess) => {
            sys_kill_process(arg1);
            0
        }

        Ok(SystemCall::DumpVMAsOfCurrentProcess) => {
            sys_dump_vmas();
            0
        }

        Ok(SystemCall::GraphicalPrint) => sys_graphical_print(arg1 as *const u8, arg2),

        Ok(SystemCall::GraphicalPrintWithPosition) => sys_graphical_print_pos(arg1, arg2, arg3 as *const u8, arg4),

        Ok(SystemCall::PaintPictureOnPos) => sys_paint_picture_on_pos(arg1, arg2, arg3, arg4, arg5, arg6 as *const u8),

        Ok(SystemCall::KernelPrint) => sys_kernel_print(arg1 as *const u8, arg2),

        Ok(SystemCall::PrintAppNames) => {
            sys_print_apps();
            0
        }

        Ok(SystemCall::PrintRunningThreads) => {
            sys_show_threads();
            0
        }

        Ok(SystemCall::PlaySongOnNoteList) => {
            sys_play_song_by_notes(arg1 as *const u8, arg2);
            0
        }

        Ok(SystemCall::DrawPixel) => {
            sys_draw_pixel(arg1, arg2, arg3);
            0
        }

        Ok(SystemCall::GetDateTime) => {
            sys_get_datetime(arg1);
            0
        }

        Ok(SystemCall::GetPitInterval) => sys_get_systime_intervall(),

        Ok(SystemCall::ActivateShell) => {
            sys_activate_shell();
            0
        }

        Ok(SystemCall::DeactivateShell) => {
            sys_deactivate_shell();
            0
        }

        Err(_) => {
            unsafe { syscall_abort(); }
            0
        }

        _ => {
            unsafe { syscall_abort(); }
            0
        }
    }
}




// * * * * * * * * //







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
