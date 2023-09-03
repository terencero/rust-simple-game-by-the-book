use std::{io, vec};
use rand::Rng;

pub fn randomize_names() {
    let mut names = vec!["Val", "Ben", "eric", "tRo", "Brandon", "Jonathan", "jve", "Wills", "Zach", "Bob"];
    let mut answer = String::new();

    println!("Generate all speaker names [1] or get one [2]?");

    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read your answer...");

    let answer = answer.trim();

    if answer == "1" {
        print_names(&mut names);
    } else if answer == "2" {
        pick_one_speaker();
    } else {
        println!("You didn't pick a valid option...");
    }
}

fn print_names(names: &mut Vec<&str>) {
    let name = pull_name_from_hat(names);

    println!("{name}");

    if names.len() > 0 {
        print_names(names);
    }
}

fn pick_one_speaker() {
    let mut list_of_names = String::new();
    
    io::stdin()
    .read_line(&mut list_of_names)
    .expect("Failed to read the names...");
    
    let list_of_names = list_of_names.trim();
    let mut speaker_candidates = read_list_of_names(list_of_names);
    
    let selected_speaker = pull_name_from_hat(&mut speaker_candidates);

    println!("{selected_speaker}");
}

fn read_list_of_names(names: &str) -> Vec<&str> {
    let mut speaker_candidates: Vec<&str> = Vec::new();
    let bytes = names.trim().as_bytes();
    let mut starting_string_index = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            starting_string_index = if starting_string_index > 0 {
                starting_string_index + 1
            } else {
                starting_string_index
            };

            let n = &names[starting_string_index..i];

            speaker_candidates.push(n);
            starting_string_index = i;
        }

        if i == bytes.len() - 1 {
            let n = &names[starting_string_index + 1..];

            speaker_candidates.push(n);
        }
    }

    println!("speaker candidates entered: {:?}", speaker_candidates);

    speaker_candidates
}

fn pull_name_from_hat(names: &mut Vec<&str>) -> String {
    let random_index = get_random_index(names.len());
    let name = names.swap_remove(random_index);

    String::from(name)
}

fn get_random_index(range: usize) -> usize {
    let random_num = rand::thread_rng().gen_range(0..=range - 1);
    random_num
}
