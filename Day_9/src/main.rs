fn main() {
    // Statement: declares a variable (does not return a value)
    let x = 5;

    // Statement: a block of code (also doesn't return a value directly)
    let y = {
        let a = 10;
        let b = 2;
        a + b // Expression: returns a value (this becomes the value of y)
    };

    println!("x = {}", x);
    println!("y = {}", y);

    // Function call as an expression
    let z = square(4); // square(4) is an expression
    println!("z = {}", z);
}

// Expression: function that returns a value
fn square(n: i32) -> i32 {
    n * n
}
