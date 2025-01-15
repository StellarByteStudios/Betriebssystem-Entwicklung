/*****************************************************************************
 *                                                                           *
 *                  u s e r _ a p i                                          *
 *                                                                           *
 *---------------------------------------------------------------------------*
 * Beschreibung:    Alle Systemaufrufe landen vom Assembler-Coder hier und   *
 *                  werden anhand der Funktionsnummerund der Funktions-      *
 *                  tabelle weitergeleitet.                                  *
 *                                                                           *
 * Autor:           Stefan Lankes, RWTH Aachen University                    *
 *                  Licensed under the Apache License, Version 2.0 or        *
 *                  the MIT license, at your option.                         *
 *                                                                           *
 *                  Michael Schoettner, 14.9.2023, modifiziert               *
 *****************************************************************************/

use core::arch::asm;

// Funktionsnummern aller Systemaufrufe
pub const SYSNO_HELLO_WORLD: usize = 0;
pub const SYSNO_HELLO_WORLD_PRINT: usize = 1;
pub const SYSNO_GET_LAST_KEY: usize = 2;
pub const SYSNO_GET_THREAD_ID: usize = 3;
pub const SYSNO_WRITE: usize = 4;
pub const SYSNO_READ: usize = 5;
pub const SYSNO_GET_SYSTIME: usize = 6;

pub const SYSNO_GRAPHICAL_PRINT: usize = 7;
/*
 * Hier muss Code eingefuegt werden
 */

pub fn usr_hello_world() {
    syscall0(SYSNO_HELLO_WORLD as u64);
}

pub fn usr_hello_world_print(arg1: u64) {
    syscall1(SYSNO_HELLO_WORLD_PRINT as u64, arg1);
}

pub fn usr_getlastkey() -> u64 {
    return syscall0(SYSNO_GET_LAST_KEY as u64);
}

pub fn usr_gettid() -> u64 {
    return syscall0(SYSNO_GET_THREAD_ID as u64);
}

pub fn usr_write(buff: *const u8, len: u64) -> u64 {
    return syscall2(SYSNO_WRITE as u64, buff as u64, len);
}

pub fn usr_read(buff: *mut u8, len: u64) -> u64 {
    return syscall2(SYSNO_READ as u64, buff as u64, len);
}

pub fn usr_get_systime() -> u64 {
    return syscall0(SYSNO_GET_SYSTIME as u64);
}

pub fn usr_graphical_print(buff: *const u8, len: u64){
    syscall2(SYSNO_GRAPHICAL_PRINT as u64, buff as u64, len);
}

/*
 * Hier muss Code eingefuegt werden
 */

#[inline(always)]
#[allow(unused_mut)]
pub fn syscall0(arg0: u64) -> u64 {
    let mut ret: u64;
    unsafe {
        asm!("int 0x80", // ===== Irgendwie lande ich hier immer in Int 13 (General Prot Fault)
            inlateout("rax") arg0 => ret,
            options(preserves_flags, nostack)
        );
    }
    ret
}

/*
 * Hier muss Code eingefuegt werden
 */

/*  Parameterreihenfolge
1. rdi
2. rsi
3. rdx
4. rcx
5. r8
6. r9
*/

#[inline(always)]
#[allow(unused_mut)]
pub fn syscall1(arg0: u64, arg1: u64) -> u64 {
    let mut ret: u64;
    unsafe {
        asm!(
            "int 0x80", // Software interrupt for syscalls on x86_64 Linux
            in("rax") arg0,     // Load arg0 into rax (typically the syscall number)
            in("rdi") arg1,     // Load arg1 into rdi (first syscall parameter)
            lateout("rax") ret, // Store return value from syscall in ret
            options(preserves_flags, nostack)
        );
    }
    ret
}

#[inline(always)]
#[allow(unused_mut)]
pub fn syscall2(arg0: u64, arg1: u64, arg2: u64) -> u64 {
    let mut ret: u64;
    unsafe {
        asm!(
            "int 0x80",           // Software interrupt for syscalls on x86_64 Linux
            in("rax") arg0,       // Load arg0 into rax (typically the syscall number)
            in("rdi") arg1,       // Load arg1 into rdi (first syscall parameter)
            in("rsi") arg2,       // Load arg2 into rsi (second syscall parameter)
            lateout("rax") ret,   // Store return value from syscall in ret
            options(preserves_flags, nostack)
        );
    }
    ret
}
