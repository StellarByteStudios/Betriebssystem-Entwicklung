# Notizen für das Betriebssystem
2) Beim Threadwechsel: Wie bekomme ich den Pointer auf die neue Page-Table in Assembler?
3) Mit dem Debugger komm ich in die Idle-Thread Methode
    - Über die Scheduler initialized Methode
    - In die vprint() Methode. (Dort drin hat es mir zu lagen gedauert)
    - Sobald man einmal continue oder step out macht, kommt man in den Intdispatcher
