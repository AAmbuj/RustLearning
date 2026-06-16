// LESSON 5: Error Handling with Result
// Rust's approach to errors - no try/catch!

use std::num::ParseIntError;
use std::fs::File;
use std::io::Read;

// Define custom error types
#[derive(Debug)]
enum DivisionError {
    DivideByZero,
    NegativeNumber(i32),
}

// Function that returns Result
fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {
    if b == 0 {
        Err(DivisionError::DivideByZero)
    } else if a < 0 || b < 0 {
        Err(DivisionError::NegativeNumber(a.min(b)))
    } else {
        Ok(a / b)
    }
}

// Parse string to number - can fail
fn parse_numbers(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input.split(',')
        .map(|s| s.trim().parse::<i32>())
        .collect()
}

// Helper function with ? operator (propagates errors)
fn safe_divide_and_double(a: i32, b: i32) -> Result<i32, DivisionError> {
    let result = divide(a, b)?;  // ? automatically returns error if result is Err
    Ok(result * 2)
}

fn main() {
    println!("=== Lesson 5: Error Handling with Result ===\n");

    // Method 1: Match pattern
    println!("--- Method 1: Pattern Matching ---");
    match divide(10, 2) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Error: {:?}", e),
    }

    match divide(10, 0) {
        Ok(result) => println!("10 / 0 = {}", result),
        Err(DivisionError::DivideByZero) => println!("Cannot divide by zero"),
        Err(DivisionError::NegativeNumber(n)) => println!("Negative number: {}", n),
    }

    // Method 2: if let (cleaner when you only care about Ok)
    println!("\n--- Method 2: if let ---");
    if let Ok(result) = divide(20, 4) {
        println!("20 / 4 = {}", result);
    }

    // Method 3: unwrap (be careful - panics on error!)
    println!("\n--- Method 3: unwrap (DANGEROUS!) ---");
    let result = divide(15, 3).unwrap();
    println!("15 / 3 = {}", result);

    // Method 4: unwrap_or (with default)
    println!("\n--- Method 4: unwrap_or ---");
    let result = divide(10, 0).unwrap_or(-1);
    println!("Result (or -1): {}", result);

    // Method 5: ? operator (propagates errors)
    println!("\n--- Method 5: ? operator ---");
    match safe_divide_and_double(10, 2) {
        Ok(n) => println!("10 / 2 * 2 = {}", n),
        Err(e) => println!("Error: {:?}", e),
    }

    // Parsing with error propagation
    println!("\n--- Parsing Example ---");
    match parse_numbers("1, 2, 3, 4") {
        Ok(numbers) => println!("Parsed: {:?}", numbers),
        Err(e) => println!("Parse error: {}", e),
    }

    match parse_numbers("1, 2, invalid, 4") {
        Ok(numbers) => println!("Parsed: {:?}", numbers),
        Err(e) => println!("Parse error: {}", e),
    }

    println!("\n✓ Lesson 5 complete!");
    println!("Key concepts:");
    println!("1. Result<T, E> replaces try/catch");
    println!("2. match for complete error handling");
    println!("3. if let for simple cases");
    println!("4. ? operator for error propagation");
    println!("5. unwrap() panics on error (avoid in production)");
    println!("6. unwrap_or() provides default value");
}
