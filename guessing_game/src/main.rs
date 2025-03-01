use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guessing Game in Rust - Ben Archard");
    println!("Date created: February 28th 2025");
    println!("-----------------------------------");
    println!("I'm thinking of a number.. ");
    println!("Guess my number: ");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut guesses = 1;

    loop {
        println!("Current try: {guesses}");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Guess my number: ");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess is too low!"),
            Ordering::Greater => println!("Your guess is too high!"),
            Ordering::Equal => {
                println!("Congrats!, you have guessed my number!");
                println!("It has taken you {guesses} tries!");
                break;
            }
        }

        guesses += 1;
    }
}
