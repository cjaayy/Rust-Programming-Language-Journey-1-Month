fn main() {
    let number = 15;

    if number > 20 {
        println!("The number is greater than 20.");
    } else if number == 20 {
        println!("The number is exactly 20.");
    } else {
        println!("The number is less than 20.");
    }

    // You can also use `if` as an expression!
    let is_even = if number % 2 == 0 {
        true
    } else {
        false
    };
    println!("Is the number even? {}", is_even);
}
