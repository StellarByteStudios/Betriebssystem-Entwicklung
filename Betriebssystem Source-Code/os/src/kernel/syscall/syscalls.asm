;******************************************************************************
;*                                                                            *
;*                  s y s c a l l s . a s m                                   *
;*                                                                            *
;*----------------------------------------------------------------------------*
;* Beschreibung:    Hier befindet sich alles rund um die low-level Behandlung *
;*                  von Systemaufrufen sowie die Weiterleitung an Rust.       *
;*                                                                            *
;*                  Achtung: '_init_syscalls' muss nach der Initialisieriung  *
;*                  der IDT aufgerufen werden!                                *
;*                                                                            *
;* Autor:           Michael Schoettner, 23.8.2023                             *
;******************************************************************************

[GLOBAL _init_syscalls]       ; Funktion exportieren

[EXTERN _idt]                 ; IDT in 'interrupts.asm' 
[EXTERN syscall_disp]         ; Funktion in Rust, die Syscalls behandelt
[EXTERN syscall_abort]        ; Funktion in Rust, die abbricht, 
                              ; falls der Systemaufruf nicht existiert

[SECTION .text]
[BITS 64]

; Hoechste Funktionsnummer für den System-Aufruf-Dispatcher
; Muss mit NO_SYSCALLS in 'kernel/syscall/mod.rs' konsistent sein!
NO_SYSCALLS: equ 13

; Vektor fuer Systemaufrufe
SYSCALL_TRAPGATE: equ 0x80



;
; Trap-Gate fuer Systemaufrufe einrichten
;
_init_syscalls:
	
	; 
	; Hier muss Code eingefuegt werden
	;
_break_syscallinit
	; an richtige Stelle gehen
	mov rax, _idt ; Läd startadresse
	add rax, 16 * SYSCALL_TRAPGATE

	; adresse von funktion syscall_handler laden
	mov rbx, _syscall_handler

	; = interrupt überschreiben = ;
	mov [rax], bx ; offset 0 .. 15 aka adresse von Funktion syscall-handler
	; adresse in rbx schiften
	shr rbx, 16

	; selector references the 64 bit code segment descriptor in the GDT, see 'boot.asm'
	mov rcx, 0x0000 | 0x8 * 2
	mov [rax+2], cx

	; privilageLevel auf 3: 0x8e00 -> 0xee00
	mov rcx, 0xef00
	mov [rax+4], cx ; e -> interrupt is present und DPL ist 3, e -> 80386 64 bit interrupt gate 


	mov [rax+6], bx ; offset 16 .. 31 aka adresse von Funktion syscall-handler
	; adresse in rbx schiften
	shr rbx, 16

	mov [rax+8], ebx ; offset 32..63 aka adresse von Funktion syscall-handler

	mov rcx, 0x00000000
	mov [rax+12], ecx ; reserved

	ret
	


;
; Handler fuer Systemaufrufe 
;
_syscall_handler:
  ; Alle Register sichern

	;push   rax
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



  ; DS und ES auf dem Stack sichern und danach Kernel-Data Segment in DS und ES setzen
	; kann DS und ES nicht direkt pushen, deswegen Umweg
	mov bx, ds
	push bx
	mov bx, es
	push bx

	mov rbx, 3     ; 3. Eintrag in der GDT
	shl rbx, 3     ; Index beginnt ab 2. Bit

	mov ds, bx
	mov es, bx

	


	; Pruefen, ob die Funktionsnummer nicht zu gross ist
	cmp rax, NO_SYSCALLS
	jge syscall_abort   ; wirft eine Panic, kehrt nicht zurueck

	; Funktionsnummer ist OK -> Rust aufrufen
	call syscall_disp

	; DS und ES wiederherstellen
  
	; 
	; Hier muss Code eingefuegt werden
	;

	pop bx
	mov es, bx
	pop bx
	mov ds, bx
    

  ; Alle Register wiederherstellen

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
	;pop    rax

  iretq
