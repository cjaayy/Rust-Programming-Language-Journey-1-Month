use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("🎯 Welcome to the Number Guessing Game!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess (1-100):");

        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Parse input to number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("📈 Too small!"),
            Ordering::Greater => println!("📉 Too big!"),
            Ordering::Equal => {
                println!("🎉 You guessed it! Congratulations!");
                break;
            }
        }
    }
}
