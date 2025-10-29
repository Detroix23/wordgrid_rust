// src/main.rs

mod grid;
mod dictionaries;
mod modules;

use crate::modules::tests;

fn main() {
	println!("# Word grid.");

	// tests::general1();

	// tests::files1();

	// tests::words1();

	tests::dicho1();
}
