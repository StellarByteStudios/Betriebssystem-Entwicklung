use alloc::rc::Rc;
use core::{cell::RefCell, fmt::Display};

// Definition eines generischen Listenelements
pub struct Node<T> {
    pub data: T,
    pub next: Option<Rc<RefCell<Node<T>>>>,
}

// Implementierung eines Konstruktors für ein generisches Listenelement
impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Self { data, next: None }
    }
}

// Definition der generischen Liste
#[derive(Clone)]
pub struct Queue<T> {
    head: Link<T>,
}

// Typ-Definition für eine Referenz auf ein Listenelement
pub type Link<T> = Option<Rc<RefCell<Node<T>>>>;

impl<T: PartialEq> Queue<T> {
    // Konstruktor, um eine leere Liste zu erzeugen
    pub const fn new() -> Self {
        Self { head: None }
    }

    // Ein Listenelement am Ende der Liste einfuegen
    pub fn enqueue(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(Node::new(data)));

        if self.head.is_none() {
            self.head = Some(new_node.clone());
        } else {
            let mut node = self.head.clone();
            while let Some(n) = node {
                if n.borrow_mut().next.is_none() {
                    n.borrow_mut().next = Some(new_node);
                    break;
                }
                node = n.borrow().next.clone();
            }
        }
    }

    // Das Listenelement am Kopf der Liste aushaengen und zurueckgeben
    pub fn dequeue(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    self.head = Some(new_head);
                }
                None => {}
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().data
        })
    }

    // Suche und entferne das Element 'data'
    // Rueckgabewert: true:  falls das Element gefunden und geloescht wurde
    //                false: sonst
    pub fn remove(&mut self, data: T) -> bool {
        // Ist überhaupt was in der Liste?
        if self.head.is_none() {
            // Liste war leer
            return false;
        }

        // Ist der Head schon die Node, die entfernt werden soll
        // Kopf Klonen
        let head_cloned = self.head.clone();
        // Option auspackens
        let head_node = head_cloned.unwrap();

        if head_node.borrow().data == data {
            // Kopf wurde gefunden. Also muss er überschrieben werden
            // Kopf-Nachfolger zum neuen Kopf machen
            let new_head_ref = self.head.clone().unwrap();
            let new_head = new_head_ref.borrow_mut().next.take();
            self.head = new_head;

            // Kopf wurde erfolgreich gelöscht
            return true;
        }

        // Zeiger auf die Node an der wir grade sind
        let mut node = self.head.clone();

        while node.is_some() {
            // Vorgänder Speichern (wichtig für Aushängen)
            let prev = node.unwrap();

            // Eins weiter gehen in der Liste
            node = prev.borrow().next.clone();

            // Ist die nächste Node überhaup eine Valide
            if node.is_none() {
                // Wenn nicht ist die Queue am Ende
                return false;
            }

            // Sind dort unsere gesuchten Daten
            if node.clone().unwrap().borrow().data == data {
                // Zeiger von Prev auf den Nachfolger von Node gesetzt und damit Node gelöscht
                prev.borrow_mut().next = node.unwrap().borrow_mut().next.take();

                return true;
            }
        }

        return false;
    }

    pub fn is_empty(&self) -> bool {
        // Ist überhaupt was in der Liste?
        if self.head.is_some() {
            // Liste war leer
            return false;
        }

        return true;
    }
}

// Ausgabe der Liste
impl<T: Display> Display for Queue<T> {
    fn fmt(&self, w: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        write!(w, "[")?;
        let mut node = self.head.clone();
        while let Some(n) = node {
            write!(w, "{}", n.borrow().data)?;
            node = n.borrow().next.clone();
            if node.is_some() {
                write!(w, ", ")?;
            }
        }
        write!(w, "]")
    }
}
