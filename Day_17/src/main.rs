fn main() {
    let s1 = String::from("hello");

    // Immutable borrow
    let len = calculate_length(&s1); // borrow s1 using &

    println!("The length of '{}' is {}.", s1, len); // ✅ s1 is still valid!

    // Mutable borrow
    let mut s2 = String::from("hello");
    change(&mut s2); // borrow mutably

    println!("After change: {}", s2);
}

// Immutable borrow (read-only)
fn calculate_length(s: &String) -> usize {
    s.len() // just reading, not modifying
}

// Mutable borrow (allows modification)
fn change(s: &mut String) {
    s.push_str(", world!");
}
