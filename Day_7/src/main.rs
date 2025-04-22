use std::io;

fn main() {
    println!("📐 Area Calculator: Rectangle");

    // Input: Length
    println!("Enter the length of the rectangle:");
    let mut length_input = String::new();
    io::stdin().read_line(&mut length_input).expect("Failed to read input");
    let length: f64 = length_input.trim().parse().expect("Please enter a valid number");

    // Input: Width
    println!("Enter the width of the rectangle:");
    let mut width_input = String::new();
    io::stdin().read_line(&mut width_input).expect("Failed to read input");
    let width: f64 = width_input.trim().parse().expect("Please enter a valid number");

    // Calculate area
    let area = length * width;

    // Output the result
    println!("The area of the rectangle is: {} square units", area);
}
