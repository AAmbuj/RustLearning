// LESSON 1: Variables, Mutability, and Basic Types
// Rust key difference: variables are IMMUTABLE by default!

fn main() {
    println!("=== Lesson 1: Variables & Mutability ===\n");

    // 1. Immutable variable (default in Rust - different from C++!)
    let x = 5;
    println!("Immutable x: {}", x);
    // x = 6;  // ❌ ERROR: cannot assign twice to immutable variable
    
    // 2. Mutable variable (need 'mut' keyword)
    let mut y = 5;
    y = 6;  // ✓ OK
    println!("Mutable y: {}", y);

    // 3. Type annotation (optional - Rust does type inference)
    let z: i32 = 100;
    println!("Explicit type z: {}", z);

    // 4. Shadowing (rebinding with let - unique to Rust!)
    let message = "Hello";
    let message = message.len();  // ✓ OK - different type!
    println!("Shadowed message (length): {}", message);

    // 5. Basic types
    let integer: i32 = -42;           // 32-bit signed
    let unsigned: u32 = 42;           // 32-bit unsigned
    let float: f64 = 3.14;            // floating point
    let boolean: bool = true;         // boolean
    let character: char = 'A';        // single character (4 bytes, not 1!)

    println!("\nTypes:");
    println!("i32: {}, u32: {}, f64: {}, bool: {}, char: {}", 
             integer, unsigned, float, boolean, character);

    // 6. Tuples (grouping multiple types)
    let person: (String, i32, f64) = ("Alice".to_string(), 30, 5.6);
    println!("\nTuple - Name: {}, Age: {}, Height: {}", 
             person.0, person.1, person.2);

    // 7. Arrays (fixed size, like C++)
    let numbers: [i32; 3] = [1, 2, 3];
    println!("Array: {:?}", numbers);
    println!("First element: {}", numbers[0]);

    println!("\n✓ Lesson 1 complete! Key takeaway: immutable by default!");
}
