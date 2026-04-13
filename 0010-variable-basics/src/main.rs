#![allow(unused_variables, dead_code, unused_assignments)]

const MY_VALUE: i32 = 42;

fn main() {
    let i = 42;
    let i_as_usize = i as usize;
    println!("Hello, world! The value of i is: {}", i);
    
    let a = 42;
    let b = 21i32;
    let c = a + b;
    dbg!(a, b, c);
    
    let d;
    if true {
        d = 1;
    } else {
        d = 2i16;
    }
    
    let numbers = [1, 2, 3, 4];
    let my_number: i16 = numbers[0];
    
    let a = 15;
    let a = 15.0; // Shaddowing
    let a = "15";
    
    let user_input = "42";
    let user_input = user_input.parse::<i32>().unwrap(); // Turbofish

    let user_input = "42";
    let user_input: i32 = user_input.parse().unwrap();
    
    let mut x = 1;
    x += 1;
    let x = x;
    {
        let mut x = x;
        x += 1;
    }
    let mut x = x;
    x += 1;
    {
        let x = x;
        // x += 1; // Not possible
    }
    x += 1;
}
