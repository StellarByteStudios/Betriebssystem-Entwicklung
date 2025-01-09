;╔═════════════════════════════════════════════════════════════════════════╗
;║ Module: intdispatcher                                                   ║
;╟─────────────────────────────────────────────────────────────────────────╢
;║ Descr.: Here is everything related to the low-level handling of x86     ║
;║         interrupts: IDT, PIC initialization, interrupt handlers, and    ║
;║         invoking interrupt dispatching in Rust; 'int_disp' function     ║
;║         in 'intdispatcher.rs'.                                          ║
;╟─────────────────────────────────────────────────────────────────────────╢
;║ Author: Michael Schoetter, Univ. Duesseldorf, 8.6.2024                  ║
;╚═════════════════════════════════════════════════════════════════════════╝

[GLOBAL _init_interrupts]      ; export init function

[Global _idt] ; expotieren des Labels für die Speicheradresse

[EXTERN int_disp]             ; im Rust function
[EXTERN int_gpf]              ; Funktion in Rust, welche GPF behandelt
[EXTERN int_pagefault]              ; Funktion in Rust, welche PageFault behandelt

[SECTION .text]
[BITS 64]


; Init the IDT and PIC
; This function should be called early during OS startup
_init_interrupts:
   call setup_idt
   call reprogram_pics
   call setup_trap_gate
   ret


; = = = = = Eingene funktion um Trap-Gate anzulegen = = = = = ;
setup_trap_gate:
	; Blatt erklärt nicht, was da rein muss
	ret






; Second-level interrupt handler
%macro _wrapper 1
_wrapper_%1:
   ; save registers
	push   rax
	push   rbx
	push   rcx
	push   rdx
	push   rdi
	push   rbp
	push   rsi
	push   r8
	push   r9
	push   r10
	push   r11
	push   r12
	push   r13
	push   r14
	push   r15

	; do we have a general protection fault?
	%if %1 == 13 
		mov    rdi, [rsp+120] ; error code
		mov    rdx, [rsp+128] ; rip
		mov    rsi, [rsp+136] ; cs
		call    int_gpf
	; do we have a page fault?
	%elif %1 == 14
	    mov    rdi, [rsp+120] ; error code
        mov    rdx, [rsp+128] ; rip
        mov    rsi, [rsp+136] ; cs
        call    int_pagefault
	%else
		; pass the vector as parameter 
		xor rax, rax
		mov al, %1
		mov    rdi, rax
		call   int_disp
	%endif

	; Restore registers
	pop    r15
	pop    r14
	pop    r13
	pop    r12
	pop    r11
	pop    r10
	pop    r9
	pop    r8
	pop    rsi
	pop    rbp
	pop    rdi
	pop    rdx
	pop    rcx
	pop    rbx
	pop    rax

	; done!
	iretq
%endmacro

 
; create 256 first-level handlers, one for each entry in the IDT
%assign i 0
	%rep 256
	_wrapper i
	%assign i i+1
%endrep


;
; Setup IDT
;
setup_idt:
	mov    rax, _wrapper_0

	; Bits 0..15 -> ax, 16..31 -> bx, 32..64 -> edx
	mov    rbx, rax
	mov    rdx, rax
	shr    rdx, 32
	shr    rbx, 16

	mov    r10, _idt   ; pointer to the interrupt gate
	mov    rcx, 255   ; counter (256 IDT entries)
.loop:
	add    [r10+0], ax
	adc    [r10+6], bx
	adc    [r10+8], edx
	add    r10, 16
	dec    rcx
	jge    .loop

	lidt   [idt_descr]
	ret

;
; Reprogramming the Programmable Interrupt Controllers (PICs) 
; so that all 15 hardware interrupts lie sequentially in the IDT
;
reprogram_pics:
	mov    al, 0x11   ; ICW1: 8086 mode with ICW4
	out    0x20, al
	call   delay
	out    0xa0, al
	call   delay
	mov    al, 0x20   ; ICW2 master: IRQ # offset (32)
	out    0x21, al
	call   delay
	mov    al, 0x28   ; ICW2 slave: IRQ # offset (40)
	out    0xa1, al
	call   delay
	mov    al, 0x04   ; ICW3 master: slaves use IRQs
	out    0x21, al
	call   delay
	mov    al, 0x02   ; ICW3 slave: using IRQ2 of master
	out    0xa1, al
	call   delay
	mov    al, 0x03   ; ICW4: 8086 modus and automatic EOI
	out    0x21, al
	call   delay
	out    0xa1, al
	call   delay

	mov    al, 0xff   ; Mask all hardware interrupts
	out    0xa1, al   ; Except IRQ2 is allowed
	call   delay      ; used for cascading
	mov    al, 0xfb   
	out    0x21, al

	ret

;
; Short delay, required for some in/out commands
;
delay:
	jmp    .L2
.L2:
	ret



[SECTION .data]

;
; Interrupt Descriptor Table with 256 entries
;

_idt: 
%macro idt_entry 1
	dw  (_wrapper_%1 - _wrapper_0) & 0xffff ; offset 0 .. 15
	dw  0x0000 | 0x8 * 2 ; selector references the 64 bit code segment descriptor in the GDT, see 'boot.asm'
	dw  0x8e00 ; 8 -> interrupt is present, e -> 80386 64 bit interrupt gate
	dw  ((_wrapper_%1 - _wrapper_0) & 0xffff0000) >> 16 ; offset 16 .. 31
	dd  ((_wrapper_%1 - _wrapper_0) & 0xffffffff00000000) >> 32 ; offset 32..63
	dd  0x00000000 ; reserved
%endmacro

%assign i 0
	%rep 256
	idt_entry i
	%assign i i+1
%endrep


; needed for LIDT instruction, see 'setup_idt'
idt_descr:
	dw  256*16 - 1    ; 256 entries
	dq _idt

