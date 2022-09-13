use crate::List::{Cons, Nil};

pub enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    pub fn new() -> List {
        Nil
    }

    fn prepend(self, element: u32) -> List {
        Cons(element, Box::new(self))
    }

    fn len(&self) -> u32 {
        match self {
            Cons(_, tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    fn stringify(self) -> String {
        match self {
            Cons(head, tail) => format!("{head} {}", tail.stringify()),
            Nil => format!("Nil"),
        }
    }
}

pub fn main() {
    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length {}", list.len());
    println!("{}", list.stringify());
}
