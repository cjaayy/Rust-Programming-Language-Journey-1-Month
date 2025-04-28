fn main() {
    let number = 3;

    match number {
        1 => println!("You chose One!"),
        2 => println!("You chose Two!"),
        3 => println!("You chose Three!"),
        4 | 5 => println!("You chose Four or Five!"), // multiple patterns
        6..=10 => println!("You chose a number between Six and Ten!"), // range pattern
        _ => println!("You chose something else."), // wildcard pattern for everything else
    }

    // match can also return a value
    let grade = 'B';

    let message = match grade {
        'A' => "Excellent!",
        'B' => "Good job!",
        'C' => "You passed.",
        'D' => "Try harder next time.",
        'F' => "Failed.",
        _ => "Invalid grade.",
    };

    println!("Grade message: {}", message);
}
