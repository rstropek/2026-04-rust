mod math;

fn main() {
    let result = math::add(5, 3);
    println!("The result is: {}", result);
    let quotient = math::div(10, 3);
    println!("10 / 3 = {}", quotient);
}
