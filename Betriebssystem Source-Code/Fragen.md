# Notizen für das Betriebssystem

- [ ] Bei Stackvergrößern wird nicht der richtige Prozess gefunden (Das Stackvergrößern scheint zu funktionieren, aber dann kommt plötzlich ein Pagefault nahe 0)
  - Scheint nicht zu funktionieren. Wenn man im Fault den Prozess holt, hat der eine leere Liste und String
  - Prozess (Speicher) wird überschrieben wenn ich versuche den Prozess zu holen
  - Der Fehler mit Prozess holen tritt nur auf, wenn im Page Fault
- [ ] Wenn ich eine VMA vom Typ Heap in die Liste Pushe gibt es einen Alignment error
  - Die Adresse von allen VMAs endet mit 0 oder 8, egal ob Heap oder nicht
- [ ] Versuche ich im Alloc-Init eine Node in den gegebenen Speicher zu schreiben bekommen ich einen General Protection Fault
  - Der Fault titt auf, direkt im Assembler-Befehl, der die Daten in die Speicherstelle schreiben soll
  - Möglich, das ptr.write ein Priviligierter Befehl ist? Listallokator ist ja im Usermode