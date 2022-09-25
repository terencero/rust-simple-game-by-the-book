// use std::io;
// use std::cmp::Ordering;
use std::{cmp::Ordering, io};
use rand::Rng;

pub fn play_the_guessing_game() {
    println!("Guess the number! 1 to 100");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut attempts: u32 = 0;
    
    println!("Ready to start the Guessing Game?");
    
    loop {
        println!("Please input your guess.");
        // println!("secret {secret_number}");
    
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                attempts += 1;
                emit_too_small(&mut attempts);
            }
            Ordering::Equal => {
                emit_you_win();
                break;
            }
            Ordering::Greater => {
                attempts += 1;
                emit_too_big(&mut attempts);
            }
        }
    }
}

fn emit_too_small(attempts: &mut u32) {
    println!("Too small! attempts at guess {attempts}");
}

fn emit_too_big(attempts: &mut u32) {
    println!("Too big! attempts at guess {attempts}");
}

fn emit_you_win() {
    println!("You win!");
}