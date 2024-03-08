pub mod nodes {
    #[derive(Debug)]
    struct Node {
        value: u8,
        next: *mut Node,
    }

    static mut HEAD: *mut Node = std::ptr::null_mut();

    pub fn push(value: u8) {
        unsafe {
            if HEAD.is_null() {
                HEAD = &mut Node {
                    value,
                    next: std::ptr::null_mut(),
                };
            } else {
                let mut current = HEAD;
                while (*current).next != std::ptr::null_mut() {
                    current = (*current).next;
                }
                (*current).next = &mut Node {
                    value,
                    next: std::ptr::null_mut(),
                };

                // @note *(*current.next).next
            }
        }
    }

    pub fn run() {
        push(1);
        push(2);
    }
}