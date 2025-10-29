// src/dictionaries/words.rs

use std::cmp;

/// ASCII index of lowercase a. 
const INDEX_A: u8 = 97;
/// ASCII index of lowercase z.
const INDEX_Z: u8 = 122;


/// When comparing 2 string, they can be `Equal`, or one can be `Greater` than the other.
#[derive(Debug)]
pub enum Comparison {
	/// Strings are the same.
	Equal,
	/// This string if after the other one in dictionary.
	Inequal {
		small: String,
		great: String,
	},
}

/// Return an `u8` between 0 and 27.
/// 'a' is 1, 'z' is 26, the rest is 0.
pub fn index(character: char) -> u8 {
	// Get and check ASCII index of the char.
	if let INDEX_A..=INDEX_Z = character as u8 {
		character as u8 - INDEX_A + 1
	} else {
		0u8
	}
}

/// Get the character in `word`, `at` the index.
/// **You must ensure that the string is ASCII only.**
pub fn char_at(word: &str, at: usize) -> Result<char, String> {
	let bytes: &[u8] = word.as_bytes();
	if at >= bytes.len() {
		return Result::Err(format!(
			"(X) - dictionaries.words.char_at(word: {}, at: {}) - Index ({}) out of range ({})",
			word, at, at, word.len())
		);
	}
	
	Result::Ok(bytes[at] as char)
}

/// Compare 2 `&'static str`.
/// **You must ensure that the strings are ASCII only.**
pub fn compare(a: &str, b: &str) -> Comparison {
	let mut cursor: usize = 0;
	let path: usize = cmp::min(a.len(), b.len());
	let mut state: Comparison = Comparison::Equal;

	while cursor < path && let Comparison::Equal = state {
		let index_a: u8 = index(char_at(&a, cursor).unwrap());
		let index_b: u8 = index(char_at(&b, cursor).unwrap());

		if index_a > index_b {
			state = Comparison::Inequal{
				small: String::from(b),
				great: String::from(a),
			};
		} else if index_a < index_b {
			state = Comparison::Inequal { 
				small: String::from(a), 
				great: String::from(b), 
			};
		}

		cursor += 1;
	}

	match state {
		Comparison::Equal => 
			// They share same starting letters but one is longer.
			if a.len() > b.len() {
				Comparison::Inequal { 
					small: String::from(b), 
					great: String::from(a), 
				}
			} else if a.len() < b.len() {
				Comparison::Inequal { 
					small: String::from(a), 
					great: String::from(b),
				}
			// If it's just equal.
			} else {
				Comparison::Equal
			},
		Comparison::Inequal { small, great } => Comparison::Inequal { small, great },
	}
}

pub fn is_greater(word: &str, than: &str) -> bool {
	match compare(word, than) {
		Comparison::Inequal { great, .. } => {
			great == word 
		},
		_ => true,
	}
}
