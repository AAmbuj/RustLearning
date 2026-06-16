// LESSON 4: Structs & Implementations
// Like C++ structs but with built-in methods (impl blocks)

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Circle {
    center: Point,
    radius: f64,
}

// Implementation block-like adding member functions
impl Point {
    // Constructor (no special syntax like C++)
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }  // When field name matches variable, shorthand works
    }

    // Method (takes &self - borrows)
    fn distance_from_origin(&self) -> f64 {
        (((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()) as f64
    }

    // Method with mutable borrow (&mut self)
    fn translate(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    // Takes ownership (consumes self)
    fn into_tuple(self) -> (i32, i32) {
        (self.x, self.y)
    }
}

impl Circle {
    fn new(x: i32, y: i32, radius: f64) -> Circle {
        Circle {
            center: Point::new(x, y),
            radius,  // shorthand
        }
    }

    fn area(&self) -> f64 {
        3.14159 * self.radius * self.radius
    }

    fn contains_point(&self, point: &Point) -> bool {
        let dx = (self.center.x - point.x) as f64;
        let dy = (self.center.y - point.y) as f64;
        (dx * dx + dy * dy).sqrt() <= self.radius
    }
}

fn main() {
    println!("=== Lesson 4: Structs & Implementations ===\n");

    // Create Point
    let mut p1 = Point::new(3, 4);
    println!("Point: {:?}", p1);
    println!("Distance from origin: {}", p1.distance_from_origin());

    // Mutable method
    p1.translate(1, 1);
    println!("After translate: {:?}", p1);

    // Create Circle
    let circle = Circle::new(0, 0, 5.0);
    println!("\nCircle: {:?}", circle);
    println!("Circle area: {}", circle.area());

    // Check if point is in circle
    let p2 = Point::new(3, 4);
    if circle.contains_point(&p2) {
        println!("Point {:?} is in circle", p2);
    }

    // Ownership - consuming self
    let p3 = Point::new(10, 20);
    let tuple = p3.into_tuple();
    println!("\nConverted to tuple: {:?}", tuple);
    // println!("p3: {:?}", p3);  // ❌ ERROR: p3 was consumed

    println!("\n✓ Lesson 4 complete!");
    println!("Key concepts:");
    println!("1. Structs group related data");
    println!("2. impl blocks add methods");
    println!("3. &self for immutable methods");
    println!("4. &mut self for mutable methods");
    println!("5. self (no reference) consumes the value");
}
