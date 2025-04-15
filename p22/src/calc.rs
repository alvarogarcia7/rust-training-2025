pub fn celsius2fahrenheit(celsius: i32) -> i32 {
    let x: f32 = (celsius as f32 * 9f32) / 5f32;
    x.round() as i32 + 32
}
pub fn fahrenheit2celsius(fahrenheit: i32) -> i32 {
    (5 * (fahrenheit - 32)) / 9
}
