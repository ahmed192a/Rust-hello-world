
// create test cases
#[cfg(test)]
mod tests {
    // import the calculate function
    use super::super::methods::calculate;

    // test the calculate function
    #[test] 
    fn test_calculate_sum() {
        // assert that the result is correct
        assert_eq!(calculate('+', 1.0, 1.0), 2.0);
    }

    // test the calculate function
    #[test]
    fn test_calculate_difference() {
        // assert that the result is correct
        assert_eq!(calculate('-', 1.0, 1.0), 0.0);
    }

    // test the calculate function
    #[test]
    fn test_calculate_product() {
        // assert that the result is correct
        assert_eq!(calculate('*', 1.0, 1.0), 1.0);
    }

    // test the calculate function
    #[test]
    fn test_calculate_quotient() {
        // assert that the result is correct
        assert_eq!(calculate('/', 1.0, 1.0), 1.0);
    }
    // diabled test
    #[test]
    fn test_calculate_invalid_operation() {
        // assert that the result is correct
        assert_eq!(calculate('a', 1.0, 1.0), 0.0);
    }
}
