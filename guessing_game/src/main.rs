use rand::Rng;
use std::cmp::Ordering;
use std::{io, u32};

fn main() {
    let mut secret_number;
    let mut level = 1;

    loop {
        secret_number = rand::thread_rng().gen_range(1..=100);

        println!("Level {level}, try to guess number:");

        loop {
            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read user input");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid number!");
                    continue;
                }
            };

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    level += 1;
                    break;
                }
            }
        }
    }
}
