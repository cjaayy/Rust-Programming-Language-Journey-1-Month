fn main() {
    let name = "Alice";
    let age = 25;
    let height = 170.5;

    // Basic printing
    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Height: {} cm", height);

    // Positional arguments
    println!("{0} is {1} years old. {0} is {2} cm tall.", name, age, height);

    // Named arguments
    println!(
        "{name} is {age} years old and {height} cm tall.",
        name = name,
        age = age,
        height = height
    );

    // Formatting numbers
    let pi = 3.141592;
    println!("Pi rounded to two decimals: {:.2}", pi);

    // Padding and alignment
    println!("Left aligned:  |{:<10}|", name);
    println!("Right aligned: |{:>10}|", name);
    println!("Center aligned:|{:^10}|", name);

    // Binary, Hex, Octal
    let number = 42;
    println!("Binary: {:b}", number);
    println!("Hex: {:x}", number);
    println!("Octal: {:o}", number);
}
