fn add(a: i32, b: i32) -> i32 {
    let result = { 
        dbg!(a, b);
        a + b 
    };
    result
}

fn div(a: i32, b: i32) -> i32 {
    let result = if b == 0 { 0 } else { a / b };
    // return b == 0 ? 0 : a / b; // This syntax would be e.g. in C
    result
}

fn find_first_fib_over_20() -> i32 {
    let mut a = 0;
    let mut b = 1;
    let result = loop {
        let next = a + b;
        if next > 20 {
            break next;
        }
        a = b;
        b = next;
    };
    result
}

fn return_something() -> f32 { // Return type must be fixed, generics are obviously possible (later)
    let a = "42";
    a.parse().unwrap()
}

fn function_in_functions() {
    fn inner_function(x: i32) -> i32 {
        x * 2
    }
    let result = inner_function(10);
    println!("The result of the inner function is: {}", result);
}

fn simple_closures() {
    let factor = 5;
    let add_one = |x: i32| x + 1 + factor;
    let result = add_one(5);
    println!("The result of adding one to 5 is: {}", result);
}

fn main() {
    let result = add(5, 3);
    println!("The result of adding 5 and 3 is: {}", result);    
}
