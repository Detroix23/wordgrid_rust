// src/main.rs

mod grid;
mod dictionaries;
mod modules;

fn main() {
	println!("# Word grid.");

	println!("## Environment variables.");
	println!(
		"Loaded: `{}`, `{}`", 
		modules::defaults::paths().dictionaries.display(), modules::defaults::paths().grids.display()
	);

	modules::tests::general1();

	// modules::tests::files1();

	// modules::tests::words1();

	// modules::tests::dicho1();
}
