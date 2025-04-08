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

[SECTION .text]
[BITS 64]

;
; fn _coroutine_start (stack_ptr: u64); 
;                     (rdi           );
;
; Start coroutine
;
_coroutine_start:
;
; Hier muss Code eingefuegt werden
;



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
;
; Hier muss Code eingefuegt werden
;
