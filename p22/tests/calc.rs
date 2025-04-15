#[cfg(test)]
mod celsius2fahrenheit_tests {
    use p22::calc::celsius2fahrenheit;

    #[test]
    fn test_celsius2fahrenheit_at_0() {
        assert_eq!(celsius2fahrenheit(0), 32);
    }

    #[test]
    fn test_celsius2fahrenheit_at_2() {
        assert_eq!(celsius2fahrenheit(2), 36);
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
#[cfg(test)]
mod fahrenheit2celsius_tests {
    use p22::calc::fahrenheit2celsius;

    #[test]
    fn test_fahrenheit2celsius_at_0() {
        assert_eq!(fahrenheit2celsius(32), 0);
    }

    #[test]
    fn test_fahrenheit2celsius_at_2() {
        assert_eq!(fahrenheit2celsius(36), 2);
    }
    #[test]
    fn test_fahrenheit2celsius_at_20() {
        assert_eq!(fahrenheit2celsius(68), 20);
    }
    #[test]
    fn test_fahrenheit2celsius_at_minus_15() {
        assert_eq!(fahrenheit2celsius(5), -15);
    }
}

#[cfg(test)]
mod fibonacci_tests {
    use p22::calc::fibonacci_loop;

    #[test]
    fn test_fibonacci() {
        assert_eq!(
            (0..13).map(fibonacci_loop).collect::<Vec<_>>(),
            [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144]
        );
    }
}
