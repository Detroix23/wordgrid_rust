// src/test.rs

use std::cmp;
use std::{path};
use rand::{self, Rng};

use crate::modules::{self, maths, defaults};
use crate::grid;
use crate::dictionaries;

pub fn random_grid(size: maths::Size) -> maths::CharGrid {
	let mut grid: maths::CharGrid = Vec::new();
	let mut rng: rand::prelude::ThreadRng = rand::rng();

	for _ in 0..size.y {
		let mut line: Vec<char> = Vec::new();
		for _ in 0..size.x {
			line.push(rng.random_range(b'a'..=b'z') as char);
		}
		grid.push(line);
	}
	
	grid
}

/// General test.
pub fn general1() -> () {

	println!("Dict folder: {}", defaults::paths().get_dictionaries().display());
	println!("Contains: {:?}", defaults::paths().list_dictionaries());

    let grid1: maths::CharGrid = vec![
        vec!['a', 'b', 'c'],
        vec!['d', 'e', 'f'],
        vec!['g', 'h', 'i'],
    ];

    let wg1: grid::Grid = grid::Grid::new(
        &grid1,
        defaults::DIRECTION_ALL.to_vec(),
    );

    wg1.display(1usize);

    let read: Vec<String> = wg1.read();

    println!("! wg1 - Read{:?}", read);

	let grid2: maths::CharGrid = random_grid(maths::Size { x: 10, y: 10 });

	let wg2: grid::Grid = grid::Grid::new(
        &grid2,
        defaults::DIRECTION_ALL.to_vec(),
    );

    wg2.display(1usize);

    let read: Vec<String> = wg2.read();

    println!("! wg2 - Read{:?}, length={}", read, read.len());

}

/// Test files 1.
pub fn files1() -> () {
	let lines = dictionaries::files::read(path::Path::new("./data/file1.txt"));

	println!("file1 lines: {:?}", lines);
}

/// Test word 1.
pub fn words1() -> () {
	let words_a: Vec<&str> = vec!["-", "a", "b", "c", "A", "av", "ab", "aaac", "asdasdasdasd", "z"];
	let words_b: Vec<&str> = vec!["-", "a", "c", "a", "z", "ab", "aasd", "aaacd", "asdasdasdasdy", "aaaaaaa"];
	let mut cursor: usize = 0;

	while cursor < cmp::min(words_a.len(), words_b.len()) {
		println!(
			"{} ? {}: {:#?} (left is greater: `{}`)", 
			words_a[cursor], 
			words_b[cursor], 
			dictionaries::words::compare(words_a[cursor], words_b[cursor]),
			dictionaries::words::is_greater(words_a[cursor], words_b[cursor]),
		);

		cursor += 1;
	}
}

pub fn dicho1() -> () {
	let words: Vec<&str> = vec!["my", "abc", "zxcb", "sfsfdasjfjdask", "gastroenterological", "repanel"];
	let dictionary = dictionaries::files::read_from_data("english_alpha1.txt");


	for word in words {
		println!("{}: {:#?}", word, dictionaries::search::dichotomy(word.to_string(), &dictionary))
	}
}