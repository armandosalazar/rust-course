pub mod list;
pub mod core;


#[derive(Debug)]
struct Node {
    data: u8,
    next: *mut Node,
}

impl Node {
    fn new(data: u8) -> Node {
        Node {
            data,
            next: std::ptr::null_mut(),
        }
    }
}

pub struct LinkedList {
    head: *mut Node,
}

impl LinkedList {
    pub fn new() -> LinkedList {
        LinkedList {
            head: std::ptr::null_mut(),
        }
    }

    pub fn push(&mut self, data: u8) {
        if self.head.is_null() {
            self.head = &mut Node::new(data);
        } else {
            let mut current = self.head;
            unsafe {
                println!("{:?}", *current);
                while !(*current).next.is_null() {
                    current = (*current).next;
                }
                (*current).next = &mut Node::new(data);
            }
        }
    }

    pub fn show(&self) {
        unsafe {
            let current = self.head;
            println!("{:?}", *current);
        }
    }
}


pub mod linked_list {
    pub fn run() {
        let mut node = Node {
            data: 1,
            next: std::ptr::null(),
        };
        println!("{:?}", node);
        node.next = &Node {
            data: 2,
            next: std::ptr::null(),
        };

        unsafe {
            println!("{:?}", *node.next);
        }
    }

    #[derive(Debug)]
    struct Node {
        data: i32,
        next: *const Node,
    }
}