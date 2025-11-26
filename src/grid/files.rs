// src/grid/files.rs

use std::{
	fs,
	io::{
		self,
		Read,
		Write,
	},
	path,
	char,
};


use crate::{
	modules::{
		self, 
		maths
	}
};

const LINE_SEPARATORS: [char; 2] = ['\n', '\r'];

/// Read a text file into a grid.
pub fn read(file_path: &path::Path) -> maths::CharGrid {
	match fs::File::open(file_path) {
		Ok(file) => {
			let mut buffer_reader: io::BufReader<fs::File> = io::BufReader::new(file);
			let mut content: String = String::new();

			buffer_reader.read_to_string(&mut content).expect("(X) - grid.files.read - Reading to string error.");

			let mut grid: maths::CharGrid = Vec::new();
			let mut line: Vec<char> = Vec::new();
			for character in content.chars() {
				if LINE_SEPARATORS.contains(&character){
					if !line.is_empty() {
						grid.push(line);
					}
					line = Vec::new();
				} else {
					line.push(character.to_ascii_lowercase());
				}
			}
			if !line.is_empty() {
				grid.push(line);
			}

			grid
		},
		Err(error) => {
			eprintln!(
				"(!) - grid.files.read - Error opening given path ({}): {}. Falling back to an empty vec.", 
				file_path.display(), error
			);
			vec![]
		}
	}
	
}

/// Write a list of word. Each word is separated by `separator`.
pub fn write(
	file_path: &path::Path, 
	string: String,
) -> () {
	let file = fs::File::create(file_path);
	match file {
		Ok(mut body) => {body
			.write_all(string.as_bytes())
			.expect(&format!("(X) - grid.files.write - Problem writting `{}`.", string));
		},
		Err(error) => {eprintln!("(!) - dictionaries.files.write - Can not write `file_path`:{} ({}).", file_path.display(), error);},
	};
}

/// Shorthand to get dictionaries path, join the `name` to it, and read it.
pub fn read_from_data(name: String) -> maths::CharGrid {
	read(&modules::defaults::paths().get_grids().join(name))
}
