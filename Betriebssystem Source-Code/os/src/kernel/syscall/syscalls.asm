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


[EXTERN NO_SYSCALLS]          ; Holt sich die Anzahl aus dem Rust-Code (Achtung! nur Adresse)

[SECTION .bss]
syscall_count: resq 1         ; 8 Byte (64 Bit) für usize (Variable die nach Init genutzt werden soll)

[SECTION .text]
[BITS 64]

; Hoechste Funktionsnummer für den System-Aufruf-Dispatcher
; Muss mit NO_SYSCALLS in 'kernel/syscall/mod.rs' konsistent sein!
;NO_SYSCALLS: equ 14


; Vektor fuer Systemaufrufe
SYSCALL_TRAPGATE: equ 0x80



;
; Trap-Gate fuer Systemaufrufe einrichten
;
_init_syscalls:
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

	; Syscall-Anzahl laden und lokal speichern
	mov rax, [NO_SYSCALLS]   ; Laufzeitwert aus Rust laden
    mov [syscall_count], rax ; lokal ablegen

	ret
	


;
; Handler fuer Systemaufrufe 
;
_syscall_handler:
  ; Alle Register sichern außer rax
  
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
	; cmp rax, NO_SYSCALLS ; Nicht mehr mit der Konstante vergleichen
	; jge syscall_abort   ; wirft eine Panic, kehrt nicht zurueck

	; vergleich mit lokaler variable
	mov rbx, [syscall_count]
    cmp rax, rbx
    jge syscall_abort

	; Funktionsnummer ist OK -> Rust aufrufen
	call syscall_disp

	; DS und ES wiederherstellen
  

	pop bx
	mov es, bx
	pop bx
	mov ds, bx
    

  ; Alle Register wiederherstellen außer rax

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

    iretq
