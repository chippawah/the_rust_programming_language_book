use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess a number:");
    println!("Please input your guess");
    // Creating a number between 1 and 100 inclusive
    let secret_number = rand::thread_rng().gen_range(0..=100);
    println!("The secret number is {secret_number}");
    loop {
        let mut guess = String::new();
        // Reading the input from the console and panicking if if can't
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line!");

        // Shadowing the previous guess and converting it into a number for comparison
        // Also using the Err variant to just continue if there's a bad guess
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");
        // Using match expression to match against the comparison between guess and secret number
        match guess.cmp(&secret_number) {
            Ordering::Greater => {
                println!("Too big")
            }
            Ordering::Less => {
                println!("Too smol")
            }
            Ordering::Equal => {
                println!("Juuuuust right");
                break; // Break here will break out of the loop and therefore end program execution.
            }
        }
    }
}
