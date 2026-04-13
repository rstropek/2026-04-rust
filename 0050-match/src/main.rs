use rand::prelude::*;

fn main() {
    let mut rng = rand::rng();
    let i = rng.random_range(0..10);
    
    let message = match i {
        n if n < 5 => "you lost",
        5 => "draw",
        6..=9 => "you win",
        _ => unreachable!(),
    };
    println!("{}", message);
    
    let i = rng.random_bool(0.5);
    println!("{}", match i {
        true => "you win",
        false => "you lost",
    });
    
    let i = (rng.random_range(0..10), rng.random_bool(0.5));
    println!("{}", match i {
        (n, true) if n < 5 => "you lost",
        (5, true) => "draw",
        (6..=9, true) => "you win",
        (_, false) => "you lost",
        _ => unreachable!(),
    });
}
