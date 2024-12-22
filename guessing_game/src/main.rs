use rand::Rng;
use std::cmp::Ordering;
use std::{io, u32};

fn main() {
    let mut secret_number;
    let mut level = 1;

    fn get_guess() -> Option<u32> {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read user input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                return None;
            }
        };

        return Some(guess);
    }

    fn match_guess(secret_number: u32, mut level: u32) -> u32 {
        loop {
            let guess = get_guess();

            if !guess.is_some() {
                continue;
            }

            match guess.unwrap().cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    level += 1;
                    return level;
                }
            }
        }
    }

    loop {
        secret_number = rand::thread_rng().gen_range(1..=100);
        level = match_guess(secret_number, level);
        println!("Level {level}, try to guess number:");
    }
}
