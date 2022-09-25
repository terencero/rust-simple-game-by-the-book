use std::io;
// use crate::fibonacci::perfect_square;

pub mod perfect_square;

pub fn calculate_fibonacci() {
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
        let is_perfect_square = if perfect_square::test_is_perfect_square(fibonacci_potential) {
            true
        } else {
            let fibonacci_potential = 5 * current_fibonacci_test_num * current_fibonacci_test_num - 4;
            perfect_square::test_is_perfect_square(fibonacci_potential)
        };
        
        if is_perfect_square {
            println!("{current_fibonacci_test_num}");
        }

        current_fibonacci_test_num += 1;
        counter += 1;
    };
}
