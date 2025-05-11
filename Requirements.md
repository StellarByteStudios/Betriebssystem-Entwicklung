# Requirements für das Betriebssysteme-Seminar
Voraussichtliche Kooperation von Carsten Krollmann und Moritz Reifer

## Grober Plan
- Jeder arbeitet in seinem eigenen System
- Gemeinsame Schnittstelle über Userlib
    - Beide Betriebssysteme laden Userlib
- rudimentäre Shell
- "Platformunabhängige" Anwendungsentwicklung für Shell-Programme


## Verfeinerungen der Ideen
### Userlib
Die userlib, welche im letzten Semester eingeführt wurde soll auf zwei unterschiedlichen Systemen implementiert werden und funktionieren
- Zwei Betriebssysteme, eine lib
- git Repository für die lib
    - wird von beiden Systemen über cargo geladen
- userlib legt die Syscalls fest (und einige weitere Funktionen)
    - Beide Betriebssysteme müssen diese Syscalls unterstützen
- userlib wird gemeinsam entwickelt

#### Erweiterte Funktionen
Es sollen noch weitere Syscalls hinzugefügt werden um die Möglichkeiten für Apps zu vergrößern. Dabei sind schon folgende Ideen aufgekommen:
- Schnittstelle für Bilderausgabe
- finden und Monitoren von Prozessen
- beenden von Prozessen
- Übergabe von Paramentern in Programme
- Abspielen von Musik
    - entweder feste Lieder oder
    - eigenes Format um extern Lieder zu laden
- (Ausgabe auf neuer serieller Schnittstelle [nicht kprint])


### Shell und ihre Programme
Ziel dieser Kopplung ist es "platformunabhängige" Apps zu schreiben welche nur eine Dependency auf die userlib haben und dementsprechend auf beiden Systemen laufen
- Bereits vorhandene Apps wieder zum Funktionieren bringen
- Neue Programme entwickeln. Ideen
    - finden und indenifizieren von Prozessen
    - beenden von Prozessen
    - verändern der Shell-Optik
    - auslesen und Anzeigen von Echter-Uhrzeit
    - Grafische Programme
        - Mandelbrot
        - Newton-Fraktale
        - (3D Grafik)
- Zusätzliche Funktionen
    - Environment Variablen
    - Pipes (Interprozesskomunikation)
        - Wahl des Kanals auf dem Ausgaben passieren
    - Kommandozeilen Argumente
    - Auto-Completion


## Vortrag
Einige mögliche Teil-Themen für den Vortrag:
- Geschichte der Shell
    - was war die erste
    - wie hat sie sich entwickelt
        - erste größere Funktionen
            - environment Variablen
            - shell/bash syntax
            - remote shells
    - was gibt es heute
        - AI shell
- Wie macht man Apps "plattformunabhängig"
    - über die Schwierigkeiten, die gemeinsame Schnittstelle zu bauen
    - über APIs/ABIs 
- Besondere Schwierigkeiten bei dem Projekt


## Lernziele
- Kooperatives Development
- Erstellen und erfüllen einer eigenen API
- Grundfunktionalitäten einer Shell
- Modularisierung des Betriebssystems