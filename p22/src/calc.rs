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
