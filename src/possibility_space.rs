use crate::{
	constants::{DIMENSIONS, GRID_SIZE, LEGAL_DIRECTION},
	possibility::Possibility,
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

	// pub fn collect_all(&self, super_position: u128) -> [u128; GRID_DIRECTION_COUNT] {
	// 	let mut result: [u128; GRID_DIRECTION_COUNT];
	// 	for i in 0..self.variations {
	// 		if super_position & (1 << i) > 0 {
	// 			let possibility = self.possibilities.get(&(i as u8)).unwrap();
	// 			for j in 0..GRID_DIRECTION_COUNT {
	// 				result[j] |= possibility.get_constraint(j);
	// 			}
	// 		}
	// 	}
	// 	return result;
	// }
}

fn calculate_possibilities(example: [[u8; GRID_SIZE]; GRID_SIZE], variation_count: usize) -> HashMap<u8, Possibility> {
	let mut possibilities: HashMap<u8, Possibility> = HashMap::with_capacity(variation_count + 1);

	// Accumulate possibilities
	for y in 0..GRID_SIZE {
		for x in 0..GRID_SIZE {
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
