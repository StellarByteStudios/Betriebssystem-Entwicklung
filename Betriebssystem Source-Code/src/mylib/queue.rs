
use core::cell::RefCell;
use alloc::rc::Rc;
use core::fmt::Display;
use core::fmt;


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
      }
      else {
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
                None => {
                }
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
        /*
        // Ist der Head schon die Node, die entfernt werden soll
        //let head_data: T = self.head.borrow().unwrap().as_ref().borrow().data;
        let head_data: T = self.head.as_ref().map(|rc| rc.borrow()).unwrap().data;
        if head_data == data{
            // Kopf wurde gefunden. Also muss er überschrieben werden
            // Kopf speichern um in später wieder frei zu geben
            let old_head = self.head.unwrap().as_ptr();

            // Kopf-Nachfolger zum neuen Kopf machen
            self.head = self.head.as_ref().map(|rc| rc.borrow()).unwrap().next;

            // Alten Kopf löschen
            drop(old_head);

            // Kopf wurde erfolgreich gelöscht
            return true;
        }

        // Node als Iterator-Pointer bestimmen
        let mut node: Link<T> = self.head.clone();

        // Vorgänder Speichern (wichtig für Aushängen)
        let mut prev: Link<T> = None;

        // Durch alle nodes durchgehen
        while node.is_some() {
            // Vorgänger speichern
            prev = node;

            // Node eins weiter gehtn
            node = node.as_ref().map(|rc| rc.borrow()).unwrap().next;

            // Daten holen
            let note_data = node.as_ref().map(|rc| rc.borrow()).unwrap().data;

            // Checken ob diese Daten übereinstimmen
            if note_data == data {
	           prev.as_ref().map(|rc| rc.borrow()).unwrap().next = None;
	           break;
            }
        } */
      

        return false;
   }




    /* Funktioniert nicht so ganz
    // Eingene Hilfsmethode um an die Nodes zu kommen
    fn get_node(node: Link<T>){
        return node.as_ref()
        .map(|rc| rc.borrow())
        .unwrap()
    } */


}


// Ausgabe der Liste
impl<T: Display> Display for Queue<T> {
    fn fmt(&self, w: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
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