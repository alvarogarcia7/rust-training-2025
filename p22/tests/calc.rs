#[cfg(test)]
mod tests {
    use p22::calc::celsius2fahrenheit;

    #[test]
    fn test_celsius2fahrenheit_at_0() {
        assert_eq!(celsius2fahrenheit(0), 32);
    }
    #[test]
    fn test_celsius2fahrenheit_at_2() {
        assert_eq!(celsius2fahrenheit(2), 35);
    }
    #[test]
    fn test_celsius2fahrenheit_at_20() {
        assert_eq!(celsius2fahrenheit(20), 68);
    }
    #[test]
    fn test_celsius2fahrenheit_at_minus_15() {
        assert_eq!(celsius2fahrenheit(-15), 5);
    }
}
