use std::collections::HashMap;
use std::io;

// receive an input of integers
// store the integers in a vector
fn get_list_of_numbers() -> String {
	let mut answer = String::new();

	io::stdin()
		.read_line(&mut answer)
		.expect("Failed to read line");

	answer
}

fn convert_to_integer_vector(string_numbers: String) -> Vec<i32> {
	let mut integers: Vec<i32> = Vec::new();
	let string_numbers_bytes = string_numbers.trim().as_bytes();
	let mut starting_string_index = 0;

	for (i, &item) in string_numbers_bytes.iter().enumerate() {
		if item == b' ' {
			starting_string_index = if starting_string_index > 0 {
				starting_string_index + 1
			} else {
				starting_string_index
			};

			let integer: i32 = string_numbers[starting_string_index..i].trim().parse().expect("Non-integer");

			integers.push(integer);
			starting_string_index = i;
		}

		if i == string_numbers_bytes.len() - 1 {
			let integer_slice = string_numbers[starting_string_index + 1..].trim();
			let integer: i32 = integer_slice.parse().expect("Non-integer");

			integers.push(integer);
		}
	}

	println!("integers {:?}", integers);

	integers
}

pub fn find_median() {
	let mut integer_list = convert_to_integer_vector(get_list_of_numbers());
	integer_list.sort();
	let integer_list_len = integer_list.len();
	let is_even = integer_list_len % 2 == 0;

	if is_even {
		let first_half = integer_list_len / 2;
		let second_half = first_half + 1;
		let median = (integer_list[first_half] + integer_list[second_half]) as f32 / 2.0;

		println!("The median is {median}");
	} else {
		let median_position = (integer_list_len as f32 / 2.0 - 1.0).ceil() as usize;
		let median = integer_list[median_position];

		println!("The median is {median}");
	}
}

pub fn find_mode() {
	let integer_list = convert_to_integer_vector(get_list_of_numbers());
	let mut hash = HashMap::new();
	let mut mode_ints = Vec::new();

	for int in integer_list.into_iter() {
		let count = hash.entry(int).or_insert(0 as i32);
		*count += 1;
	}

	println!("{:?}", hash);

	let mut current_highest_mode = 2;

	for (key, val) in hash.iter() {
		// because val is a borrow it can only be compared against another borrow?
		if val > &current_highest_mode {
			mode_ints.drain(0..);
			mode_ints.push(key);
			// need to dereference val?
			current_highest_mode = *val;
		} else if val == &current_highest_mode {
			mode_ints.push(key);
		}
	}

	if mode_ints.len() < 1 {
		println!("No mode");
	} else {
		println!("The mode(s) {:?}", mode_ints);
	}
}
