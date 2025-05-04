fn main() {
    let input = String::from("Hello, Rustaceans!");
    let count = count_vowels(&input);

    println!("Input: {}", input);
    println!("Number of vowels: {}", count);
}

// Function that takes a string slice and returns the number of vowels
fn count_vowels(s: &str) -> usize {
    let mut count = 0;

    for c in s.chars() {
        match c.to_ascii_lowercase() {
            'a' | 'e' | 'i' | 'o' | 'u' => count += 1,
            _ => (),
        }
    }

    count
}
