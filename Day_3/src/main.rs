// 1. `let` - Immutable variable
fn main() {
    let x = 5;
    println!("Immutable variable x = {}", x);
    // x = 6; // ❌ Error: cannot assign twice to immutable variable

    // 2. `mut` - Mutable variable
    let mut y = 10;
    println!("Mutable variable before: y = {}", y);
    y = 15; // ✅ Can change value
    println!("Mutable variable after: y = {}", y);

    // 3. `const` - Constant value, must have type annotation and is always immutable
    const MAX_POINTS: u32 = 100;
    println!("Constant MAX_POINTS = {}", MAX_POINTS);

    // 4. Shadowing - Re-declaring a variable with the same name
    let z = 5;
    let z = z + 1; // shadows previous z
    let z = z * 2; // shadows again
    println!("Shadowed variable z = {}", z); // prints 12

    // Shadowing can also change the type
    let spaces = "   ";
    let spaces = spaces.len(); // now it's an integer!
    println!("Shadowed variable spaces = {}", spaces); // prints 3
}
