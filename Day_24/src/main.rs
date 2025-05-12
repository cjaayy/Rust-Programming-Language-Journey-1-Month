fn main() {
    println!("Hello, world!");
}
// Named-field struct
struct Employee {
    name: String,
    id: u32,
    position: String,
}

// Tuple struct
struct Color(u8, u8, u8);

// Unit-like struct (no fields)
struct Marker;

fn main() {
    // Using the named-field struct
    let emp = Employee {
        name: String::from("Alice"),
        id: 101,
        position: String::from("Developer"),
    };
    println!("Employee: {}, ID: {}, Position: {}", emp.name, emp.id, emp.position);

    // Using the tuple struct
    let red = Color(255, 0, 0);
    println!("Favorite Color - RGB: ({}, {}, {})", red.0, red.1, red.2);

    // Destructuring the tuple struct
    let Color(r, g, b) = red;
    println!("Destructured Color: R={}, G={}, B={}", r, g, b);

    // Using the unit-like struct
    let _mark = Marker;
    println!("Marker struct created. It has no data but exists as a type.");
}
