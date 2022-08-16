use std::io;
use std::cmp::Ordering;
use rand::Rng;

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
        play_the_guessing_game();
    } else if answer == "2" {
        convert_temperature();
    } else if answer == "3" {
        print_the_lyrics_to_the_twelve_days_of_christmas();
    } else if answer == "4" {
        calculate_fibonacci()
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

fn print_the_lyrics_to_the_twelve_days_of_christmas() {
    struct Lyrics {
        days: [String; 12],
        presents: [String; 11],
    }

    impl Lyrics {
        fn get_days(&self, index: usize) -> &String {
            &self.days[index]
        }

        fn get_presents(&self, index: usize) -> &String {
            &self.presents[index]
        }
    }

    let lyrics = Lyrics {
        days: [
            String::from("first"),
            String::from("second"),
            String::from("third"),
            String::from("fourth"),
            String::from("fifth"),
            String::from("sixth"),
            String::from("seventh"),
            String::from("eighth"),
            String::from("ninth"),
            String::from("tenth"),
            String::from("eleventh"),
            String::from("twelfth"),
        ],
        presents: [
            String::from("Two turtle doves"),
            String::from("Three french hens"),
            String::from("Four calling birds"),
            String::from("Five golden rings"),
            String::from("Six geese a-laying"),
            String::from("Seven swans a-swimming"),
            String::from("Eight maids a-milking"),
            String::from("Nine ladies dancing"),
            String::from("Ten lords a-leaping"),
            String::from("Eleven pipers piping"),
            String::from("Twelve drummers drumming"),
        ],
    };
    for day in 0..12 {
        let chorus_line = lyrics.get_days(day);
        println!("On the {} day of Christmas, my true love sent to me", chorus_line);

        if day > 0 {
            for element in 0..day {
                let next_line = lyrics.get_presents(element);
                let and = if element + 1 == day { ", and" } else { "" };
                println!("{next_line}{and}");
            };
        }

        println!("A partridge in a pear tree");
        println!("");
    };
}

fn calculate_fibonacci() {
    let mut lower_range = String::new();
    let mut upper_range = String::new();

    println!("Choose starting and ending number ranges up to 1,000");
    println!("Enter the starting number");

    io::stdin()
        .read_line(&mut lower_range)
        .expect("Failed to read line");
        
    let lower_range: u32 = lower_range.trim().parse().expect("Not a number");
    println!("Enter the ending number");

    io::stdin()
        .read_line(&mut upper_range)
        .expect("Failed to read line");

    let upper_range: u32 = upper_range.trim().parse().expect("Not a number");
    let range = (lower_range..upper_range).len();
    let mut current_fibonacci_test_num = lower_range;
    let mut counter = 0;

    loop {
        if counter > range {
            println!("Out of range, finished.");
            break;
        }
        let fibonacci_potential = 5 * current_fibonacci_test_num * current_fibonacci_test_num + 4;
        let is_perfect_square = if test_is_perfect_square(fibonacci_potential) {
            true
        } else {
            let fibonacci_potential = 5 * current_fibonacci_test_num * current_fibonacci_test_num - 4;
            test_is_perfect_square(fibonacci_potential)
        };
        
        if is_perfect_square {
            println!("{current_fibonacci_test_num}");
        }

        current_fibonacci_test_num += 1;
        counter += 1;
    };
}

fn test_is_perfect_square(num: u32) -> bool {
    // iterative way to find square root instead of using sqrt method
    let start: f64 = num.into();
    let mut test_num: f64 = start - 1.0;
    let mut previous_test_num: f64 = 0.0;
    
    let calculated_square_root = loop {
        test_num = (test_num + start / test_num )/ 2.0;
        if test_num == previous_test_num {
            break test_num;
        }
        previous_test_num = test_num;
    };

    if start % calculated_square_root > 0.0 {
        return false;
    }

    true
}
