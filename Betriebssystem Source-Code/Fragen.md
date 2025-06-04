# Notizen für das Betriebssystem

## Allgemeine Fragen
### Probleme mit zu viel Ram
- Überprüft: Tar ist korrekt
- Mapping ist korrekt
- Tar-Adresse ist kleiner als die maximale Physische Adresse
- Grub und Multiboot scheinen die Tar richtig zu laden
- Wenn ich den Speicherauslese, hat es keinen richtigen Header
  - Wenn wenig RAM: Anfang besteht aus "./" was ein vernüftiger Tar-Anfang ist 

## Ideen für die Shell
- [x] Erstmal Apps starten können
  - [x] Argumente übergeben können
- [ ] auto complete
- [ ] serielle Übertragung
- [ ] kill-switch
- [ ] Shell-Syntax
  - [ ] Environment Variablen
    - [x] Variablen nachschlagen und ersetzen
    - [ ] neue Variablen anlegen
  - [ ] Pipes (Interprozesskommunikation)

## Refactoring:

## Userlib noch implementieren
- [ ] neue Syscalls


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
- Pagefault tritt im Printer Macro auf. Wenn das weg ist, gehts
- Jetzt bekomme ich `Panic: invalid opcode - processor halted.` 
  - Ich habe versucht die App zu finden bei der das passiert, aber immer wenn ich nur 3 beliebige habe, läuft das, aber mit allen 4 geht alles kaputt
  - In der Regel von Music erzeugt
- Alternative: Starte nicht alle Apps bei boot sondern über shell:
  - quasi immer page faults, sobalt "hello" oder "extra" mit drin sind. Aber nicht alleine
  - Meistens ein Pagefault direkt am Code (0x...1000) oder bei `0xfd280003`
  - Einige Appkombinationen laufen ohne Probleme und manche nicht. Fehlerquelle meist die Animation
- Ich habe herausgefunden, selbst wenn ich in allen `Cargo.toml` die Optimierung auf 0 Stelle bleibt das Problem bestehen



## Alte Probleme
- [ ] Wenn ich eine VMA vom Typ Heap in die Liste Pushe gibt es einen Alignment error
  - Die Adresse von allen VMAs endet mit 0 oder 8, egal ob Heap oder nicht
 
## Vortrags-Notizen
- Reden über Shell Geschichte
  - KI Shell 


## Promt an KI
Ich schreibe ein Betriebssystem in Rust. Dort habe ich ein sehr kompliziertes Problem.
*Zur Struktur des Systems:*
Ich habe ein Hauptsystem welches Speicherverwaltung, schedulling, paging, etc. implementiert und verwaltet. Ich habe eine userlib, welche eine Schnittstelle für User Applications bildet. Diese sollen nämlich separat vom Kernel kompiliert werden, aber im Usermode laufen. Daher stellt die Userlib die Schnittstelle über Syscalls bereit. Ich habe des weiteren 4 verschiedene Apps mit einer kleinen Main Methode, welche einige dieser Syscalls verwenden. Jedes der 6 genannten Systeme ist in einem eigenen Rust workspace und wird von einem darüberliegenden Makefile mit Cargo make zusammen gebaut und gelinkt.
Wenn das System zusammen gelinkt wird, werden erst alle einzelnen Subsysteme gebaut. Die Apps werden dann von einer .elf Datei in eine .bin Datei umgewandelt und zu einem einzigen Tar-File zusammengepackt. Beim Starten des Betriebssystems läd der Kernel diese Tar-Datei als Multiboot-Modul, und extrahiert daraus die Programme, sodass sie später gestartet werden können
Das System verwendet Multiboot und grub und läuft im Graphic VGA modus

*Problembeschreibung:*
Es gibt in Rust 2 unterschiedliche Möglichkeiten zu Kompilieren. Es gibt den development modus. Dort werden alle Debug-Symbole beibehalten. Methoden werden gleich benannt und es wird nichts optimiert. Dann gibt es den Production oder Release Modus, dort werden vom Kompiler einige optimierungen vorgenommen.
Ich habe jetzt das Problem, dass ich verschiedene unerklärliche Probleme habe, welche aber auschließlich auftreten, wenn ich im Production Modus kompiliere. Wenn ich das System im Development Modus kompiliere läuft alles reibungslos.

*Folgende Probleme Treten auf:*
Ich habe 4 Programme namens "hello", "extra", "animation" und "music" die folgendes machen:
- "hello" & "Extra": geben nur ein paar Zahlen über die serielle Schnittstelle via Syscalls aus
- "animation": gibt wiederholt eine Bit-Map auf dem Bildschirm aus
- "music": spielt durch Triggern des pc-speakers eine Melodie

Problemfälle:
- Wenn ich 3 Programme von "hello" und "Extra" in beliebiger reihenfolge und kombination starte, gibt es beim 3ten mal einen Pagefault und zwar immer an der addresse "0x10000001000". Dabei Startet der Usercode bei "0x10000000000" und die Page im Virtuellen Adressraum geht genau bis "0x10000000fff", es ist also ein off bei one error. Es wirkt so, als würde die App direkt versuchen bei "0x10000001000" zu starten, da sie vorher nix ausgibt und "PageFault: error_code = 0b100, cs-Register = 0b100011, rip-Register = 0x10000001000, CR2 = 0x10000001000" gilt
- wenn ich zuerst das Programm "animation" starte und dann ein weiteres Programm, dann bekomme ich auch einen Pagefault "PageFault: error_code = 0b101, cs-Register = 0b100011, rip-Register = 0x10000000004, CR2 = 0xfd300003" auf eine Scheinbar zufällige Addresse
- wenn ich zuerst das programm "music" starte und dann ein weiteres programm bekomme ich dauerhaft "Panic: invalid opcode - processor halted."
- Jede App alleine läuft ohne Probleme


Hast du eine Idee, wo ich nach dem Fehler suchen könnte?

