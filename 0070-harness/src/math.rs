/// Adds two integers
///
/// This method adds to integers together and
/// returns the result.
///
/// # Examples
///
/// ```
/// let result = math::add(5, 3);
/// // result now has the value 8
/// # assert_eq!(result, 8);
/// ```
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

/// Divides two integers using integer division.
///
/// # Panics
///
/// Panics if `y` is zero.
pub fn div(x: i32, y: i32) -> i32 {
    x / y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        assert_eq!(add(5, 3), 8);
        assert_eq!(add(-2, 2), 0);
        assert_eq!(add(-4, -6), -10);
    }

    #[test]
    fn div_works() {
        assert_eq!(div(10, 2), 5);
        assert_eq!(div(7, 2), 3);
        assert_eq!(div(-9, 3), -3);
    }

    #[test]
    #[should_panic]
    fn div_by_zero_panics() {
        let _ = div(1, 0);
    }
}
