;******************************************************************************
;*                                                                            *
;*                  i n t e r r u p t s . a s m                               *
;*                                                                            *
;*----------------------------------------------------------------------------*
;* Beschreibung:    Hier befindet sich alles rund um die low-level Behandlung *
;*                  von Interrupts: IDT, PIC-Initialisierung und Interrupt-   *
;*                  Handler und Aufruf der Interrupt-Behandlung in Rust.      * 
;*                                                                            *
;* Autor:           Michael Schoettner, 23.1.2024                             *
;******************************************************************************

[GLOBAL _init_interrupts]     ; Funktion exportieren
[GLOBAL _idt]                 ; exportieren, brauchen wir in syscalls.asm

[EXTERN int_disp]             ; Funktion in Rust, welche Interrupts behandelt
[EXTERN int_exc_with_error_code] ; Funktion in Rust, welche GPF und PF behandelt

[SECTION .text]
[BITS 64]

; Exportiere Funktion
_init_interrupts:
   call _setup_idt
   call _reprogram_pics
   ret

;
;   Unterbrechungsbehandlung
;

; Spezifischer Kopf der Unterbrechungsbehandlungsroutinen
%macro _wrapper 1
_wrapper_%1:
   ; alle Register sichern
	  push   rax
	  push   rbx
	  push   rcx
	  push   rdx
	  push   rdi
	  push   rsi
	  push   r8
	  push   r9
	  push   r10
	  push   r11
   push   r12
   push   r13
   push   r14
   push   r15

   ; Error-Codes fuer General Protection Fault (GPF)
			; und Page Fault (PF)
			%if %1 == 13 || %1 == 14
	     mov    rdi, [rsp+112] ; error code
	     mov    rdx, [rsp+120] ; rip
	     mov    rsi, [rsp+128] ; cs
	     xor    rax, rax
						mov    al, %1
	     mov    rcx, rax
	     call   int_exc_with_error_code
   %else
	     ; Vektor als Parameter übergeben
	     xor rax, rax
						mov al, %1
	     mov    rdi, rax
	     call   int_disp
			%endif

	  ; Register wiederherstellen
   pop    r15
   pop    r14
   pop    r13
   pop    r12
	  pop    r11
	  pop    r10
	  pop    r9
	  pop    r8
	  pop    rsi
	  pop    rdi
	  pop    rdx
	  pop    rcx
   pop    rbx
	  pop    rax
			
			; Der Error-Code muss manuell abgeraeumt
			%if %1 == 13 || %1 == 14
   add rsp, 8
			%endif

	  ; Fertig!
  	iretq
%endmacro

; ... wird automatisch erzeugt.
%assign i 0
   %rep 256
   _wrapper i
   %assign i i+1
%endrep


;
; Relokation der Eintraege in der IDT und Setzen des IDTR
;
_setup_idt:
	  mov    rax, _wrapper_0

	  ; Bits 0..15 -> ax, 16..31 -> bx, 32..64 -> edx
	  mov    rbx, rax
	  mov    rdx, rax
	  shr    rdx, 32
	  shr    rbx, 16

	  mov    r10, _idt  ; Zeiger auf das aktuelle Interrupt-Gate
	  mov    rcx, 255   ; Zähler
_loop:
	  add    [r10+0], ax
	  adc    [r10+6], bx
	  adc    [r10+8], edx
	  add    r10, 16
	  dec    rcx
	  jge    _loop

	  lidt   [_idt_descr]
	  ret

;
; Neuprogrammierung der PICs (Programmierbare Interrupt-Controller), damit
; alle 15 Hardware-Interrupts nacheinander in der idt liegen.
;
_reprogram_pics:
   mov    al, 0x11   ; ICW1: 8086-Modus mit ICW4
	  out    0x20, al
	  call   _delay
	  out    0xa0, al
	  call   _delay
	  mov    al, 0x20   ; ICW2 Master: IRQ # Offset (32)
	  out    0x21, al
	  call   _delay
	  mov    al, 0x28   ; ICW2 Slave: IRQ # Offset (40)
	  out    0xa1, al
	  call   _delay
	  mov    al, 0x04   ; ICW3 Master: Slaves an IRQs
	  out    0x21, al
	  call   _delay
	  mov    al, 0x02   ; ICW3 Slave: Verbunden mit IRQ2 des Masters
	  out    0xa1, al
	  call   _delay
	  mov    al, 0x03   ; ICW4: 8086-Modus und automatischer EOI
	  out    0x21, al
	  call   _delay
	  out    0xa1, al
	  call   _delay

	  mov    al, 0xff   ; Hardware-Interrupts durch PICs
	  out    0xa1, al   ; ausmaskieren. Nur der Interrupt 2,
	  call   _delay      ; der der Kaskadierung der beiden
	  mov    al, 0xfb   ; PICs dient, ist erlaubt.
	  out    0x21, al

	  ret

;
; Kurze Verzögerung für in/out-Befehle
;
_delay:
   jmp    _L2
_L2:
   ret


[SECTION .data]

;
; Interrupt Descriptor Table mit 256 Einträgen
;
_idt:
%macro _idt_entry 1
   dw  (_wrapper_%1 - _wrapper_0) & 0xffff ; Offset 0 .. 15
   dw  0x0000 | 0x8 * 2 ; Selector zeigt auf den 64-Bit-Codesegment-Deskriptor der GDT (3. Eintrag)
  	dw  0x8e00 ; 8 -> interrupt is present, e -> Interrupt gate
   dw  ((_wrapper_%1 - _wrapper_0) & 0xffff0000) >> 16 ; Offset 16 .. 31
   dd  ((_wrapper_%1 - _wrapper_0) & 0xffffffff00000000) >> 32 ; Offset 32..63
   dd  0x00000000 ; Reserviert
%endmacro

%assign i 0
%rep 256
_idt_entry i
%assign i i+1
%endrep


_idt_descr:
   dw  256*16 - 1    ; 256 Einträge (pro Eintrag 16 Byte)
   dq _idt
