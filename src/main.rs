use std::io;

// use self::guessing_gam;
// use temperature;
// use lyrics;
// use fibonacci;

pub mod guessing_game;
pub mod temperature;
pub mod lyrics;
pub mod fibonacci;

fn main() {
    println!("What would you like to do?");
    println!("Play the Guessing Game [1]");
    println!("Convert the current temperature [2]");
    println!("Sing me a Christmas song [3]");
    println!("Print the fibonacci sequence [4]");
    let mut answer = String::new();

    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line");
    
    let answer = answer.trim();

    if answer == "1" {
        guessing_game::play_the_guessing_game();
    } else if answer == "2" {
        temperature::convert_temperature();
    } else if answer == "3" {
        lyrics::print_the_lyrics_to_the_twelve_days_of_christmas();
    } else if answer == "4" {
        fibonacci::calculate_fibonacci();
    } else {
        println!("Aww you didn't choose an option... Till next time!");
    }
}

