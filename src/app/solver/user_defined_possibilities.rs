// TODO Fix issue where edges can be legal connections to tiles

use crate::app::{
	solver::constants::{DIMENSIONS, GRID_SIZE, LEGAL_DIRECTION},
	solver::possibility::Possibility,
	solver::possibility_space::PossibilitySpace,
};
use std::collections::HashMap;

pub struct ExampleBasedPossibilities {
	variations: u32,
	possibilities: HashMap<u8, Possibility>,
}

impl ExampleBasedPossibilities {
	pub fn new(example: &[[u8; GRID_SIZE]; GRID_SIZE], variations: u32) -> Self {
		Self {
			variations,
			possibilities: calculate_possibilities(example, variations),
		}
	}
}

fn calculate_possibilities(example: &[[u8; GRID_SIZE]; GRID_SIZE], variation_count: u32) -> HashMap<u8, Possibility> {
	let mut possibilities: HashMap<u8, Possibility> = HashMap::with_capacity((variation_count + 1) as usize);

	// Accumulate possibilities
	for x in 0..GRID_SIZE {
		for y in 0..GRID_SIZE {
			let e = example[x][y];
			if !possibilities.contains_key(&e) {
				println!("Creating possibility for {}", e);
				possibilities.insert(e, Possibility::new());
			}

			let p: &mut Possibility = possibilities.get_mut(&e).unwrap();
			for i in 0..DIMENSIONS {
				let (safe, (_x, _y)) = LEGAL_DIRECTION(i, x, y);
				if safe {
					p.union(1 << example[_x][_y], i);
				}
			}
		}
	}
	return possibilities;
}

impl PossibilitySpace for ExampleBasedPossibilities {
	fn print(&self) -> String {
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

	fn constraints(&self, super_position: u128, direction: usize) -> u128 {
		// Scan through super_position
		// 0011 & 0010 etc
		let mut result: u128 = 0;
		for i in 0..self.variations {
			if super_position & (1 << i) > 0 {
				let possibility = self.possibilities.get(&(i as u8)).unwrap();
				result |= possibility.get_constraint(direction);
			}
		}
		if result < 1 {
			println!("Result is zero, this isn't legal");
		}
		return result;
	}
}
