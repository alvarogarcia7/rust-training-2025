pub fn celsius2fahrenheit(celsius: i32) -> i32 {
    ((celsius * 9) / 5) + 32
}
pub fn fahrenheit2celsius(fahrenheit: i32) -> i32 {
    (5 * (fahrenheit - 32)) / 9
}
