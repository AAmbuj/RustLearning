// LESSON 2: Ownership & Borrowing
// This is THE core concept of Rust - the main difference from C++/Python!

fn main() {
    println!("=== Lesson 2: Ownership & Borrowing ===\n");

    // RULE 1: Each value has ONE owner
    let s1 = String::from("hello Ambuj");
    let s2 = s1;  // ownership MOVES from s1 to s2
    
    println!("s2: {}", s2);
    // println!("s1: {}", s1);  // ❌ ERROR: s1 no longer owns the data!

    println!("\n--- Borrowing (Immutable References) ---");
    let s3 = String::from("rust");
    let len = calculate_length(&s3);  // Borrow s3, don't take ownership
    
    println!("s3: {}, length: {}", s3, len);  // s3 still works!
    println!("s3 still owns its data: {}", s3);

    println!("\n--- Mutable Borrowing ---");
    let mut s4 = String::from("hello");
    modify_string(&mut s4);  // Mutable borrow
    println!("After modification: {}", s4);

    println!("\n--- Ownership Transfer (Move) ---");
    let s5 = String::from("owned");
    take_ownership(s5);  // Function takes ownership
    // println!("s5: {}", s5);  // ❌ ERROR: s5's data moved to function

    println!("\n--- Return Ownership ---");
    let s6 = give_ownership();
    println!("s6 received: {}", s6);

    println!("\n✓ Lesson 2 complete!");
    println!("Key rules:");
    println!("1. Each value has one owner");
    println!("2. & creates immutable borrow (many allowed)");
    println!("3. &mut creates mutable borrow (only one allowed)");
}

// Function that borrows (& reference)
// Doesn't take ownership - can still use original
fn calculate_length(s: &String) -> usize {
    s.len()
}  // s goes out of scope, but it doesn't own the String

// Function with mutable borrow
fn modify_string(s: &mut String) {
    s.push_str(" world");
}

// Function that takes ownership
fn take_ownership(s: String) {
    println!("Function received: {}", s);
}  // s is dropped here

// Function that gives ownership
fn give_ownership() -> String {
    String::from("yours")
}
