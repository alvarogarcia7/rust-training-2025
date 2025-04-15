#[cfg(test)]
mod tests {
    use p22::calc::celsius2fahrenheit;

    #[test]
    fn test_celsius2fahrenheit() {
        assert_eq!(celsius2fahrenheit(0), 32);
    }
}
