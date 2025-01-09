;******************************************************************************
;*                        B O O T . A S M                                     *
;*----------------------------------------------------------------------------*
;* Die Funktion 'kmain' ist der Eintrittspunkt des eigentlichen Systems. Die  *
;* Umschaltung in den 32-bit 'Protected Mode' ist bereits durch grub erfolgt. *
;* Hier wird alles vorbereitet, damit so schnell wie möglich mit der Aus-     *
;* führung von Rust-Code im 64-bit 'Long Mode' begonnen werden kann.          *
;*                                                                            *
;* Unser Image wird durch grub ab 1 MB geladen. Die PageTables brauchen auch  *
;* noch 1 MB, sodass der Rust Code entsprechend oberhalb von oberhalb von 4 MB*
;* oder hoeher liegt.                                                         *
;*                                                                            *
;* Der Assembler-Code stellt einen Stack mit 64 KB zur Verfuegung und sollte  *
;* Rust bald durch einen groesseren Stack ersetzt werden.                     *
;*                                                                            *
;* Autor: Michael Schoettner, Uni Duesseldorf, 30.10.2023                     *
;******************************************************************************

;
;   Konstanten
;

; Auskommentieren, um im Grafikmodus zu booten
;%define TEXT_MODE 

 
; Lade-Adresse des Kernels, muss mit der Angabe in 'sections' konsistent sein!
KERNEL_START: equ 0x100000

; Stack fuer die main-Funktion
STACKSIZE: equ 65536

; 254 GB maximale RAM-Groesse fuer die Seitentabelle
MAX_MEM: equ 254

; Speicherplatz fuer die Seitentabelle
[GLOBAL _pagetable_start]
_pagetable_start:  equ 0x103000    ; 1 MB + 12 KB

[GLOBAL _pagetable_end]
_pagetable_end:  equ 0x200000      ;  = 2 MB

;
;   System
;

; Von uns bereitgestellte Funktionen
[GLOBAL _start]

; Adresse des TSS abfragen
[GLOBAL _get_tss_address]

; Kernel-Stack im TSS setzen (beim Thread-Wechsel)
[GLOBAL _tss_set_rsp0]

; Rust-Einstiegsfunktion die am Ende des Assembler-Codes aufgerufen werden
[EXTERN kmain]


; Vom Compiler bereitgestellte Adressen
[EXTERN ___BSS_START__]
[EXTERN ___BSS_END__]

; In 'sections' definiert
[EXTERN ___KERNEL_DATA_START__]
[EXTERN ___KERNEL_DATA_END__]

; Multiboot constants
MULTIBOOT_HEADER_MAGIC:           equ 0x1BADB002
MULTIBOOT_ARCHITECTURE_I386:      equ 0
MULTIBOOT_HEADER_TAG_OPTIONAL:    equ 1
MULTIBOOT_HEADER_TAG_FRAMEBUFFER: equ 5
MULTIBOOT_HEADER_TAG_END:         equ 0
MULTIBOOT_PAGE_ALIGN:             equ 0x1000

MULTIBOOT_MEMORY_INFO	equ	1<<1
MULTIBOOT_GRAPHICS_INFO equ 1<<2

MULTIBOOT_HEADER_FLAGS	equ	MULTIBOOT_MEMORY_INFO | MULTIBOOT_GRAPHICS_INFO
MULTIBOOT_HEADER_CHKSUM	equ	-(MULTIBOOT_HEADER_MAGIC + MULTIBOOT_HEADER_FLAGS)

%ifdef TEXT_MODE
   MULTIBOOT_GRAPHICS_MODE    equ 1
   MULTIBOOT_GRAPHICS_WIDTH   equ 80
   MULTIBOOT_GRAPHICS_HEIGHT  equ 25
   MULTIBOOT_GRAPHICS_BPP     equ 0

; originale Auflösung
;%else	
;   MULTIBOOT_GRAPHICS_MODE   equ 0
;   MULTIBOOT_GRAPHICS_WIDTH  equ 800
;   MULTIBOOT_GRAPHICS_HEIGHT equ 600
;   MULTIBOOT_GRAPHICS_BPP    equ 32
;%endif

