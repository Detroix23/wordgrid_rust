// src/dictionaries/files.rs

use std::{
	fs::{
		self,
		File
	}, 
	io::{
		self,
		Read, 
		Write,
	},
	path
};

use crate::dictionaries::base;
use crate::modules;

/// Define line breaks, new words.
const LINE_SEPARATORS: [char; 2] = ['\n', '\r'];
/// Define character that will prematurely end the reading of the current line. 
const LINE_SHORT: [char; 3] = [' ', ';', ','];

/// Read a file `name` and return a list of lines `String`.
pub fn read(file_path: &path::Path) -> base::WordList {
	match fs::File::open(file_path) {
		Ok(file) => {
			let mut buffer_reader: io::BufReader<fs::File> = io::BufReader::new(file);
			let mut content: String = String::new();
			let mut line_shorted: bool = false;

			buffer_reader.read_to_string(&mut content).expect("(X) - dictionaries.files.read - Reading to string error.");

			let mut lines: base::WordList = Vec::new();
			let mut current: String = String::new();
			for character in content.chars() {
				if LINE_SEPARATORS.contains(&character){
					if !current.is_empty() {
						lines.push(current);
					}
					line_shorted = false;
					current = String::new();
				} else if LINE_SHORT.contains(&character) {
					line_shorted = true;
				} else if !line_shorted {
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

/// Write a list of word. Each word is separated by `separator`.
pub fn write(file_path: &path::Path, words: &base::WordList, separator: &str) {
	let file = File::create(file_path);
	match file {
		Ok(mut body) => {
			match body.write_all(words.join(separator).as_bytes()) {
				Err(error) => {eprintln!(
					"(!) - dictionaries.files.write - Problem writting `words` (len: {}) into {}. Error: {}",
					words.len(), file_path.display(), error
				)},
				_ => ()
			}
		},
		Err(error) => {eprintln!("(!) - dictionaries.files.write - Can not write `file_path`:{} ({}).", file_path.display(), error);},
	};
}


