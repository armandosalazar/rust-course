// @todo - Permitir variables sin usar en todo el archivo
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
#[derive(Debug)]
struct Node {
    value: u8,
    next: Option<Box<Node>>
}
#[derive(Debug)]
pub struct LinkedList {
    head: Option<Box<Node>>
}

impl LinkedList {
    pub fn new() -> LinkedList {
        LinkedList { head: None }
    }

    pub fn push(&mut self, value: u8) {
        let node: Node = Node {
            value: value,
            next: None
        };
        if self.head.is_none() {
            self.head = Some(Box::new(node));
        } else {
            let current = &mut self.head;
            print!("{:?}", *current);
        }
    }
}
