mod cell;
mod constants;
mod possibility;
mod possibility_space;

use cell::Cell;
use constants::DIMENSIONS;
use constants::GRID_SIZE;
use constants::LEGAL_DIRECTION;
use possibility_space::PossibilitySpace;

fn main() {
	// Blank world with all superpositions representing all possible states
	let mut world: [[Cell; GRID_SIZE]; GRID_SIZE] = [0; GRID_SIZE].map(|_| [0; GRID_SIZE].map(|_| Cell::new(4)));

	// Example of what a potential "world" could look like
	let example: [[u8; GRID_SIZE]; GRID_SIZE] = [[1, 2, 2, 1], [1, 3, 3, 1], [1, 3, 0, 1], [1, 2, 2, 1]];
	/*
		[1, 2, 2, 1],
		[1, 3, 3, 1],
		[1, 3, 0, 1],
		[1, 2, 2, 1]
	*/

	println!("--- Setup ---");
	// Define learned possibilities
	let possibilities = PossibilitySpace::new(example, 4);

	println!("--- Solve ---");
	// Let's collapse a corner
	world[0][0].collapse(1);

	let mut stack = Vec::new();
	stack.push((0, 0));
	while stack.len() > 0 {
		let (x, y) = stack.pop().unwrap();
		let super_position = world[x][y].super_position;
		for i in 0..DIMENSIONS {
			let (safe, pos) = LEGAL_DIRECTION(i, x, y);
			if safe {
				let neighbor = &mut world[pos.0][pos.1];
				let constraint = possibilities.collect(super_position, i);
				neighbor.constrain(constraint);
				// println!("Constraining ({}, {}):{}", pos.0, pos.1, neighbor.print());
				if neighbor.dirty {
					stack.push(pos);
					neighbor.dirty = false;
				}
			}
		}
	}

	for x in 0..GRID_SIZE {
		for y in 0..GRID_SIZE {
			print!("{} ", world[x][y].print());
		}
		println!("");
	}
}
