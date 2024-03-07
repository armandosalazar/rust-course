#![allow(dead_code)]

#[derive(Debug)]
pub struct User {
    id: i32,
    name: String,
    email: String,
}

impl User {
    pub fn new(id: i32, name: String, email: String) -> User {
        User { id, name, email }
    }
}