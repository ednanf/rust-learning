// input/output library to obtain user input
use std::io;

// comparison library
use std::cmp::Ordering;

// random library from rand crate
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        // creates a new variable to store user input - "mut" makes it mutable
        // ::new() is an **associated function**, which is a function implemented
        // on a type
        let mut guess = String::new();

        io::stdin()
            // creates a new variable to store user input - "mut" makes it mutable
            .read_line(&mut guess)
            .expect("Failed to read line!");

        // trims and converts the input into a u32 so it can be compared
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
