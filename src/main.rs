// src/main.rs

mod maths;
mod grid;


const DIRECTION_ALL: [maths::Size; 8] = [
   maths::Size { x: 1, y: 0 },
   maths::Size { x: 1, y: 1 },
   maths::Size { x: 0, y: 1 },
   maths::Size { x: -1, y: 1 },
   maths::Size { x: -1, y: 0 },
   maths::Size { x: -1, y: -1 },
   maths::Size { x: 0, y: -1 },
   maths::Size { x: 1, y: -1 },

];

const DIRECTION_RIGHT_TO_LEFT: [maths::Size; 4] = [
   maths::Size { x: 1, y: 0 },
   maths::Size { x: 1, y: 1 },
   maths::Size { x: 0, y: -1 },
   maths::Size { x: 1, y: -1 },

];


fn main() {
    println!("# Word grid.");

    let grid1: maths::CharGrid = vec![
        vec!['a', 'b', 'c'],
        vec!['d', 'e', 'f'],
        vec!['g', 'h', 'i'],
    ];

    let directions1: Vec<maths::Size> = DIRECTION_ALL.to_vec();

    let wg: grid::Grid = grid::Grid::new(
        &grid1,
        directions1,
    );

    wg.display(2usize);

    let read = wg.read();

    println!("! wg1 - Read{:?}", read);

}




