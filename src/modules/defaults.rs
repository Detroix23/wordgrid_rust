// src/defaults.rs

use dotenv;
use std::env;
use std::path;
use glob;

use crate::modules::maths;

/// Default (fall-back) dictionary file name.
pub const DICTIONARY: &'static str = "hermitdave-FrequencyWords_en-50k.sorted";
/// Default (fall-back) grid file name.
pub const GRID: &'static str = "grid.txt";

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
	pub grids: path::PathBuf,
	pub results: path::PathBuf,
}
impl PathsTuple {
	/// Return a `&Path` of the `dictionary`.
	pub fn get_dictionaries(self: &Self) -> &path::Path {
		self.dictionaries.clone().leak()
	}

	/// Return a `&Path` of the `dictionary`.
	pub fn get_grids(self: &Self) -> &path::Path {
		self.grids.clone().leak()
	}

	pub fn get_results(self: &Self) -> &path::Path {
		self.results.clone().leak()
	}
}

/// List path.
pub fn list_path(target: path::PathBuf) -> Vec<path::PathBuf> {
	let expression: &str = &(target
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

	let grids_path: path::PathBuf = match env::var("GRIDS") {
		Ok(value) => path::PathBuf::from(value),
		Err(error) => {
			eprintln!("(!) - Error reading env key `GRIDS`: {} \n -> Falling back to `./data/grids`", error);
			path::PathBuf::from("./data/grids")
		},
	};

	let results_path: path::PathBuf = match env::var("RESULTS") {
		Ok(value) => path::PathBuf::from(value),
		Err(error) => {
			eprintln!("(!) - Error reading env key `GRIDS`: {} \n -> Falling back to `./data/grids`", error);
			path::PathBuf::from("./data/grids")
		},
	};

	PathsTuple { 
		dictionaries: dictionaries_path,
		grids: grids_path,
		results: results_path,
	}
}




