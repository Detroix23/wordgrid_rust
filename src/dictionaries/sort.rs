// src/dictionaries/sort.rs

use crate::dictionaries;


pub fn is_sorted(words: &dictionaries::base::WordList) -> bool {
	let mut index: usize = 0;
	let mut sorted: bool = true;

	while index < words.len() - 1 && sorted {
		sorted = dictionaries::words::is_greater(&words[index + 1], &words[index]);
		index += 1;
	}

	// println!("Unsorted 1st on: {} {}", &words[index], &words[index - 1]);

	sorted
}

/// Insertion sort by reference `words`.
pub fn sort(words: &mut dictionaries::base::WordList) -> () {
	eprintln!("Sort - Started ... ");
	let mut i: usize = 1;

	while i < words.len() {
		let value: String = words[i].clone();
		let mut before: String = words[i - 1].clone();
		let mut j: usize = 0;

		eprint!("\r{}: {}, {}; \r", i, value, before);

		while !dictionaries::words::is_greater(&value, &before) && i - j > 0 {
			words[i - j] = String::from(&before);
			j += 1;
			before = words[i - 1 - j].clone();

			// eprintln!("\t{}: {}.", j, before);

		}

		words[i - j] = value;

		i += 1;
	}

	eprintln!("Finished.")
}
