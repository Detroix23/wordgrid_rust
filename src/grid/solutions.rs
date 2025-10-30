// src/grid/solutions.rs

use crate::modules::maths;
use crate::dictionaries;

/// Define 1 solution.
#[derive(Clone, Debug)]
pub struct Solution {
    word: String,
    position: maths::Size,
    direction: maths::Size,
}
impl Solution {
	/// Return a nicely formatted string as `word (position.x, position.y; direction.x, direction.y)`.
	pub fn display(self: &Self) -> String {
		format!("{} ({}, {}; {}, {})", self.word, self.position.x, self.position.y, self.direction.x, self.direction.y)
	}
}

/// Treat a given word. If `word`` exists, push into `found` a new `Solution`.
pub fn check_word(
	word: String,
	dictionary: &dictionaries::base::WordList,
	found: &mut Vec<Solution>, 
	position: maths::Size, 
	direction: maths::Size
) -> dictionaries::search::Result {
	let result: dictionaries::search::Result = dictionaries::search::dichotomy(word.clone(), &dictionary);
	if let dictionaries::search::Result::Existant = result {
		found.push(Solution { 
			word,
			position,
			direction,
		})
	}

	result
}

