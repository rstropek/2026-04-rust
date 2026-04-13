#![allow(unused_variables)]

use rand::prelude::*;

fn main() {
    // Tuple
    let a = 42;
    let a = (42, true);
    let first = a.0;
    let second = a.1;
    
    let unit = ();
    
    let (first, second) = a;
    dbg!(first);
    dbg!(second);
    
    let (first, _) = a;
    dbg!(first);
    
    let a = 1;
    let b = 2;
    let (b, a) = (a, b);
    
    // Ranges
    let r = 1..10; // 1, 2, 3, 4, 5, 6, 7, 8, 9
    let r = 1..=10;
    for i in 1..=10 {
        println!("{}", i);
    }
    
    // Arrays
    let mut numbers = [1, 2, 3, 4, 5];
    let first = numbers[0];
    numbers[0] = 42;
    dbg!(numbers);
    
    let numbers: [u8; 500] = [1; _];
    
    // Random numbers
    let mut rng = rand::rng();
    let my_number = rng.random_range(0..=10);
    dbg!(my_number);
}
