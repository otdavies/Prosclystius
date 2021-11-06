mod cell;
mod constants;
mod possibility;
mod possibility_space;

use cell::Cell;
use constants::GRID_SIZE;
use possibility_space::PossibilitySpace;

fn main() {
	// Blank world with all superpositions representing all possible states
	let mut world: [[Cell; GRID_SIZE]; GRID_SIZE] = [0; GRID_SIZE].map(|_| [0; GRID_SIZE].map(|_| Cell::new()));

	// Example of what a potential "world" could look like
	let example: [[u8; GRID_SIZE]; GRID_SIZE] = [[1, 2, 2, 1], [1, 3, 3, 1], [1, 3, 0, 1], [1, 2, 2, 1]];

	let possibilities = PossibilitySpace::new(example, 4);

	let mut cell = &mut world[0][0];
	cell.collapse(1);
	cell.super_position = possibilities.collect(cell.super_position, 0);
	println!("{:b}", cell.super_position);

	// println!("{}", possibilities.print());

	// Collapse and propagate
}
