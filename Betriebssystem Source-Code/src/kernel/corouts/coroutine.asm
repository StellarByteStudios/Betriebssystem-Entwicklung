; ╔═════════════════════════════════════════════════════════════════════════╗
; ║ Module: coroutine                                                       ║
; ╟─────────────────────────────────────────────────────────────────────────╢
; ║ Descr.: Assembly function for starting a coroutine and switching between║
; ║         coroutines.                                                     ║
; ╟─────────────────────────────────────────────────────────────────────────╢
; ║ Author: Michael Schoettner, Univ. Duesseldorf, 15.5.2023                ║
; ╚═════════════════════════════════════════════════════════════════════════╝

; exported functions
[GLOBAL _coroutine_start]
[GLOBAL _coroutine_switch]
[GLOBAL _debug_stop]
[GLOBAL _debug_push_stop]

[SECTION .text]
[BITS 64]

;
; fn _coroutine_start (stack_ptr: u64); 
;                     (rdi           );
;
; Start coroutine
;
_coroutine_start:

    mov rsp, rdi

    popf            ; flagregister in Register speichern
    pop rbp         ; Base Pointer holen
    pop rdi         ; Parameter für das Context switchen
    pop rsi

    ; Alle Daten vom Stack holen
    pop rdx
    pop rcx
    pop rbx
    pop rax

    pop r15
    pop r14
    pop r13
    pop r12
    pop r11
    pop r10
    pop r9
    pop r8

    ret

;
; fn _coroutine_switch (now_stack_ptr: *mut u64, then_stack: u64);
;                      (rdi,                     rsi            );
;    
; Switch coroutines
;
;    now_stack_ptr: This is a pointer to 'stack_ptr' in the coroutine struct of
;                   the current coroutine. Here we save RSP
;    then_stack:    This is the value of 'stack_ptr' of the coroutine which we
;                   switch to. This is the RSP saved before.
;
_coroutine_switch:

    ; Allgemeine Register sichern
    push r8
    push r9
    push r10
    push r11
    push r12
    push r13
    push r14
    push r15

    push rax
    push rbx
    push rcx
    push rdx

    push rsi
    push rdi
    push rbp
    pushf

    _debug_push_stop:

    ; Stack kontext wechseln

    mov [rdi], rsp

    mov rsp, rsi


    ; Daten der nächsten Coroutine vom Stack holen

    popf            ; flagregister in Register speichern
    pop rbp         ; Base Pointer holen
    pop rdi         ; Parameter für das Context switchen
    pop rsi

    ; Alle Daten vom Stack holen
    pop rdx
    pop rcx
    pop rbx
    pop rax

    pop r15
    pop r14
    pop r13
    pop r12
    pop r11
    pop r10
    pop r9
    pop r8

    _debug_stop:
    ret