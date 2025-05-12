// Define a struct named `Employee`
struct Employee {
    name: String,
    id: u32,
    is_active: bool,
    position: String,
    age: u8,
}

fn main() {
    // Create an instance of Employee
    let emp1 = Employee {
        name: String::from("Alice"),
        id: 1001,
        is_active: true,
        position: String::from("Developer"),
        age: 28,
    };

    // Access struct fields
    println!("Employee Info:");
    println!("Name: {}", emp1.name);
    println!("ID: {}", emp1.id);
    println!("Position: {}", emp1.position);
    println!("Active: {}", emp1.is_active);
    println!("Age: {}", emp1.age);

    // You can also use destructuring
    let Employee { name, id, .. } = emp1;
    println!("\nDestructured -> Name: {}, ID: {}", name, id);
}
