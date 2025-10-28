// src/grid/mod.rs

use std::{char};

use crate::modules::maths;


/// Define 1 solution.
pub struct Solution {
    position: maths::Size,
    direction: maths::Size,
    word: String,
}


/// Solutions and the grid itself.
pub struct Grid<'a> {
    grid: &'a maths::CharGrid,
    found: Vec<Solution>,
    size: maths::Size,
    directions: Vec<maths::Size>,
}

impl Grid<'_> {
    /// Construct a new grid using a given CharGrid.
    pub fn new<'a>(grid: &'a maths::CharGrid, directions: Vec<maths::Size>) -> Grid<'a> {
        // Find maximum width.
		let mut max_x: usize = 0;
        for line in grid {
            if line.len() > max_x {
                max_x = line.len();
            }
        }

        Grid { 
            grid: grid, 
            found: Vec::new(), 
            size: maths::Size { 
                x: max_x as i32,
                y: grid.len() as i32,
            },
            directions,
        }
    }

    /// Prints the grid to the console.
    pub fn display(self: &Self, space: usize) -> () {
        for lines in self.grid {
            for character in lines {
                print!("{}{}", character, " ".repeat(space));
            }
            println!();
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
		lines: &mut Vec<String>, 
		start: maths::Size, 
		direction: maths::Size
	) {
		// Extend the reach slowly.
		let mut reach: usize = 2;
		let mut reach_in_grid: bool = true;

		while reach_in_grid {
			// Read to the end of grid.
			let mut cursor:maths::Size =maths::Size { 
				x: start.x, 
				y: start.y, 
			};
			let mut reading: String = String::new();
			let mut steps: usize = 0;

			while 
				self.in_grid(cursor) 
				&& steps < reach
			{   
				let character: &char = &self.grid[cursor.y as usize][cursor.x as usize];
				reading += &character.to_string();
				
				cursor.x += direction.x;
				cursor.y += direction.y;

				steps += 1;
			}

			if reading.len() > 1 {
				lines.push(reading);
			}
			
			reach_in_grid = self.in_grid(cursor);
			reach += 1;					
		}
	}

    /// Read in all spots, and in all the given directions the grid.
    pub fn read(self: &Self) -> Vec<String> {
        let mut lines: Vec<String> = Vec::new();

		// Loop for all positions.
        for (start_y, line) in self.grid.iter().enumerate() {
            for (start_x, _) in line.iter().enumerate() {
				// Loop for all directions.
				for direction in &self.directions {
                    self.arm(
						&mut lines, 
						maths::Size { x: start_x as i32, y: start_y as i32}, 
						*direction
					);
				}
            }
        }

        lines
    }

}
