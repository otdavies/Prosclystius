use crate::{
	cell::Cell,
	constants::{DIRECTION_DOWN, DIRECTION_LEFT, DIRECTION_RIGHT, DIRECTION_UP, GRID_SIZE},
	possibility::Possibility,
	BOUND_CHECK,
};
use std::collections::HashMap;

pub struct PossibilitySpace {
	example: [[u8; GRID_SIZE]; GRID_SIZE],
	variations: usize,
	possibilities: HashMap<u8, Possibility>,
}

impl PossibilitySpace {
	pub fn new(example: [[u8; GRID_SIZE]; GRID_SIZE], variations: usize) -> Self {
		Self {
			example,
			variations,
			possibilities: calculate_possibilities(example, variations),
		}
	}

	pub fn print(&self) -> String {
		let mut result = String::new();
		for i in 0..self.variations {
			let key = i as u8;
			if self.possibilities.contains_key(&key) {
				result.push_str(format!("{}", key).as_str());
				result.push_str("|-------------------\n");
				result.push_str(self.possibilities.get(&key).unwrap().to_string().as_str());
				result.push('\n');
			}
		}
		return result;
	}

	pub fn collect(&self, super_position: u128, direction: usize) -> u128 {
		// Scan through super_position
		// 0011 & 0010 etc
		let mut result: u128 = 0;
		for i in 0..self.variations {
			if super_position & (1 << i) > 0 {
				let possibility = self.possibilities.get(&(i as u8)).unwrap();
				result |= possibility.get_constraint(direction);
			}
		}
		return result;
	}
}

fn calculate_possibilities(example: [[u8; GRID_SIZE]; GRID_SIZE], size: usize) -> HashMap<u8, Possibility> {
	let mut possibilities: HashMap<u8, Possibility> = HashMap::with_capacity(size + 1);

	// Accumulate possibilities
	for y in 0..size {
		let y_i32 = y as i32;
		for x in 0..size {
			let x_i32 = x as i32;
			let e = example[x][y];
			if !possibilities.contains_key(&e) {
				println!("Creating possibility for {}", e);
				possibilities.insert(e, Possibility::new());
			}
			let p: &mut Possibility = possibilities.get_mut(&e).unwrap();

			// Right
			if BOUND_CHECK!(x_i32, y_i32 + 1) {
				p.union_value(1 << example[x][y + 1], DIRECTION_RIGHT);
			}
			// Down
			if BOUND_CHECK!(x_i32 + 1, y_i32) {
				p.union_value(1 << example[x + 1][y], DIRECTION_DOWN)
			}
			// Left
			if BOUND_CHECK!(x_i32, y_i32 - 1) {
				p.union_value(1 << example[x][y - 1], DIRECTION_LEFT)
			}
			// Up
			if BOUND_CHECK!(x_i32 - 1, y_i32) {
				p.union_value(1 << example[x - 1][y], DIRECTION_UP)
			}
		}
	}
	return possibilities;
}
