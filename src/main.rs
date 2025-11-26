// src/main.rs

use std::env;

mod grid;
mod dictionaries;
mod modules;
mod ui;

fn main() {
	println!("# Word grid. Initialisation.");

	println!("\n## Environment variables.");
	println!(
		"* Loaded: `{}`, `{}`", 
		modules::defaults::paths().dictionaries.display(), modules::defaults::paths().grids.display()
	);

	let arguments: Vec<String> = env::args().collect();

	ui::inputs::launch_from_arguments(arguments);

}
