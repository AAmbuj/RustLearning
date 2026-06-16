// LESSON 6: Collections & Iterators
// Work with data efficiently using Rust's powerful collection types

use std::collections::HashMap;

fn main() {
    println!("=== Lesson 6: Collections & Iterators ===\n");

    // Vec - dynamic array (like C++ vector)
    println!("--- Vec (Dynamic Array) ---");
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("Original: {:?}", numbers);

    numbers.push(6);
    println!("After push: {:?}", numbers);

    // Iteration methods
    println!("\nIteration methods:");
    println!("for loop:");
    for n in &numbers {  // &numbers - borrow to iterate without consuming
        print!("{} ", n);
    }
    println!();

    // Still can use numbers after iteration (because we borrowed)
    println!("numbers still works: {:?}\n", numbers);

    // Iterator methods (functional style)
    println!("Iterator chain:");
    let result: Vec<i32> = numbers.iter()
        .map(|x| x * 2)              // double each
        .filter(|x| x % 4 == 0)      // keep divisible by 4
        .collect();                   // collect results
    println!("Doubled and filtered: {:?}", result);

    // Sum and other aggregations
    let sum: i32 = numbers.iter().sum();
    let count = numbers.len();
    let average = sum as f64 / count as f64;
    println!("Sum: {}, Count: {}, Average: {:.2}", sum, count, average);

    // HashMap - key-value store (like C++ map or Python dict)
    println!("\n--- HashMap ---");
    let mut scores: HashMap<String, i32> = HashMap::new();
    
    scores.insert("Alice".to_string(), 100);
    scores.insert("Bob".to_string(), 85);
    scores.insert("Charlie".to_string(), 92);

    println!("All scores: {:?}", scores);

    // Get value
    if let Some(score) = scores.get("Alice") {
        println!("Alice's score: {}", score);
    }

    // Iterate over HashMap
    println!("\nScores in order (unordered):");
    for (name, score) in &scores {
        println!("  {}: {}", name, score);
    }

    // HashMap iterator chains
    let above_90: Vec<_> = scores.iter()
        .filter(|(_, score)| **score > 90)
        .map(|(name, _)| name.clone())
        .collect();
    println!("Scores above 90: {:?}", above_90);

    // String operations
    println!("\n--- String Operations ---");
    let words = vec!["Hello", "from", "Rust"];
    let sentence: String = words.join(" ");
    println!("Joined: {}", sentence);

    let parts: Vec<&str> = sentence.split_whitespace().collect();
    println!("Split: {:?}", parts);

    // Closures with iterators
    println!("\n--- Closures & Iterators ---");
    let numbers = vec![1, 2, 3, 4, 5];
    
    // Closure as parameter
    let doubled: Vec<i32> = numbers.iter()
        .map(|n| n * 2)
        .collect();
    println!("Doubled: {:?}", doubled);

    // Find with closure
    let first_even = numbers.iter().find(|n| *n % 2 == 0);
    if let Some(n) = first_even {
        println!("First even: {}", n);
    }

    // Any, all predicates
    let all_positive = numbers.iter().all(|n| n > &0);
    let has_even = numbers.iter().any(|n| n % 2 == 0);
    println!("All positive: {}, Has even: {}", all_positive, has_even);

    println!("\n✓ Lesson 6 complete!");
    println!("Key concepts:");
    println!("1. Vec for dynamic arrays");
    println!("2. HashMap for key-value storage");
    println!("3. Iterators with .iter(), .map(), .filter()");
    println!("4. Closures (|x| x * 2) for inline functions");
    println!("5. Method chaining for data transformations");
    println!("6. .collect() to gather iterator results");
}
