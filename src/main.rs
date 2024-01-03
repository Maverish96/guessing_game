use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let num = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");
        let mut guess: String = "".to_string();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {guess}");
        let guess: u32 = match guess.trim().parse() {
            Ok(guess) => guess,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };

        match guess.cmp(&num) {
            Ordering::Less => println!("Oops! SMALL!"),
            Ordering::Greater => println!("Oops! BIG!"),
            Ordering::Equal => {
                println!("Voila!");
                break;
            }
        }
    }
}
