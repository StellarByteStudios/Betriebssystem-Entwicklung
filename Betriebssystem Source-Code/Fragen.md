# Notizen für das Betriebssystem

## Allgemeine Fragen
- [ ] Gibt es eine Möglichkeit den Programmen (Apps) direkt irgendwie (So wie vorher bei den Threads) Argumente zu übergeben?
- [ ] Wo gebe ich an, welche Funktion die Startfunktion der App/ des Kernels ist
- [ ] --profile production geht bei mir nicht. ~~Findet main nicht richtig~~ Optimierungen führen zu Pointerverlust

## Probleme mit Production
```rust
pub extern "C" fn kmain(mbi: u64) {
    kprintln!("kmain");

    kprintln!("----------- Noch vor dem Init. MBI-Pointer: {:#x}\n", mbi); // Print 1

    // Alles Wichtige Initialisieren
    init_all(mbi);

    // Kernel-Prozess mit Idle-Thread erzeugen und im Scheduler registrieren
    scheduler::spawn_kernel();

    kprintln!("----------- Nach Spawn kernel. MBI-Pointer: {:#x}\n", mbi); // Print 2

    // Apps aus initrd.tar extrahieren
    let opt_apps: Option<Vec<AppRegion>> = appregion::get_apps_from_tar(mbi); // Use of mbi 2

    // Wurde was geladen?
    if opt_apps.is_none() {
        kprintln!("!=!=!=!=!=!=!=!=!=!=!=!=!=!=! No apps found !=!=!=!=!=!=!=!=!=!=!=!=!=!=!");
        // Dauerloop
        loop {}
    }
  // ...

// ========================= Teil in Appregion ========================= //
pub fn get_apps_from_tar(mbi_ptr: u64) -> Option<Vec<AppRegion>> {
    // Erstmal Multiboot auslesen
    let multiboot_info: &MultibootInfo = unsafe { MultibootInfo::read(mbi_ptr) };
    kprintln!("!=!=!=!==!=!=!=!=! Ich bin hier Nummer 1");
  
    //dump(mbi_ptr);
    kprintln!("!=!=!=!=!=!=!=!=! ------ Multiboot Pointer: {:#x}", mbi_ptr);
    //...
```
Ich habe folgendes herausgefunden: 
- Wenn ich beide Prints weg lasse, hat der Pointer in der Methode `get_apps_from_tar` den wert `0`
- Wenn ich das erste Print weg lasse und nur das zweite Print da habe, dannläuft das System bis zu dem Print, aber wenn der Pointer im Print 2 Ausgegeben werden soll, bleibt es da einfach stehen
- Wenn beide Prints drin sind, werden alle Apps ohne Probleme geladen

**Jetzt bekomme ich einen Pagefault irgendwo in meiner App auf Adresse `0x0`**


## Ideen für die Shell
- [ ] Erstmal Apps starten können
- [ ] auto complete
- [ ] serielle Übertragung
- [ ] kill-switch
- [ ] Shell-Syntax
  - [ ] Environment Variablen
  - [ ] Pipes (Interprozesskommunikation)

## Refactoring:
- [ ] Syscalls aufräumen
  - [ ] const Nummern durch [ein Enum](https://github.com/hhu-bsinfo/D3OS/blob/main/os/library/syscall/src/lib.rs) austauschen 
  - [ ] Nicht benutze Syscalls aufräumen
    - [ ] sys_read
    - [ ] sys_write
    - [ ] hello (ohne Print)
  - [ ] evtl. nur noch eine `syscall`-Methode mit variabler Eingabemenge `pub fn syscall(call: SystemCall, args: &[usize]) -> SyscallResult {`

## Alte Probleme
- [ ] Wenn ich eine VMA vom Typ Heap in die Liste Pushe gibt es einen Alignment error
  - Die Adresse von allen VMAs endet mit 0 oder 8, egal ob Heap oder nicht
 
## Vortrags-Notizen
- Reden über Shell Geschichte
  - KI Shell 

