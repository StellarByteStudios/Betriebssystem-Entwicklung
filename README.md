# Betriebssystem-Entwicklung
Dies ist ein Betriebssystem, welches im Rahmen von 3 Modulen in 3 Semestern an der HHU entwickelt wurde.

---

## Funktionalität
Das Betriebsystem bietet folgende grundlegenden Funktionen:
* Eingabe durch PS/2 Tastatur
* Grafische Ausgabe auf Bildschirm durch VGA Treiber
* Ausgabe auf Serieller Schnittstelle
* Präemptives Multitasking
* Trennung von User/Kernel Mode
* Paging/Speicherverwaltung
* Offene Schnittstelle zu Syscalls

Im aktuellen Zustand wird die Shell als Prozess gestartet und von da aus kann man dann die anderen Apps starten

## Abhängigkeiten
Sowohl die Apps als auch der Kernel sind abhängig von der [User-Library](https://github.com/StellarByteStudios/hhuTOSuserlib) welche auch im Rahmen der Uni-Module entwickelt wurde. Ansonsten gibt es wenig Abhängigkeiten (Library zum entpacken von Tar-Dateien und random Zahlen generator) und der gesamte Code ist in `#[no_std]` geschrieben.


## Ausführung
Gebaut wir das Betriebssystem mit Rust Nightly (cargo 1.86.0-nightly) und getestet mit qemu-system-x86_64 (QEMU emulator version 7.2.19 (Debian 1:7.2+dfsg-7+deb12u16)). Man geht in den Ordner des Betriebssystems (Betriebssystem Source-Code). Dort kann das System mit `cargo make --no-workspace` bauen oder mit `cargo make --no-workspace --profile production qemu` testen.