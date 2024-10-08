// Assignment 1: Temperature Converter

// Declare a constant for the freezing point of water in Fahrenheit (32°F).
const FREEZE:f64 = 32.0;

// fahrenheit_to_celsius(f: f64) -> f64: Converts Fahrenheit to Celsius
fn fahrenheit_to_celsius(f: f64) -> f64 {
    let celsius = (f - FREEZE) * 5.0/9.0;
    return celsius;
}

// celsius_to_fahrenheit(c: f64) -> f64: Converts Celsius to Fahrenheit
fn celsius_to_fahrenheit(c: f64) -> f64 {
    let fahrenheit = (c * 9.0/5.0) + FREEZE;
    return fahrenheit;
}

// In the main function:
fn main() {
    // Declare a mutable variable with a temperature in Fahrenheit
    let mut temp = 72.0;
    // Convert it to Celsius using your function and print the result
    let mut f_c = fahrenheit_to_celsius(temp);
    // Also did it the other way around :)
    let mut c_f = celsius_to_fahrenheit(f_c);
    
    println!("F to C: {:.2}", f_c);
    println!("C to F: {:.2}", c_f);
    
    // Use a loop to convert and print the next 5 integer temperatures
    let mut count = 0;
    loop {
        count += 1;
        temp += 1.0;
        f_c = fahrenheit_to_celsius(temp);
        c_f = celsius_to_fahrenheit(f_c);
        
        println!("F to C: {:.2}", f_c);
        println!("C to F: {:.2}", c_f);
        if count == 5 {
            break;
        }
    }
}
