// src/dictionaries/files.rs

use std::{
	fs, 
	io::{
		self,
		Read,
	},
	path
};

use crate::dictionaries::base;
use crate::modules;

const LINE_SEPARATORS: [char; 2] = ['\n', '\r'];

/// Read a file `name` and return a list of lines `String`.
pub fn read(file_path: &path::Path) -> base::WordList {
	match fs::File::open(file_path) {
		Ok(file) => {
			let mut buffer_reader: io::BufReader<fs::File> = io::BufReader::new(file);
			let mut content: String = String::new();

			buffer_reader.read_to_string(&mut content).expect("(X) - dictionaries.files.read - Reading to string error.");

			let mut lines: base::WordList = Vec::new();
			let mut current: String = String::new();
			for character in content.chars() {
				if LINE_SEPARATORS.contains(&character){
					if !current.is_empty() {
						lines.push(current);
					}
					current = String::new();
				} else {
					current.push(character);
				}
			}
			if !current.is_empty() {
				lines.push(current);
			}

			lines
		},
		Err(error) => {
			eprint!(
				"(!) - dictionaries.files.read - Error opening given path ({}): {}. Falling back to an empty vec.", 
				file_path.display(), error
			);
			vec![]
		}
	}
	
}

/// Shorthand to get dictionaries path, join the `name` to it, and read it.
pub fn read_from_data(name: &str) -> base::WordList {
	read(&modules::defaults::paths().get_dictionaries().join(name))
}


