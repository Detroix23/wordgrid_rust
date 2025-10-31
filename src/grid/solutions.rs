// src/grid/solutions.rs

use crate::modules::maths;
use crate::dictionaries;

/// Define 1 solution.
#[derive(Clone, Debug)]
pub struct Solution {
    pub word: String,
    pub position: maths::Size,
    pub direction: maths::Size,
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

/// Return a nicely formated String from a `Vec<Solution>`.  
pub fn display_solutions(found: &Vec<Solution>, columns: usize) -> String {
	let mut string: String = String::new();
	
	let longest: usize = found
		.iter()
		.fold(0, |length, solution| 
			if solution.display().len() > length {
				solution.display().len()
			} else {
				length
			}
		);

	for (index, solution) in found.iter().enumerate() {
		if index % columns == 0 {
			string += "\n";
			string += &format!(
				"{}{} │ ", 
				" ".repeat((found.len() / columns).to_string().len() - (index / columns).to_string().len()), 
				index / columns
			);
		}
		string += &format!("{}{}│ ", solution.display(), " ".repeat(longest - solution.display().len() + 1));
	}

	string
}


/// Return a new vector, insertion sorted increasing of `list`: `Vec<Solution>`.
pub fn sort(list: &Vec<Solution>) -> Vec<Solution> {
	let mut new: Vec<Solution> = list.clone();
	let mut i: usize = 1;

	while i < new.len() {
		let value: Solution = new[i].clone();
		let mut before: Solution = new[i - 1].clone();
		let mut j: usize = 0;

		while value.word.len() < before.word.len() && i - j > 1 {
			new[i - j] = before;
			j += 1;
			before = new[i - 1 - j].clone();

			// eprintln!("\t{}: {}.", j, before);

		}

		new[i - j] = value;

		i += 1;
	}

	new
}
