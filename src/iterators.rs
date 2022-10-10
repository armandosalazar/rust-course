// Trait Iterator

pub fn iterators() {
    let numbers = [1, 2, 3, 4];
    for number in numbers.iter() {
        print!(" {} ", number);
    }
    print!("{}", '\n');
    println!("{:?}", numbers);
    let vector: Vec<String> = Vec::new();
    let _ = vector.iter();
    let mut c = Counter::new();
    let option = c.next();
    match option {
        Some(number) => println!("{}", number),
        None => (),
    }
}

struct Counter {
    count: u8,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

// Trait
impl Iterator for Counter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        Some(self.count)
    }
}