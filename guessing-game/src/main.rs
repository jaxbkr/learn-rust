use colored::*;
use rand::prelude::*;
use std::io;

fn main() {
    println!("Guess the number!");

    let mut rng = rand::rng();
    let mut guesses = 0;
    let secret_number: u32 = rng.random_range(1..=10);

    loop {
        guesses += 1;
        println!("Please input your guess between 1 and 10.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("{}", "Too small!".red()),
            std::cmp::Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
            std::cmp::Ordering::Greater => println!("{}", "Too big!".red()),
        }
    }
    println!(
        "The solution was: {}\nYou took {} guesses.",
        secret_number, guesses
    );
}
