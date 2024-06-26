#![allow(dead_code)]
mod models;

fn main() {
    println!("{}", 1);
    let user = User {
        id: 1,
        name: String::from("Armando")
    };


    println!("{:?}", user);
    print_user(&user);
    println!("{:?}", user);

    // get memory address of user
    let user_address = &user as *const User;
    println!("{:?}", user_address);
    let address: *const User = &user;
    println!("{:?}", address);
    unsafe {
        println!("{:?}", *address);
    }
}

#[derive(Debug)]
struct User {
    id: u8,
    name: String
}

fn print_user(user: &User) {
    println!("{}", user.id)
}


// create crate
// cargo new --lib my_crate
// create module
// mod my_module {
//    pub fn my_function() {
//    println!("Hello from my_function");
//    }
//    pub mod my_sub_module {
//    pub fn my_sub_function() {
//    println!("Hello from my_sub_function");
//    }
//    }
//    }
//
//    use my_crate::my_module;
mod my_module {
    pub fn my_function() {
        println!("Hello from my_function");
    }

    pub mod my_sub_module {
        pub fn my_sub_function() {
            println!("Hello from my_sub_function");
        }
    }
}
