use crate::mylib::queue::{self, Queue};

pub fn run() {
    kprintln!("Testen der Queue");
    kprintln!("Queue Initialisieren");

    let mut queue: Queue<u32> = queue::Queue::new();

    kprintln!("Leere Queue: {}", queue);

    kprintln!("Werten nun in die Queue einfügen");

    queue.enqueue(1);
    queue.enqueue(3);
    queue.enqueue(5);

    kprintln!("Daten in der Queue: {}", queue);

    kprintln!("Teste Dequeue");

    let front_value = queue.dequeue();

    kprintln!(
        "Ausgehangener Wert: {:} und hier die Queue: {}",
        front_value.unwrap(),
        queue
    );

    kprintln!("Neue Werte einfügen");

    queue.enqueue(10);
    queue.enqueue(30);
    queue.enqueue(50);

    kprintln!("Daten in der Queue: {}", queue);

    kprintln!("Teste remove");

    let mut remover = 30;
    let mut successfull_remove = queue.remove(remover);

    kprintln!(
        "Gelöschter Wert: {:} war erfolgreich {:} und hier die Queue: {}",
        remover,
        successfull_remove,
        queue
    );

    kprintln!("Für weitere Tests noch ein paar sachen in die Queue schreiben");

    queue.enqueue(10046);
    queue.enqueue(34);
    queue.enqueue(70);

    kprintln!("Daten in der Queue: {}", queue);

    kprintln!("\nVersuche Head zu löschen");
    remover = 3;
    successfull_remove = queue.remove(remover);
    kprintln!(
        "Gelöschter Wert: {:} war erfolgreich {:} und hier die Queue: {}",
        remover,
        successfull_remove,
        queue
    );

    kprintln!("\nVersuche Letztes zu löschen");
    remover = 70;
    successfull_remove = queue.remove(remover);
    kprintln!(
        "Gelöschter Wert: {:} war erfolgreich {:} und hier die Queue: {}",
        remover,
        successfull_remove,
        queue
    );

    kprintln!("\nVersuche nicht Vorhandenes zu löschen");
    remover = 99;
    successfull_remove = queue.remove(remover);
    kprintln!(
        "Gelöschter Wert: {:} war erfolgreich {:} und hier die Queue: {}",
        remover,
        successfull_remove,
        queue
    );

    kprintln!("\nVersuche aus leerer Liste zu löschen zu löschen");
    let mut empty_queue: Queue<u32> = queue::Queue::new();
    remover = 99;
    successfull_remove = empty_queue.remove(remover);
    kprintln!(
        "Gelöschter Wert: {:} war erfolgreich {:} und hier die Queue: {}",
        remover,
        successfull_remove,
        empty_queue
    );
}

pub fn run_display() {
    println!("Testen der Queue");
    println!("Queue Initialisieren");

    let mut queue: Queue<u32> = queue::Queue::new();

    println!("Leere Queue: {}", queue);

    println!("Werten nun in die Queue einfügen");

    queue.enqueue(1);
    queue.enqueue(3);
    queue.enqueue(5);

    println!("Daten in der Queue: {}", queue);

    println!("Teste Dequeue");

    let front_value = queue.dequeue();

    println!(
        "Ausgehangener Wert: {:} und hier die Queue: {}",
        front_value.unwrap(),
        queue
    );

    println!("Neue Werte einfügen");

    queue.enqueue(10);
    queue.enqueue(30);
    queue.enqueue(50);

    println!("Daten in der Queue: {}", queue);

    println!("Teste remove");

    let mut remover = 30;
    let mut successfull_remove = queue.remove(remover);

    println!(
        "Gelöschter Wert: {:} war erfolgreich {:} und hier die Queue: {}",
        remover, successfull_remove, queue
    );

    println!("Für weitere Tests noch ein paar sachen in die Queue schreiben");

    queue.enqueue(10046);
    queue.enqueue(34);
    queue.enqueue(70);

    println!("Daten in der Queue: {}", queue);

    println!("\nVersuche Head zu löschen");
    remover = 3;
    successfull_remove = queue.remove(remover);
    println!(
        "Gelöschter Wert: {:} war erfolgreich {:} und hier die Queue: {}",
        remover, successfull_remove, queue
    );

    println!("\nVersuche Letztes zu löschen");
    remover = 70;
    successfull_remove = queue.remove(remover);
    println!(
        "Gelöschter Wert: {:} war erfolgreich {:} und hier die Queue: {}",
        remover, successfull_remove, queue
    );

    println!("\nVersuche nicht Vorhandenes zu löschen");
    remover = 99;
    successfull_remove = queue.remove(remover);
    println!(
        "Gelöschter Wert: {:} war erfolgreich {:} und hier die Queue: {}",
        remover, successfull_remove, queue
    );

    println!("\nVersuche aus leerer Liste zu löschen zu löschen");
    let mut empty_queue: Queue<u32> = queue::Queue::new();
    remover = 99;
    successfull_remove = empty_queue.remove(remover);
    println!(
        "Gelöschter Wert: {:} war erfolgreich {:} und hier die Queue: {}",
        remover, successfull_remove, empty_queue
    );
}
