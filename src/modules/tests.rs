// src/test.rs

use rand::{self, Rng};

use crate::modules::{maths, defaults};
use crate::grid;

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