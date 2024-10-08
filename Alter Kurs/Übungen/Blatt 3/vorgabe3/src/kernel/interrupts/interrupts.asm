;╔═════════════════════════════════════════════════════════════════════════╗
;║ Module: intdispatcher                                                   ║
;╟─────────────────────────────────────────────────────────────────────────╢
;║ Descr.: Here is everything related to the low-level handling of x86     ║
;║         interrupts: IDT, PIC initialization, interrupt handlers, and    ║
;║         invoking interrupt dispatching in Rust; 'int_disp' function     ║
;║         in 'intdispatcher.rs'.                                          ║
;╟─────────────────────────────────────────────────────────────────────────╢
;║ Author: Michael Schoetter, Univ. Duesseldorf, 6.7.2022                  ║
;╚═════════════════════════════════════════════════════════════════════════╝

[GLOBAL _init_interrupts]      ; export init function

[EXTERN int_disp]             ; im Rust function

[SECTION .text]
[BITS 64]


; Init the IDT and PIC
; This function should be called early during OS startup
_init_interrupts:
   call setup_idt
   call reprogram_pics
   ret


; First-level interrupt handler code template for following macro
%macro wrapper 1
wrapper_%1:
	push   rbp
	mov    rbp, rsp
	push   rax
	mov    al, %1
	jmp    wrapper_body
%endmacro

; create 256 first-level handlers, one for each entry in the IDT
%assign i 0
%rep 256
wrapper i
%assign i i+1
%endrep

; Second-level interrupt handler
wrapper_body:
	; save registers
	push   rcx
	push   rdx
	push   rdi
	push   rsi
	push   r8
	push   r9
	push   r10
	push   r11

	; get vector number (8 Bit)
	and    rax, 0xff

	; pass the vector as parameter to the Rust function
	mov    rdi, rax
	call   int_disp

	; Restore registers
	pop    r11
	pop    r10
	pop    r9
	pop    r8
	pop    rsi
	pop    rdi
	pop    rdx
	pop    rcx

	; and finally restore registers saved in the first-level handler
	pop    rax
	pop    rbp

	; done!
	iretq


;
; Setup IDT
;
setup_idt:
	mov    rax, wrapper_0

	; Bits 0..15 -> ax, 16..31 -> bx, 32..64 -> edx
	mov    rbx, rax
	mov    rdx, rax
	shr    rdx, 32
	shr    rbx, 16

	mov    r10, idt   ; pointer to the interrupt gate
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

idt:
%macro idt_entry 1
	dw  (wrapper_%1 - wrapper_0) & 0xffff ; offset 0 .. 15
	dw  0x0000 | 0x8 * 2 ; selector references the 64 bit code segment descriptor in the GDT, see 'boot.asm'
	dw  0x8e00 ; 8 -> interrupt is present, e -> 80386 64 bit interrupt gate
	dw  ((wrapper_%1 - wrapper_0) & 0xffff0000) >> 16 ; offset 16 .. 31
	dd  ((wrapper_%1 - wrapper_0) & 0xffffffff00000000) >> 32 ; offset 32..63
	dd  0x00000000 ; reserved
%endmacro

%assign i 0
%rep 256
idt_entry i
%assign i i+1
%endrep


; needed for LIDT instruction, see 'setup_idt'
idt_descr:
	dw  256*8 - 1    ; 256 entries
	dq idt

