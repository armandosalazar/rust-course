// Trait Debug
use std::fmt::{Display, Formatter, write};

// Attribute Derive
#[derive(Debug)]
struct User {
    name: String,
    age: u8,
}

// Trait Display -> Imprimir sin necesidad de usar el Debug de una forma más bonita
impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "User {{ {}, {} }}", self.name, self.age)
    }
}

// impl std::fmt::Debug for User {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "User {{ {}, {} }}", self.name, self.age)
//     }
// }

pub fn println_exp() {
    let user = User {
        name: String::from("Armando Salazar"),
        age: 0,
    };
    println!("hola {:?}", "Armando");
    println!("{:?}", user);
    println!("{}", user);
}