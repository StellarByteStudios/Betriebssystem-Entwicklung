# Notizen für das Betriebssystem

## Allgemeine Fragen
- [ ] Datetime: Warum format und nicht Display?
- [ ] Weiterer Syscall: Clear Screen?

## Ideen für die Shell
- [x] Erstmal Apps starten können
  - [x] Argumente übergeben können
- [ ] auto complete
- [ ] serielle Übertragung
- [x] kill-switch
- [ ] Shell-Syntax
  - [x] Environment Variablen
    - [x] Variablen nachschlagen und ersetzen
    - [x] neue Variablen anlegen
  - [ ] Pipes (Interprozesskommunikation)

## Refactoring:

## Userlib noch implementieren


## Alte Probleme
- [ ] Wenn ich eine VMA vom Typ Heap in die Liste Pushe gibt es einen Alignment error
  - Die Adresse von allen VMAs endet mit 0 oder 8, egal ob Heap oder nicht
