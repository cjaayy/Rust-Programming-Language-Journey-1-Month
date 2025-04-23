// Define a function that takes two numbers and returns their sum
fn add(a: i32, b: i32) -> i32 {
    a + b // return value (no semicolon means it's returned)
}

// Function that prints a greeting message
fn greet(name: &str) {
    println!("Hello, {}! 👋", name);
}

// Function that returns the square of a number
fn square(num: i32) -> i32 {
    return num * num; // explicit return
}

fn main() {
    // Calling greet function
    greet("Rustacean");

    // Using add function
    let sum = add(10, 5);
    println!("10 + 5 = {}", sum);

    // Using square function
    let result = square(6);
    println!("The square of 6 is {}", result);
}
