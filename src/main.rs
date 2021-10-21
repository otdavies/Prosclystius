mod cell;
mod constants;
mod possibility;

use cell::Cell;
use constants::{DIRECTION_DOWN, DIRECTION_LEFT, DIRECTION_RIGHT, DIRECTION_UP, GRID_SIZE, POSSIBILITY_SPACE_SIZE};
use possibility::Possibility;
use std::collections::HashMap;

fn main() {
	// Blank world with all superpositions representing all possible states
	let world: [[Cell; GRID_SIZE]; GRID_SIZE] = [0; GRID_SIZE].map(|_| [0; GRID_SIZE].map(|_| Cell::new(4)));

	// Example of what a potential "world" could look like
	let example: [[u8; GRID_SIZE]; GRID_SIZE] = [[1, 2, 2, 1], [1, 3, 3, 1], [1, 3, 0, 1], [1, 2, 2, 1]];

	// Learn from the given example
	let possibilities: HashMap<u8, Possibility> = calculate_possibilities(example, POSSIBILITY_SPACE_SIZE);
	println!("{}", possibilities.get(&1).unwrap().to_string());

	// Collapse and propagate
}

fn calculate_possibilities(example: [[u8; GRID_SIZE]; GRID_SIZE], possibility_space_size: usize) -> HashMap<u8, Possibility> {
	let mut possibilities: HashMap<u8, Possibility> = HashMap::with_capacity(possibility_space_size + 1);
	let bound_check = |x: i32, y: i32| x >= 0 && x < GRID_SIZE as i32 && y >= 0 && y < GRID_SIZE as i32;

	// Accumulate possibilities
	for x in 0..possibility_space_size {
		let x_i32 = x as i32;
		for y in 0..possibility_space_size {
			let y_i32 = y as i32;
			let e = example[x][y];
			if !possibilities.contains_key(&e) {
				println!("Creating possibility for {}", e);
				possibilities.insert(e, Possibility::new());
			}
			let p: &mut Possibility = possibilities.get_mut(&e).unwrap();

			// Up
			if bound_check(x_i32, y_i32 + 1) {
				p.union_value(1 << example[x][y + 1], DIRECTION_UP);
			}
			// Right
			if bound_check(x_i32 + 1, y_i32) {
				p.union_value(1 << example[x + 1][y], DIRECTION_RIGHT)
			}
			// Down
			if bound_check(x_i32, y_i32 - 1) {
				p.union_value(1 << example[x][y - 1], DIRECTION_DOWN)
			}
			// Left
			if bound_check(x_i32 - 1, y_i32) {
				p.union_value(1 << example[x - 1][y], DIRECTION_LEFT)
			}
		}
	}
	return possibilities;
}
