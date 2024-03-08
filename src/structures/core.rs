#[derive(Debug)]
struct Node {
    data: u8,
    next: Option<Box<Node>>,
}

struct LinkedList {
    head: Option<Box<Node>>,
}

impl Node {
    pub fn new(data: u8) -> Self {
        Node { data, next: None }
    }
}

impl LinkedList {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn push(&mut self, data: u8) {
        let node: Node = Node::new(data);
        if self.head.is_none() {
            self.head = Some(Box::new(node));
        } else {}
    }
}

pub fn main() {}