// LESSON 3: Enums & Pattern Matching
// Rust's enums are more powerful than C++ - they can hold values!

use std::fmt;

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
    RGB(u8, u8, u8),      // Enum variant with data
    CMYK { cyan: u8, magenta: u8, yellow: u8, key: u8 },  // Named fields
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    println!("=== Lesson 3: Enums & Pattern Matching ===\n");

    // Basic enum usage
    let color1 = Color::Red;
    let color2 = Color::RGB(255, 0, 0);
    let color3 = Color::CMYK { cyan: 0, magenta: 255, yellow: 255, key: 0 };

    println!("color1: {:?}", color1);
    println!("color2: {:?}", color2);
    println!("color3: {:?}", color3);

    // Pattern matching - the power of Rust enums!
    println!("\n--- Pattern Matching ---");
    match color2 {
        Color::Red => println!("It's red!"),
        Color::Green => println!("It's green!"),
        Color::Blue => println!("It's blue!"),
        Color::RGB(r, g, b) => println!("RGB: ({}, {}, {})", r, g, b),
        Color::CMYK { cyan, magenta, yellow, key } => {
            println!("CMYK: C={}, M={}, Y={}, K={}", cyan, magenta, yellow, key);
        }
    }

    // Option enum (built-in) - replaces null pointers!
    println!("\n--- Option (No null pointers!) ---");
    let maybe_number: Option<i32> = Some(5);
    let no_number: Option<i32> = None;

    match maybe_number {
        Some(n) => println!("Found number: {}", n),
        None => println!("No number here"),
    }

    // Result enum (built-in) - replaces exceptions!
    println!("\n--- Result (No exceptions!) ---");
    let success: std::result::Result<i32, String> = Ok(42);
    let failure: std::result::Result<i32, String> = Err("Something went wrong".to_string());

    match success {
        Ok(n) => println!("Success: {}", n),
        Err(e) => println!("Error: {}", e),
    }

    match failure {
        Ok(n) => println!("Success: {}", n),
        Err(e) => println!("Error: {}", e),
    }

    // Safe way to extract values
    println!("\n--- Safe extraction ---");
    if let Some(n) = maybe_number {
        println!("Using if let: {}", n);
    }

    // Unwrap (be careful!)
    println!("Unwrap: {}", maybe_number.unwrap());
    // This would panic: println!("Unwrap: {}", no_number.unwrap());

    println!("\n✓ Lesson 3 complete!");
    println!("Key concepts:");
    println!("1. Enums can hold data (unlike C++)");
    println!("2. Pattern matching extracts enum values safely");
    println!("3. Option replaces null pointers");
    println!("4. Result replaces exceptions");
}
