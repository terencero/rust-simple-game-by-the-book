use std::io;

pub mod guessing_game;
pub mod temperature;
pub mod lyrics;
pub mod fibonacci;
pub mod speaker_randomizer;
pub mod common_collections;
pub mod pig_latin;

fn main() {
    println!("What would you like to do?");
    println!("[1] Play the Guessing Game");
    println!("[2] Convert the current temperature");
    println!("[3] Sing me a Christmas song");
    println!("[4] Print the fibonacci sequence");
    println!("[5] Randomize speakers");
    println!("[6] Find the median of a set of numbers. Enter a white space separated list of numbers.");
    println!("[7] Find the mode of a set of number. Enter a white space separated list of numbers.");
    println!("[8] Run the Pig Latin translator.");

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
    } else if answer == "5" {
        speaker_randomizer::randomize_names();
    } else if answer == "6" {
        common_collections::find_median();
    } else if answer == "7" {
        common_collections::find_mode();
    } else if answer == "8" {
        pig_latin::translate_to_pig_latin();
    } else {
        println!("Aww you didn't choose an option... Till next time!");
    }
}

