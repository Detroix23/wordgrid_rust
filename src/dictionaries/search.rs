// src/dictionaries/search.rs

use crate::dictionaries::{base, words};


/// Define a search result.
/// `Inexistant`, `Existant`, or `Prefix` (another, at least 1, word start with the given word).
#[derive(Clone, Copy, Debug)]
pub enum Result {
	Inexistant,
	Existant,
	Prefix,
}

/// Iterative binary search.
/// Take a `word` (`String`) and a `dictionary` (List of word). 
pub fn dichotomy(word: String, dictionary: &base::WordList) -> Result {
	// eprintln!("=====================================");
	//eprintln!("dictionaries.search.dichotomy - New word: {}, dict len: {}", word, dictionary.len());

	let mut start: usize = 0;
	let mut end: usize = dictionary.len() - 1;
	let mut comparison: words::Comparison;
	let mut found: bool = false;

	while !found && start <= end {
		let cursor: usize = (start + end) / 2;
		let value: String = dictionary[cursor].clone();

		comparison = words::compare(&word, &value);

		if let words::Comparison::Equal = comparison {
			found = true;
		}

		// eprintln!("	- ({}, {}; {}) value: {}, comparison: {:?}", start, end, cursor, value, comparison);
		
		if words::is_greater(&word, &value) {
			start = cursor + 1;
		} else {
			end = cursor - 1;
		}
	}

	if found {
		Result::Existant
	} else {
		Result::Inexistant
	}
}
