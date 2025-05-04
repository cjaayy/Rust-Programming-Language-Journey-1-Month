fn main() {
    // ---- Move Semantics (for types like String) ----
    let s1 = String::from("Rustacean");
    let s2 = s1; // Ownership of the heap-allocated string is moved to s2

    // println!("{}", s1); // ❌ Error: s1 no longer valid after move
    println!("s2 has the value: {}", s2);

    // If you want to keep both:
    let s3 = String::from("Clone me!");
    let s4 = s3.clone(); // Creates a deep copy
    println!("s3: {}, s4: {}", s3, s4);

    // ---- Copy Semantics (for simple types like integers) ----
    let x = 42;
    let y = x; // x is copied into y

    println!("x: {}, y: {}", x, y); // ✅ x is still valid

    // ---- Function example to show move ----
    let name = String::from("Alice");
    greet(name); // Ownership moves into the function

    // println!("{}", name); // ❌ name was moved and is no longer valid
}

fn greet(n: String) {
    println!("Hello, {}!", n);
}
