mod garden;
mod models;

fn main() {
    let number: u8 = rand::random();
    println!("Random number: {}", number);
    println!("{:?}", garden::vegetables::Asparagus {});
    let user: models::User = models::User::new(1, "John".to_string(),  "ae@email.com".to_string());
    println!("{:?}", user);
}
