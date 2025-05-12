// Define a struct to represent a Contact
#[derive(Debug)]
struct Contact {
    name: String,
    phone_number: String,
    email: String,
}

// Implement methods for the Contact struct
impl Contact {
    fn new(name: &str, phone_number: &str, email: &str) -> Self {
        Contact {
            name: name.to_string(),
            phone_number: phone_number.to_string(),
            email: email.to_string(),
        }
    }

    fn display(&self) {
        println!("Name: {}, Phone: {}, Email: {}", self.name, self.phone_number, self.email);
    }
}

fn main() {
    // Create a vector to hold multiple contacts
    let mut contact_list: Vec<Contact> = Vec::new();

    // Add contacts to the list
    contact_list.push(Contact::new("Alice", "123-456-7890", "alice@example.com"));
    contact_list.push(Contact::new("Bob", "987-654-3210", "bob@example.com"));
    contact_list.push(Contact::new("Charlie", "555-555-5555", "charlie@example.com"));

    // Iterate over the contact list and display each contact
    println!("Contact List:");
    for contact in &contact_list {
        contact.display();
    }

    // Optionally: Remove a contact (e.g., removing the last contact)
    if let Some(removed_contact) = contact_list.pop() {
        println!("\nRemoved Contact: {:?}", removed_contact);
    }

    // Display the final contact list
    println!("\nFinal Contact List:");
    for contact in &contact_list {
        contact.display();
    }
}
