/// A simple calculator struct
pub struct Calculator;

impl Calculator {
    /// Adds two numbers
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    /// Subtracts second number from the first
    pub fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }

    /// Multiplies two numbers
    pub fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }

    /// Divides first number by the second (integer division)
    /// Panics if the second number is zero
    pub fn divide(a: i32, b: i32) -> i32 {
        if b == 0 {
            panic!("Cannot divide by zero");
        }
        a / b
    }
}

#[cfg(test)]
mod tests {
    use super::Calculator;

    #[test]
    fn test_add() {
        assert_eq!(Calculator::add(2, 3), 5);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(Calculator::subtract(10, 4), 6);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(Calculator::multiply(3, 7), 21);
    }
}
