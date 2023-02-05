use std::io;

pub fn translate_to_pig_latin() {
	// TODO: take in input: text to translate
	let mut answer = String::new();

	println!("Enter the text you would like to translate to pig latin.");
	io::stdin()
		.read_line(&mut answer)
		.expect("failed to read line");

	let untranslated_text = answer.trim();
	let mut translated_text = String::new();
	let mut word: Vec<char> = Vec::new();

	for (i, char) in untranslated_text.chars().enumerate() {
		word.push(char);

		if char == ' ' || i == untranslated_text.chars().count() - 1 {
			match word[0] {
				'a' | 'e' | 'i' | 'o' | 'u' => {
					translated_text.push_str(&arrange_vowel_word(&mut word));
					word.clear();
				},
				_ => {
					translated_text.push_str(&arrange_consonant_word(&mut word));
					word.clear();
				}
			};
			continue;
		}
	};
	println!("Translated text: {translated_text}");
}

fn arrange_vowel_word(word: &mut Vec<char>) -> String {
	let mut pig_latinized = String::new();

	for letter in word {
		pig_latinized.push(*letter);
	};

	pig_latinized = pig_latinized.trim().to_string();
	pig_latinized.push_str("-hay ");
	pig_latinized
}

fn arrange_consonant_word(word: &mut Vec<char>) -> String {
	let mut pig_latinized = String::new();
	let pig_suffix = String::from("-") + &String::from(word[0]) + "ay ";

	for (i, letter) in word.iter().enumerate() {
		if i == 0 {
			continue;
		}
		pig_latinized.push(*letter);
	};

	pig_latinized = pig_latinized.trim().to_string();
	pig_latinized.push_str(&pig_suffix);
	pig_latinized
}