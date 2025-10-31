// src/grid/ui.rs

use crate::{
	grid,
	modules,
};


impl grid::Grid {
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
		let longest_y: usize = (0..self.size.y)
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
			let delta: usize = longest_y - y.to_string().len(); 
            
			for (x, character) in lines.iter().enumerate() {
				if x == 0 {
					string += &format!("{}{}{}", " ".repeat(delta), y, vertical);
				}
                string += &format!("{}{}{}{}", spaces, character, spaces, vertical);
            }
            string += &format!("\n{}{}{}\n", horizontal.repeat(longest_y), corner, row_separator.repeat(self.size.x as usize));
        }

		string
    }

	pub fn solution_tiles(self: &Self) -> usize {
		self.found
			.iter()
			.fold(0, |tiles: usize, solution: &grid::solutions::Solution| {
				tiles + solution.word.len()
			})
	}

	/// Report nicely.
	pub fn report_solutions(self: &Self, columns: usize) -> String {
		let mut string: String = String::from("* Solutions");
		string += &grid::solutions::display_solutions(
			&grid::solutions::sort(&self.found), 
			columns
		);
		string += "\n\n";
		string += &format!(
			"* Statistics: 
n(words) = {}, n(solutions) = {}, q = {};
n(tiles) = {}, n(solution_tiles) = {}, q = {};
", 
			self.total_words, 
			self.found.len(), 
			self.found.len() as f32 / self.total_words as f32,
			self.size.x * self.size.y,
			self.solution_tiles(),
			self.solution_tiles() as f32 / (self.size.x * self.size.y) as f32,
			
		);

		string
	}

	/// Return a complete report in a `String`.
	pub fn report_all(self: &Self) -> String {
		let mut string: String = String::from(format!("# Grid report: {}\n", self.name.clone()));
		string += "\n";
		string += &self.display(1);
		string += "\n";
		string += &self.report_solutions(4);
		string += "\n";

		string
	}

	/// Create a new file reporting the grid and the solutions.
	pub fn write_report(self: &Self) -> () {
		grid::files::write(
			&modules::defaults::paths().get_results().join(self.name.clone() + ".grid-report"),
			self.report_all(),
		);
		eprintln!("! Succefully written a report for grid `{}`.", self.name);
	}
}


