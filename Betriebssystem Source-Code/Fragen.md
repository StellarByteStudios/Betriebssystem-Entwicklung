# Notizen für das Betriebssystem

## Allgemeine Fragen
- [ ] Gibt es eine Möglichkeit den Programmen direkt irgendwie (So wie vorher bei den Threads) Argumente zu übergeben?
- [ ] Wo gebe ich an, welche Funktion die Startfunktion der App/ des Kernels ist
- [ ] --profile production geht bei mir nicht. Findet main nicht richtig

## Alte Probleme
- [ ] Wenn ich eine VMA vom Typ Heap in die Liste Pushe gibt es einen Alignment error
  - Die Adresse von allen VMAs endet mit 0 oder 8, egal ob Heap oder nicht
 