; custom Auflösung
%else						
   MULTIBOOT_GRAPHICS_MODE   equ 0
   MULTIBOOT_GRAPHICS_WIDTH  equ 1280
   MULTIBOOT_GRAPHICS_HEIGHT equ 720
   MULTIBOOT_GRAPHICS_BPP    equ 32
%endif

[SECTION .text]

;
;   System-Start, Teil 1 (im 32-bit Protected Mode)
;
;   Initialisierung von GDT und Seitentabelle und Wechsel in den 64-bit
;   Long Mode.
;

[BITS 32]

_multiboot_header:
	align  4

;
;   Multiboot-Header zum Starten mit GRUB oder QEMU (ohne BIOS)
;
	dd MULTIBOOT_HEADER_MAGIC
	dd MULTIBOOT_HEADER_FLAGS
	dd -(MULTIBOOT_HEADER_MAGIC + MULTIBOOT_HEADER_FLAGS)
	dd _multiboot_header   
	dd (___KERNEL_DATA_START__   - KERNEL_START)
	dd (___KERNEL_DATA_END__     - KERNEL_START)
	dd (___BSS_END__             - KERNEL_START)
	dd (kmain                    - KERNEL_START)
	dd MULTIBOOT_GRAPHICS_MODE
	dd MULTIBOOT_GRAPHICS_WIDTH
	dd MULTIBOOT_GRAPHICS_HEIGHT
	dd MULTIBOOT_GRAPHICS_BPP

;  GRUB Einsprungspunkt
_start:
	  cld               ; GCC-kompilierter Code erwartet das so
	  cli               ; Interrupts ausschalten
	  lgdt   [_gdt_80]  ; Neue Segmentdeskriptoren setzen

   ; Globales Datensegment
	  ;mov    eax, 3 * 0x8
			mov    eax, 3     ; 3. Eintrag in der GDT
			shl    eax, 3     ; Index beginnt ab 2. Bit
	  mov    ds, ax
	  mov    es, ax
	  mov    fs, ax
	  mov    gs, ax

   ; Stack festlegen
	  mov    ss, ax
	  mov    esp, _init_stack+STACKSIZE
   
   ; EBX = Adresse der Multiboot-Struktur
	  mov    [_multiboot_addr], ebx

   jmp    _init_longmode


;
;  Umschalten in den 64 Bit Long-Mode
;
_init_longmode:

	  ; Adresserweiterung (PAE) aktivieren
	  mov    eax, cr4
	  or     eax, 1 << 5
	  mov    cr4, eax

	  ; Seitentabelle anlegen (Ohne geht es nicht)
	  call   _setup_paging

	  ; Long-Mode (fürs erste noch im Compatibility-Mode) aktivieren
	  mov    ecx, 0x0C0000080 ; EFER (Extended Feature Enable Register) auswaehlen
	  rdmsr
	  or     eax, 1 << 8 ; LME (Long Mode Enable)
	  wrmsr

	  ; Paging aktivieren
	  mov    eax, cr0
	  or     eax, 1 << 31
	  mov    cr0, eax

	  ; Sprung ins 64 Bit-Codesegment -> Long-Mode wird vollständig aktiviert
	  jmp    2 * 0x8 : _longmode_start    ; CS = 2. Eintrag in der GDT


;
;   Anlegen einer (provisorischen) Seitentabelle mit 2 MB Seitengröße, die die
;   ersten MAX_MEM GB direkt auf den physikalischen Speicher abbildet.
;   Dies ist notwendig, da eine funktionierende Seitentabelle für den Long-Mode
;   vorausgesetzt wird. Mehr Speicher darf das System im Moment nicht haben.
;
_setup_paging:
   ; PML4 (Page Map Level 4 / 1. Stufe)
	  mov    eax, _pdp
	  or     eax, 0xf
	  mov    dword [_pml4 + 0], eax
	  mov    dword [_pml4 + 4], 0

	  ; PDPE (Page-Directory-Pointer Entry / 2. Stufe) für aktuell 16GB
	  mov    eax, _pd
	  or     eax, 0x7           ; Adresse der ersten Tabelle (3. Stufe) mit Flags.
	  mov    ecx, 0
_fill_tables2:
	  cmp    ecx, MAX_MEM       ; MAX_MEM Tabellen referenzieren
	  je     _fill_tables2_done
	  mov    dword [_pdp + 8*ecx + 0], eax
	  mov    dword [_pdp + 8*ecx + 4], 0
	  add    eax, 0x1000        ; Die Tabellen sind je 4kB groß
	  inc    ecx
	  ja     _fill_tables2
