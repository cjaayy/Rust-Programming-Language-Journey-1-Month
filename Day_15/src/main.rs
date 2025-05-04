fn main() {
    // String is stored on the heap (heap-allocated, growable)
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2 — ownership is transferred!

    // println!("{}", s1); // ❌ This will cause a compile-time error because s1 no longer owns the data.

    println!("s2 owns the string now: {}", s2);

    // To keep both, you need to clone:
    let s3 = String::from("world");
    let s4 = s3.clone(); // Now both s3 and s4 own separate data

    println!("s3: {}, s4: {}", s3, s4);

    // Primitive types implement Copy trait
    let x = 5;
    let y = x; // x is copied, not moved

    println!("x: {}, y: {}", x, y); // ✅ works fine
}
