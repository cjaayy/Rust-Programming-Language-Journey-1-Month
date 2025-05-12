// Define an enum for different user roles
enum UserRole {
    Admin,
    Moderator,
    Guest,
    Member(String), // Variant with data
    Banned { reason: String }, // Struct-like variant
}

// Function to greet the user based on role
fn greet_user(role: UserRole) {
    match role {
        UserRole::Admin => println!("Welcome, Admin! You have full access."),
        UserRole::Moderator => println!("Hello, Moderator. You can manage content."),
        UserRole::Guest => println!("Welcome, Guest! Please sign up to unlock more features."),
        UserRole::Member(name) => println!("Hello, {}! Thanks for being a member.", name),
        UserRole::Banned { reason } => println!("Access denied. You are banned because: {}", reason),
    }
}

fn main() {
    let user1 = UserRole::Admin;
    let user2 = UserRole::Member(String::from("Alice"));
    let user3 = UserRole::Banned { reason: String::from("Spamming") };

    greet_user(user1);
    greet_user(user2);
    greet_user(user3);
}
