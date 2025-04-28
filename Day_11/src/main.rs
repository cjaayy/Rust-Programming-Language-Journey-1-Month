fn main() {
    let age = 20;
    let has_id = true;

    // Nested conditionals
    if age >= 18 {
        if has_id {
            println!("✅ You are allowed to enter.");
        } else {
            println!("❌ You need to show your ID.");
        }
    } else {
        println!("❌ You must be at least 18 years old.");
    }

    // Logical operators example
    let is_student = true;
    let has_discount_card = false;

    if is_student || has_discount_card {
        println!("🎫 You are eligible for a discount!");
    } else {
        println!("💸 No discount available.");
    }

    if !(age < 18) && has_id {
        println!("🔒 Access granted.");
    } else {
        println!("🔒 Access denied.");
    }
}
