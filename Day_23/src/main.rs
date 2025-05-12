// Define a struct for a Rectangle
struct Rectangle {
    width: u32,
    height: u32,
}

// Implement methods for Rectangle
impl Rectangle {
    // Associated method (like a constructor)
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    // Instance method to calculate area
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Instance method to check if the rectangle is a square
    fn is_square(&self) -> bool {
        self.width == self.height
    }

    // Method to display dimensions
    fn display(&self) {
        println!("Width: {}, Height: {}", self.width, self.height);
    }
}

fn main() {
    // Create a rectangle using the `new` method
    let rect1 = Rectangle::new(10, 20);
    let rect2 = Rectangle::new(15, 15);

    rect1.display();
    println!("Area: {}", rect1.area());
    println!("Is square? {}\n", rect1.is_square());

    rect2.display();
    println!("Area: {}", rect2.area());
    println!("Is square? {}", rect2.is_square());
}
