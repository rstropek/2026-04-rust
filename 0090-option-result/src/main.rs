#![allow(unused_variables, dead_code)]

fn main() {
    {
        let some = Some(42);
        let none: Option<i32> = None;

        // Turn option into bool
        dbg!(some.is_some());
        dbg!(some.is_none());

        // Turn option into bool WITH additional condition
        dbg!(some.is_some_and(|n| n > 40));
        dbg!(none.is_none_or(|n| n < 40));

        // Extract value
        let payload = some.unwrap();
        let payload = some.expect("this value must be set");
        let payload = some.unwrap_or(0);
        dbg!(payload);

        // Combining options
        let a = Some(1);
        let b = Some(2);
        dbg!(a.and(b));

        // References
        let text = Some("Hello world!".to_string());
        let text: Option<String> = None;
        // (&)Option<String> -> Option<&String>
        let x = (&text).as_ref();
        dbg!(x);

        let number = Some(Box::new("42".to_string()));
        let number = number.as_deref();

        let mut x = Some("42".to_string());
        let old = x.take(); // x -> None, old -> Some(42)
    }

    {
        let result = try_div(10, 5);
        match result {
            Ok(value) => println!("Result: {value}"),
            Err(error) => match error {
                DivisionError::TooBig => println!("Too big!"),
                DivisionError::DivisionByZero => println!("Division by zero!"),
            },
        }
    }
}

enum DivisionError {
    TooBig,
    DivisionByZero,
}

fn try_div(a: i32, b: i32) -> Result<i32, DivisionError> {
    // Let's invent something: We can only divide numbers up to 1000 (a <= 1000)

    match (a, b) {
        (a, _) if a > 1000 => Err(DivisionError::TooBig),
        (_, 0) => Err(DivisionError::DivisionByZero),
        (a, b) => Ok(a / b),
    }
}

struct User {
    name: String,
    profile_picture: Option<Vec<u8>>,
}

impl User {
    fn has_profile_picture(&self) -> bool {
        self.profile_picture.is_some()
    }

    fn profile_picture_size(&self) -> usize {
        // &Option<Vec<u8>> -> Option<&Vec<u8>>
        let pic = &self.profile_picture;
        pic.as_ref().map(|data| data.len()).unwrap_or(0)
    }
}
