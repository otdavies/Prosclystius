use crate::app::{
	solver::constants::{DIMENSIONS, GRID_SIZE, LEGAL_DIRECTION},
	solver::possibility::Possibility,
};
use std::collections::HashMap;

pub struct PossibilitySpace {
	variations: usize,
	possibilities: HashMap<u8, Possibility>,
}

impl PossibilitySpace {
	pub fn new(example: [[u8; GRID_SIZE]; GRID_SIZE], variations: usize) -> Self {
		Self {
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
		if result < 1 {
			println!("Result is zero, this isn't legal");
		}
		return result;
	}
}

fn calculate_possibilities(example: [[u8; GRID_SIZE]; GRID_SIZE], variation_count: usize) -> HashMap<u8, Possibility> {
	let mut possibilities: HashMap<u8, Possibility> = HashMap::with_capacity(variation_count + 1);

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
					p.union_value(1 << example[_x][_y], i);
				}
			}
		}
	}
	return possibilities;
}
