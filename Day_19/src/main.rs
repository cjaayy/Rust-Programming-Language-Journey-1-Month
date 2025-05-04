fn main() {
    // --- STRING SLICE ---
    let sentence = String::from("hello world from rust");

    let first = first_word(&sentence);
    let part = &sentence[6..11]; // "world"

    println!("Original sentence: {}", sentence);
    println!("First word: {}", first);
    println!("Second word: {}", part);

    // --- ARRAY SLICE ---
    let numbers = [4, 10, 23, 7, 19];

    let slice = &numbers[1..4]; // elements: 10, 23, 7
    let max = find_max(slice);

    println!("Array: {:?}", numbers);
    println!("Sliced part: {:?}", slice);
    println!("Maximum in slice: {}", max);
}

// Function to get the first word from a string slice
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..] // return entire string if no space found
}

// Function to find the max value in an integer slice
fn find_max(slice: &[i32]) -> i32 {
    let mut max = slice[0];
    for &num in slice.iter() {
        if num > max {
            max = num;
        }
    }
    max
}
