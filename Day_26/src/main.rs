fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn get_username(user_id: u32) -> Option<&'static str> {
    match user_id {
        1 => Some("Alice"),
        2 => Some("Bob"),
        _ => None,
    }
}

fn main() {
    // Part 1: Match usage
    let result = divide(10.0, 2.0);
    match result {
        Some(value) => println!("10 / 2 = {}", value),
        None => println!("Cannot divide by zero."),
    }

    let bad_result = divide(5.0, 0.0);
    match bad_result {
        Some(value) => println!("5 / 0 = {}", value),
        None => println!("Cannot divide by zero."),
    }

    // Part 2: if let
    let name = get_username(1);
    if let Some(username) = name {
        println!("User 1 is: {}", username);
    }

    // Part 3: unwrap_or
    let unknown_name = get_username(3).unwrap_or("Guest");
    println!("User 3 is: {}", unknown_name);
}
