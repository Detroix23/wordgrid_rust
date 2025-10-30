// src/grid/mod.rs

use std::{char, fmt::format};

pub mod solutions;
pub mod files;

use crate::{ 
	modules::maths,
	dictionaries,
};

/// Solutions and the grid itself.
pub struct Grid {
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

    /// Return a nice String of the grid.
    pub fn display(self: &Self, space: usize) -> String {
		let mut string: String = String::from("* Displaying grid: \n");


		let vertical: &str = "│";
		let horizontal: &str = "─";
		let corner: &str = "┼";
		let spaces: String = " ".repeat(space);
		let row_separator: String = format!("{}{}", horizontal.repeat(space * 2 + 1), corner);

		let x_axis: Vec<String> = (0..self.size.x)
			.map(|n| n.to_string())
			.collect();
		let longest_x: usize = x_axis
			.iter()
			.fold(0, |length, n| {
				if n.len() > length { n.len() }
				else { length }
			});
		let longest_y: usize = (1..self.size.y)
			.fold(0, |length, n| {
				if n.to_string().len() > length { n.to_string().len() }
				else { length }
			});

		string += &format!("{}{}", " ".repeat(longest_y), vertical);
		for x in x_axis {
			string += &format!(
				"{}{}{}{}", 
				spaces, 
				x, 
				" ".repeat(if space > 0 && x.len() % 2 == 0 { 
					space - 1
				} else {
					space
				}), 
				vertical
			);
		}

		string += &format!("\n{}{}{}\n", horizontal.repeat(longest_y), corner, row_separator.repeat(self.size.x as usize));
	
        for (y, lines) in self.grid.iter().enumerate() {
			let delta: usize = longest_y - (y + 1).to_string().len(); 
            
			for (x, character) in lines.iter().enumerate() {
				if x == 0 {
					string += &format!("{}{}{}", " ".repeat(delta), y + 1, vertical);
				}
                string += &format!("{}{}{}{}", spaces, character, spaces, vertical);
            }
            string += &format!("\n{}{}{}\n", horizontal.repeat(longest_y), corner, row_separator.repeat(self.size.x as usize));
        }

		string
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

	/// Report nicely.
	pub fn report_solutions(self: &Self, columns: usize) -> () {
		let longest: usize = self.found
			.iter()
			.fold(0, |length, solution| 
				if solution.display().len() > length {
					solution.display().len()
				} else {
					length
				}
			);

		for (index, solution) in self.found.iter().enumerate() {
			if index % columns == 0 {
				println!();
				print!(
					"{}{}. ", 
					" ".repeat((self.found.len() / columns).to_string().len() - (index / columns).to_string().len()), 
					index / columns
				);
			}
			print!("{}{}, ", solution.display(), " ".repeat(longest - solution.display().len() + 1));
		}
		println!();
		println!(
			"=> n(words) = {}, n(solutions) = {}, q = {}.", 
			self.total_words, self.found.len(), self.found.len() as f32 / self.total_words as f32,
		);
	}
}
