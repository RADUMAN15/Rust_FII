use std::io::{self, Write};
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

struct Cache {
    values: HashMap<u64, bool>,
    order: Vec<u64>,
    capacity: usize,
}

impl Cache {
    fn new(capacity: usize) -> Self {
        Cache {
            values: HashMap::new(),
            order: Vec::with_capacity(capacity),
            capacity,
        }
    }

    fn get(&self, key: &u64) -> Option<bool> {
        Some(self.values.get(key).clone().is_some())
    }

    fn insert(&mut self, key: u64, value: bool) {
        if self.order.len() == self.capacity {
            if let Some(oldest) = self.order.pop() {
                self.values.remove(&oldest);
            }
        }

        self.values.insert(key, value);
        self.order.insert(0, key);
    }
}

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=(n as f64).sqrt() as u64 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {

    let cache = Rc::new(RefCell::new(Cache::new(10)));

    loop {
        print!("Enter a number (or 'exit' to quit): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        if input.trim().to_lowercase() == "exit" {
            break;
        }

        if let Ok(number) = input.trim().parse::<u64>() {
            let result = {
                let cache = cache.borrow();
                cache.get(&number).clone()
            };

            if let Some(result) = result {
                println!("Result from cache: {}", result);
            } else {
                let is_prime_result = is_prime(number);
                println!("Result calculated: {}", is_prime_result);

                let mut cache = cache.borrow_mut();
                cache.insert(number, is_prime_result);
            }
        } else {
            println!("Invalid input. Please enter a valid number.");
        }
    }
}