_fill_tables2_done:

	  ; PDE (Page Directory Entry / 3. Stufe)
	  mov    eax, 0x0 | 0x87    ; Startadressenbyte 0..3 (=0) + Flags
	  mov    ebx, 0             ; Startadressenbyte 4..7 (=0)
	  mov    ecx, 0
_fill_tables3:
	  cmp    ecx, 512*MAX_MEM   ; MAX_MEM Tabellen mit je 512 Einträgen füllen
	  je     _fill_tables3_done
	  mov    dword [_pd + 8*ecx + 0], eax ; low bytes
	  mov    dword [_pd + 8*ecx + 4], ebx ; high bytes
	  add    eax, 0x200000      ; 2 MB je Seite
	  adc    ebx, 0             ; Overflow? -> Hohen Adressteil inkrementieren
	  inc    ecx
	  ja     _fill_tables3
_fill_tables3_done:

   ; Basiszeiger auf PML4 setzen
	  mov    eax, _pml4
	  mov    cr3, eax
	  ret


;
;   System-Start, Teil 2 (im 64-bit Long-Mode)
;
;   Das BSS-Segment wird gelöscht und die IDT die PICs initialisiert.
;   Anschließend werden die Konstruktoren der globalen C++-Objekte und
;   schließlich main() ausgeführt.
;
[BITS 64]
_longmode_start:
    
	; BSS löschen
	mov    rdi, ___BSS_START__
_clear_bss:
	mov    byte [rdi], 0
	inc    rdi
	cmp    rdi, ___BSS_END__
	jne    _clear_bss

	; TSS-Basisadresse im GDT-Eintrag setzen
	call _tss_set_base_address



	; Kernel-Stack im TSS setzen -> rsp0 
	mov rdi, _init_stack.end 
	call _tss_set_rsp0

_break_after_tss:

	; Lade TSS-Register mit dem TSS-Deskriptor
	; ; === Eigener Code === ; ;
	xor rax, rax
	mov rax, 0x6 	; Index 6 in das Register schreiben
	shl rax, 3 		; Schiebe 3 Bits nach Links weil die untersten nicht verwendet werden
	;mov ax, 6*8
	ltr ax			; = = = = BEI DIESEM AUFRUF GEHT WAS SCHIEF
	; ; ===  === ; ;



	; 'kmain' mit Parametern aufrufen    
	  xor    rax,rax
	  mov    dword eax, _multiboot_addr
	  mov    rdi, [rax]                 ; 1. Parameter wird in rdi uebergeben
	  call   kmain ; kernel starten
	
  	cli            ; Hier sollten wir nicht hinkommen
	  hlt


;
; TSS Basisadresse in GDT-Eintrag setzen
;
_tss_set_base_address:
	; ; === Eigener Code === ; ;
;_break_tss_begin:
	; Adresse der TSS in ein Register schreiben
	mov rbx, _tss

	; Adresse des TSS Descriptors in ein anderes Register schreiben
	mov rax, _tssd_start_adress
;_break_tss_rbx:
	; Adresse Byteweise an die richtigen Stellen des Descriptors schreiben

	; erster 2 Byte Abschnit
	mov [rax+2], bx
	; shiften
	shr rbx, 16

	; nächster 1 Byte Abschnitt, Teil 1 in Chaos Zeile
	mov [rax+4], bl
	shr rbx, 8

	; nächster 1 Byte Abschnitt, Teil 2 in Chaos Zeile
	mov [rax+7], bl
	shr rbx, 8

	; letzter 4 Byte Abschnitt
	mov [rax+8], ebx
;_break_tss_end:
	ret
	; ; ===  === ; ;



;
; Kernel-Stack im TSS = rsp0 setzen
; 1. Parameter -> rdi = Zeiger auf den Stack (letzter genutzer Eintrag)
_tss_set_rsp0:
   mov rax, _tss
   mov [rax+4], rdi
   ret


; Adresse des TSS abfragen
_get_tss_address:
   mov rax, _tss
   ret




[SECTION .data]

