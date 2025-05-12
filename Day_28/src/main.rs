fn main() {
    // ✅ &str - string slice, immutable, usually hardcoded
    let greeting: &str = "Hello";
    println!("&str example: {}", greeting);

    // ✅ String - growable, heap-allocated
    let mut name = String::from("Rust");
    println!("String example: {}", name);

    // ✅ push_str() - add a &str to String
    name.push_str("acean");
    println!("After push_str: {}", name); // "Rustacean"

    // ✅ push() - add a single char
    name.push('!');
    println!("After push: {}", name); // "Rustacean!"

    // ✅ replace() - create a new string with replacements
    let excited = name.replace("Rust", "Crab");
    println!("After replace: {}", excited); // "Crabacean!"

    // ✅ Conversion: &str -> String
    let from_slice = "Programming".to_string();
    println!("Converted &str to String: {}", from_slice);

    // ✅ Conversion: String -> &str
    let as_str: &str = &from_slice;
    println!("Converted String to &str: {}", as_str);

    // ✅ Loop through characters
    println!("Characters in '{}':", name);
    for ch in name.chars() {
        print!("{} ", ch);
    }
    println!();
}
