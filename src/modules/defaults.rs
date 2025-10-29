// src/defaults.rs

use dotenv;
use std::env;
use std::path;
use glob;

use crate::modules::maths;

#[allow(dead_code)]
pub const DIRECTION_ALL: [maths::Size; 8] = [
   maths::Size { x: 1, y: 0 },
   maths::Size { x: 1, y: 1 },
   maths::Size { x: 0, y: 1 },
   maths::Size { x: -1, y: 1 },
   maths::Size { x: -1, y: 0 },
   maths::Size { x: -1, y: -1 },
   maths::Size { x: 0, y: -1 },
   maths::Size { x: 1, y: -1 },

];

#[allow(dead_code)]
pub const DIRECTION_RIGHT_TO_LEFT: [maths::Size; 4] = [
   maths::Size { x: 1, y: 0 },
   maths::Size { x: 1, y: 1 },
   maths::Size { x: 0, y: -1 },
   maths::Size { x: 1, y: -1 },

];


pub struct PathsTuple {
	pub dictionaries: path::PathBuf,
}
impl PathsTuple {
	/// Return a `&Path` of the `dictionary`.
	pub fn get_dictionaries(self: &Self) -> &path::Path {
		self.dictionaries.clone().leak()
	}

	pub fn list_dictionaries(self: &Self) -> Vec<path::PathBuf> {
		let expression: &str = &(self.dictionaries
			.to_str()
			.expect("(X) - modules.default.PathsTuple - Error transforming `to_str`.")
			.to_owned()
			+ "/*"
		);
		println!("{}", expression);
		match glob::glob(expression) {
			Ok(files) => {
				let mut list: Vec<path::PathBuf> = vec![];
				for file in files {
					match file {
						Ok(path_) => list.push(path_),
						Err(_) => (),
					}
				}

				list
			},
			Err(error) => {
				eprint!("(!) - modules.default.PathsTuple - Error globing `dictionaries`: {}.", error);
				vec![]
			}
		}
	}
}


/// Extract the paths from the `.env` file.
pub fn paths() -> PathsTuple {
	dotenv::dotenv().expect("modules.defaults.paths - Couldn't load .env file.");
	
	let dictionaries_path: path::PathBuf = match env::var("DICTIONARIES") {
		Ok(value) => path::PathBuf::from(value),
		Err(error) => {
			eprintln!("(!) - Error reading env key `DICTIONARIES`: {} \n -> Falling back to `./data/dictionaries`", error);
			path::PathBuf::from("./data/dictionaries")
		},
	};

	PathsTuple { 
		dictionaries: dictionaries_path,
	}
}