;
; Segment-Deskriptoren
;
_gdt:
	dw 0,0,0,0 ; NULL-Deskriptor

	; Kernel 32-Bit-Codesegment-Deskriptor (nur fuer das Booten benoetigt)
	dw 0xFFFF ; [00:15] limit = 4Gb - (0x100000*0x1000 = 4Gb)
	dw 0x0000 ; [16:31] base = 0
	dw 0x9A00 ; [32:47] base = 0, code read/exec, DPL=0, present
	dw 0x00CF ; [48:63] base, granularity=4096, IA32,

	; Kernel 64-Bit-Codesegment-Deskriptor
	dw 0xFFFF ; [00:15] limit
	dw 0x0000 ; [16:31] base
	dw 0x9A00 ; [32:47] base (8 bits), type = code, DPL=0, present
	dw 0x00AF ; [48:63] limit (4 bits), long mode, granularity=4096

	; Kernel 64-Bit-Datensegment-Deskriptor
	dw 0xFFFF ; [00:15] limit
	dw 0x0000 ; [16:31] base
	dw 0x9200 ; [32:47] base address (8 bits), type = data, DPL=0, present
	dw 0x00CF ; [48:63] limit (4 bits), long mode, granularity=4096


	; ; === Eigener Code === ; ;

	; User 64-Bit-Codesegment-Deskriptor
	dw 0xFFFF ; [00:15] limit
	dw 0x0000 ; [16:31] base
	dw 0xFA00 ; [32:47] base (8 bits), type = code, DPL=3, present
	dw 0x00AF ; [48:63] limit (4 bits), long mode, granularity=4096

	; User 64-Bit-Datensegment-Deskriptor
	dw 0xFFFF ; [00:15] limit
	dw 0x0000 ; [16:31] base
	dw 0xF200 ; [32:47] base address (8 bits), type = data, DPL=3, present
	dw 0x00CF ; [48:63] limit (4 bits), long mode, granularity=4096


	; TSSD 2*64 Bit Segmente

_tssd_start_adress:
	; Limit wird auf Max gesetzt also alles mit F
	dw 0x0068 ; [00:15] limit
	dw 0x0000 ; [16:31] Bit 0 bis 15 der Base-Adresse

	dw 0x8900 ; [00:15] Seg Pressent = 1 | Priv. Level = 1 | 0  ||  Type = Avaiable? = 0x9  || Base 20+4  || Base 16+4
	dw 0x0000 ; [16:31] Base 24+4  || Base 28+4  ||  Granularity = 0 | 0 | 0 | Avaiable for Software = 0  ||  Limit 16+4

	dw 0x0000 ; [00:15] Bit 32 bis 47 der Base-Adresse
	dw 0x0000 ; [16:31] Bit 48 bis 63 der Base-Adresse

	dw 0x0000 ; [00:15] Reserved und genullt
	dw 0x0000 ; [16:31] Reserved und genullt

	; ; ===  === ; ;

_gdt_80:
	; 4 Eintraege in der GDT
	; ; === Alter Code === ; ;
	;dw  4*8 - 1   ; GDT Limit=32, 7 GDT Eintraege - 1
	;dq  _gdt      ; Adresse der GDT

	; ; === Eigener Code === ; ;
	dw  6*8 - 1 + 16   ; GDT Limit=32, 15 GDT Eintraege - 1
	dq  _gdt      ; Adresse der GDT

;
; Addresse fuer die Multiboot-Infos wird hier gesichert
;
_multiboot_addr:
	  dq 0

;
; Speicher (104 Bytes) fuer ein Task State Segment (TSS) ohne IO-Bitmap
; siehe auch: https://stackoverflow.com/questions/54876039/creating-a-proper-task-state-segment-tss-structure-with-and-without-an-io-bitm
;
_tss:
   times 100 db 0
   dw 0
   dw 0x68


[SECTION .bss]

;
; Stack space 
;
global _init_stack:data (_init_stack.end - _init_stack)
_init_stack:
	  resb STACKSIZE
.end:


;
; Speicher fuer Page-Tables
;
[SECTION .global_pagetable]

[GLOBAL _pml4]
[GLOBAL _pdp]
[GLOBAL _pd]

_pml4:
   times 4096 db 0
	  alignb 4096

_pd:
   times MAX_MEM*4096 db 0
	  alignb 4096

_pdp:
   times MAX_MEM*8 db 0    ; 254*8 = 2032
