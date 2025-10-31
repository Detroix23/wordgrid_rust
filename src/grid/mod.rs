// src/grid/mod.rs

use std::{
	char
};

pub mod solutions;
pub mod files;
pub mod ui;

use crate::{ 
	modules::{
		maths
	},
	dictionaries,
};

/// Solutions and the grid itself.
pub struct Grid {
	name: String,
    grid: maths::CharGrid,
    found: Vec<solutions::Solution>,
	total_words: usize,
    size: maths::Size,
    directions: Vec<maths::Size>,
	dictionary: dictionaries::base::WordList,
}

impl Grid {
    /// Construct a new grid using a given CharGrid.
    pub fn new(
		name: String,
		grid: maths::CharGrid, 
		directions: Vec<maths::Size>,
		dictionary: dictionaries::base::WordList,
	) -> Grid {
        // Find maximum width.
		let max_y: usize = grid.len();
		let mut max_x: usize = 0;
        for line in &grid {
            if line.len() > max_x {
                max_x = line.len();
            }
        }

        Grid { 
			name,
            grid: grid, 
            found: Vec::new(),
			total_words: 0usize,
            size: maths::Size { 
                x: max_x as i32,
                y: max_y as i32,
            },
            directions,
			dictionary,
        }
    }

	/// Check if given point is in grid.
	pub fn in_grid(self: &Self, point: maths::Size) -> bool {
		point.x >= 0 
		&& point.x < self.size.x
		&& point.y >= 0
		&& point.y < self.size.y 
	}

	/// Read all possible words from a `start` position in a straight line in a given `direction`.
	/// Write the found words in `lines`.
	/// Used in self.read
	fn arm(
		self: &Self,
		found: &mut Vec<solutions::Solution>, 
		start: maths::Size, 
		direction: maths::Size,
		minimum_length: usize,
	) -> usize {
		let mut reach: usize = 2;
		let mut reach_in_grid: bool = true;
		let mut new_words: usize = 0;

		while reach_in_grid {
			// Read to the end of grid.
			let mut cursor: maths::Size = maths::Size { 
				x: start.x, 
				y: start.y, 
			};
			let mut reading: String = String::new();
			let mut valid_word_start: bool = true;
			let mut steps: usize = 0;

			while 
				self.in_grid(cursor) 
				&& steps < reach
				&& valid_word_start
			{   
				let character: &char = &self.grid[cursor.y as usize][cursor.x as usize];
				reading += &character.to_string();
				
				cursor.x += direction.x;
				cursor.y += direction.y;

				steps += 1;
			}

			if reading.len() > minimum_length {
				new_words += 1;
				solutions::check_word(
					reading, 
					&self.dictionary, 
					found, 
					start, 
					direction
				);
			}
			
			reach_in_grid = self.in_grid(cursor);
			reach += 1;			
		}

		new_words
	}

    /// Read in all spots, and in all the given directions the grid. `minimum_length` is strict.
    pub fn read(self: &mut Self, minimum_length: usize) -> Vec<solutions::Solution> {
        let mut found: Vec<solutions::Solution> = Vec::new();
		let mut new_words: usize = 0;

		// Loop for all positions.
        for (start_y, line) in self.grid.iter().enumerate() {
            for (start_x, _) in line.iter().enumerate() {
				// Loop for all directions.
				for direction in &self.directions {
                    new_words += self.arm(
						&mut found, 
						maths::Size { x: start_x as i32, y: start_y as i32}, 
						*direction,
						minimum_length
					);
				}
            }
        }

		self.total_words += new_words;
		self.found = found.clone();
        found
    }

}
