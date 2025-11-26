// src/dictionaies/mod.rs

pub mod base;
pub mod words;
pub mod files;
pub mod search;
pub mod sort;

use crate::dictionaries;
use crate::modules;

/// Create a dictionary from a `name`, and sort and write if necessary.
pub fn prepare_dictionary(name: String) -> dictionaries::base::WordList {
	eprintln!("\n### Dictionary preparation.");
	
	let mut dictionary: dictionaries::base::WordList = dictionaries::files::read_from_data(&name);

	if !dictionaries::sort::is_sorted(&dictionary) {
		eprintln!("! Dict not sorted. This might take some time...");
		
		dictionaries::sort::sort(&mut dictionary);
		
		eprintln!("! Writing new dict under `{}`.", name.clone() + ".sorted");

		if !dictionaries::sort::is_sorted(&dictionary) {
			
			eprintln!(
				"! Dict (len={}) passed through `sort::sort`, though not sorted. Sorting again.",
				dictionary.len(),
			);

			dictionaries::sort::sort(&mut dictionary);
		}

		dictionaries::files::write(
			&modules::defaults::paths().get_dictionaries().join(name.clone() + ".sorted"),
			&dictionary,
			"\n",
		);

		eprintln!("! Dict written.");
	}

	eprintln!("Dict {} is good to go! ", name);

	dictionary
}
