fn main() {
    // loop: infinite until we break
    let mut counter = 0;
    loop {
        counter += 1;
        println!("Loop counter: {}", counter);

        if counter == 3 {
            println!("Breaking the loop!");
            break;
        }
    }

    println!("-------------------");

    // while loop: repeat while a condition is true
    let mut number = 5;
    while number > 0 {
        println!("While loop number: {}", number);
        number -= 1;
    }

    println!("Liftoff! 🚀");

    println!("-------------------");

    // for loop: iterate over a range
    for i in 1..=5 { // 1 to 5 inclusive
        println!("For loop iteration: {}", i);
    }

    // for loop: iterate over an array
    let fruits = ["Apple", "Banana", "Cherry"];
    for fruit in fruits.iter() {
        println!("I love {}", fruit);
    }
}
