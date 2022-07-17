use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("What would you like to do?");
    println!("Play the Guessing Game [1]");
    println!("Convert the current temperature [2]");

    let mut answer = String::new();

    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line");
    
    let answer = answer.trim();

    if answer == "1" {
        play_the_guessing_game();
    } else if answer == "2" {
        convert_temperature();
    } else {
        println!("Aww you didn't choose an option... Till next time!");
    }
}

fn play_the_guessing_game() {
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

fn convert_temperature() {
    let mut answer = String::new();

    println!("What is the temperature unit of measure? [f / c]?");
    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line");

    let temperature_scale = if answer.trim() == "f" { "f" } else if answer.trim() == "c" { "c" } else { "" };

    let mut next_answer = String::new();

    println!("What is the current temperature measurement? i.e. 71");
    io::stdin()
        .read_line(&mut next_answer)
        .expect("Failed to read line");

    let og_temperature_unit: f64 = next_answer.trim().parse().expect("Please enter a number");

    if temperature_scale == "c" {
        let temperature = (og_temperature_unit / 5.0) * 9.0 + 32.0;
        print_temperature_conversion(og_temperature_unit, temperature, &temperature_scale);
    } else if temperature_scale == "f" {
        let temperature = (og_temperature_unit - 32.0) * 5.0 / 9.0;
        print_temperature_conversion(og_temperature_unit, temperature, &temperature_scale);
    } else {
        println!("Not a valid temperature unit of measure");

    };
}

fn print_temperature_conversion(original_temperature_unit: f64, converted_temperature_unit: f64, original_scale: &str) {
    let scale = if original_scale == "c" { "f" } else { "c" };
    println!("{original_temperature_unit} degrees {original_scale} is {converted_temperature_unit} degrees in {scale}");
}