use rand::random;
use crate::UserRole::{ADMIN, USER};
use crate::WebSite::URL;

// Structs and Enums
struct User {
    name: String,
    email: String,
    age: Option<u8>,
    active: Option<bool>,
    role: UserRole,
    website: WebSite,
}

enum UserRole {
    USER,
    ADMIN,
}

enum WebSite {
    URL(String),
}

impl User {
    // Static method
    fn new(name: &str, email: &str, age: Option<u8>, active: Option<bool>, role: UserRole, website: WebSite) -> User {
        User {
            name: String::from(name),
            email: email.to_string(),
            age,
            active,
            role,
            website,
        }
    }

    fn get_age(&self) -> Option<u8> {
        self.age
    }

    fn get_email(&self) -> String {
        self.email.clone()
    }

    fn is_active(&self) -> bool {
        // Importante no usar unwrap() porque si aquí estuviera así tendría un error en caso de que el Option sea None
        // porque estaría intentando desenvolver un valor que es None, por eso es mejor usar cualquier otro método.
        self.active.unwrap_or_default()
    }
}

fn main() {
    let number: u8 = random();
    println!("Random number: {}", number);
    // Scope and Shadowing
    {
        let x: u8 = 17;
        println!("Number: {}", x)
    }

    // println!("x: {}", x);
    let number = "one";
    println!("Length of string: {}", number.len());

    // Const
    const PI: f64 = std::f64::consts::PI;
    println!("PI: {}", PI);
    // Integers literals
    let _sig: i8 = -12;
    let _uns: u8 = 12;
    let _dec = 98_22;
    let _hex = 0xff;
    let _oct = 0o77;
    let _bin = 0b1111_0000;
    let _bol: bool = false;
    let _cha: char = 'F';
    // Tuple
    let tup: (u8, u8, u8) = (1, 2, 2);
    let (_x, _y, _z) = tup;
    println!("{}", tup.1);
    // Array
    let _arr: [u8; 5] = [1, 2, 3, 4, 5];
    // String
    let _name: &'static str = "Armando";
    let _lastname: String = "Armando".to_string();
    let mut _address = String::new();
    say_hi("Armando");
    // Expression -> Statement vs Expression
    let _value = { 8 };
    println!("Ref: {}", add_ref(&8));
    // Use struct
    let mut user = User {
        name: String::from("armando@email"),
        email: "armando@email".to_string(),
        age: Some(0),
        active: Some(true),
        role: ADMIN,
        website: URL("www.armando@salazar.com".to_string()),
    };
    println!("username: {}", user.name);
    user.active = Some(false);
    println!("email: {}", user.get_email());
    let age = user.get_age();
    match age {
        Some(age) => println!("age: {}", age),
        None => (),
    }
    println!("active: {}", user.is_active());
    let access = has_access(&user.role);
    println!("Have access: {}", access);
    let _new_user = User {
        name: String::from("bryan@email"),
        ..user
    };

    let _last_user = User::new("luis@user", "luis@email", Some(24), Some(true), USER, URL("www.google.com".to_string()));

    // Tuple structs
    struct Point(u8, u8, u8);
    let _point = Point(12, 23, 34);

    // Use enums
    let _role = USER;

    // Option
    let mut username: Option<&str> = None;
    username = Some("armando@user");
    match username {
        Some(username) => println!("{}", username),
        None => println!("username none"),
        // _, indica el resto de los casos
        // (), indica nada, hacer nada
        // _ => ()
    }
}

// Functions snake_case
fn say_hi(name: &str) {
    println!("Hi {}!", name)
}

fn add_ref(number: &u8) -> u8 {
    *number + 1
}

fn has_access(role: &UserRole) -> bool {
    match role {
        USER => false,
        ADMIN => true,
    }
}
