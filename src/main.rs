mod cell;
mod constants;
mod possibility;
mod possibility_space;

use cell::Cell;
use constants::DIRECTION_DOWN;
use constants::DIRECTION_LEFT;
use constants::DIRECTION_RIGHT;
use constants::DIRECTION_UP;
use constants::GRID_SIZE;
use possibility_space::PossibilitySpace;

fn main() {
	// Blank world with all superpositions representing all possible states
	let mut world: [[Cell; GRID_SIZE]; GRID_SIZE] = [0; GRID_SIZE].map(|_| [0; GRID_SIZE].map(|_| Cell::new(4)));

	// Example of what a potential "world" could look like
	let example: [[u8; GRID_SIZE]; GRID_SIZE] = [[1, 2, 2, 1], [1, 3, 3, 1], [1, 3, 0, 1], [1, 2, 2, 1]];

	let possibilities = PossibilitySpace::new(example, 4);

	let mut cell = &mut world[0][0];
	println!("{}", cell.print());
	cell.collapse(0);
	println!("{}", cell.print());
	cell.super_position = possibilities.collect(cell.super_position, DIRECTION_RIGHT);
	println!(" -> {}", cell.print());

	println!("{}", BOUND_CHECK!(2, 0));

	// println!("{}", possibilities.print());

	// Collapse and propagate
}
