fn main() {
    // --- Ownership Transfer Example ---
    let name = String::from("Rustacean");
    print_name(name); // ownership moved to the function

    // println!("{}", name); // ❌ Error: `name` moved, cannot use here

    // --- Borrowing Example ---
    let phrase = String::from("Ownership and borrowing are key!");
    let vowels = count_vowels(&phrase); // borrowed as &str

    println!("Phrase: {}", phrase); // ✅ Still usable
    println!("Vowel count: {}", vowels);

    // --- Array Borrowing Example ---
    let nums = [4, 8, 15, 16, 23, 42];
    let max = find_max(&nums); // borrow slice

    println!("Array: {:?}, Max: {}", nums, max);

    // --- Dangling Reference Example (will NOT compile) ---
    // let r = dangle(); // ❌ Error: cannot return reference to local variable
    // println!("{}", r);
}

// Function takes ownership of the string
fn print_name(n: String) {
    println!("Owned name: {}", n);
}

// Borrowing string slice to count vowels
fn count_vowels(s: &str) -> usize {
    s.chars()
        .filter(|c| matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u'))
        .count()
}

// Borrowing an array slice to find the max number
fn find_max(slice: &[i32]) -> i32 {
    let mut max = slice[0];
    for &n in slice.iter() {
        if n > max {
            max = n;
        }
    }
    max
}

// This function will not compile due to dangling reference
// fn dangle() -> &String {
//     let s = String::from("oops");
//     &s
// }
