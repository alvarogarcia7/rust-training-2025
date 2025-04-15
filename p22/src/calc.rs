/// Convert Celsius to Fahrenheit degrees
///
/// Note: This function rounds to the nearest integer, half up. https://en.wikipedia.org/wiki/Rounding#Rounding_half_up
///
/// ```
/// use p22::calc::celsius2fahrenheit;
/// assert_eq!(celsius2fahrenheit(0), 32);
/// ```
pub fn celsius2fahrenheit(celsius: i32) -> i32 {
    let x: f32 = (celsius as f32 * 9f32) / 5f32;
    x.round() as i32 + 32
}

/// Convert Fahrenheit to Celsius degrees
///
/// ```
/// use p22::calc::fahrenheit2celsius;
/// assert_eq!(fahrenheit2celsius(32), 0);
/// ```
pub fn fahrenheit2celsius(fahrenheit: i32) -> i32 {
    (5 * (fahrenheit - 32)) / 9
}

const PRECOMPUTED_VALUES: [u64; 3] = [0, 1, 1];

/// Fibonacci series, in the loop form
pub fn fibonacci_loop(limit: u32) -> u64 {
    let mut n = if limit == 0 {
        PRECOMPUTED_VALUES[0]
    } else if limit == 1 {
        PRECOMPUTED_VALUES[1]
    } else {
        PRECOMPUTED_VALUES[2]
    };
    let mut n_minus_1 = PRECOMPUTED_VALUES[1];
    let mut n_minus_2 = PRECOMPUTED_VALUES[0];
    for _ in 2..=limit {
        // compute: fib(n) = fib(n-1) + fib(n-2)
        n = n_minus_1 + n_minus_2;
        // shift the previous values
        n_minus_2 = n_minus_1;
        n_minus_1 = n;
    }
    n
}

/// Fibonacci series, in the recursive form
pub fn fibonacci_rec(limit: u32) -> u64 {
    if limit == 0 {
        PRECOMPUTED_VALUES[0]
    } else if limit == 1 {
        PRECOMPUTED_VALUES[1]
    } else {
        fibonacci_rec(limit - 1) + fibonacci_rec(limit - 2)
    }
}
