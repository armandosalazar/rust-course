mod garden;
mod models;
mod structures;

fn main() {
    // let number: u8 = rand::random();
    // println!("Random number: {}", number);
    // println!("{:?}", garden::vegetables::Asparagus {});
    // let user: models::User = models::User::new(1, "John".to_string(),  "ae@email.com".to_string());
    // println!("{:?}", user);

    let mut list: structures::LinkedList = structures::LinkedList::new();
    list.push(1);
    println!("{:?}", list);
    list.push(2);

}
