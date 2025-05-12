fn main() {
    // ✅ Create a vector
    let mut numbers: Vec<i32> = Vec::new();

    // ✅ Push elements into the vector
    numbers.push(10);
    numbers.push(20);
    numbers.push(30);
    numbers.push(40);

    // ✅ Access elements by index
    println!("First number: {}", numbers[0]);

    // ✅ Safe indexing with .get()
    match numbers.get(2) {
        Some(value) => println!("Third number: {}", value),
        None => println!("No third number."),
    }

    // ✅ Iterate using for loop
    println!("All numbers using for loop:");
    for num in &numbers {
        println!("{}", num);
    }

    // ✅ Iterate with index
    println!("Numbers with index:");
    for (i, num) in numbers.iter().enumerate() {
        println!("Index {}: {}", i, num);
    }

    // ✅ Remove the last element
    if let Some(last) = numbers.pop() {
        println!("Removed last number: {}", last);
    }

    println!("Final vector: {:?}", numbers);
}
