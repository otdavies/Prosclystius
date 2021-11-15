use rand::prelude::*;
pub struct Cell {
	// u128 represents all the possible unique states (up to 128) the Cell can be in
	pub super_position: u128,
	pub total_variations: u32,
	pub remaining_variations: u32,
	pub dirty: bool,
	random: ThreadRng,
}

impl Cell {
	pub fn new(variations: u32) -> Self {
		let v = if variations < 128 { variations } else { 127 };
		Self {
			// All positions
			super_position: (1 << variations) - 1,
			total_variations: v,
			remaining_variations: v,
			dirty: false,
			random: rand::thread_rng(),
		}
	}

	pub fn constrain_multiple(&mut self, neighbor_constraints: &[u128]) {
		let previous_state = self.super_position;
		let mut unioned_constraints: u128 = 0;
		for constraint in neighbor_constraints {
			unioned_constraints |= constraint;
		}
		self.super_position &= unioned_constraints;
		self.dirty = previous_state != self.super_position;

		if self.dirty {
			self.remaining_variations = self.super_position.count_ones();
		}
	}

	pub fn constrain(&mut self, constraint: u128) {
		let previous_state = self.super_position;
		self.super_position &= constraint;
		self.dirty = previous_state != self.super_position;

		if self.dirty {
			self.remaining_variations = self.super_position.count_ones();
		}
	}

	// Collapse the possibility space to a single outcome
	pub fn collapse(&mut self, mut identity: u32) -> bool {
		let previous_state = self.super_position;

		if self.super_position & (1 << identity) < 1 {
			println!("Not a legal constraint!");
			return false;
		}

		if identity > 127 {
			identity = 127;
		}

		self.super_position &= 1 << identity;
		self.dirty = previous_state != self.super_position;
		self.remaining_variations = 1;
		return true;
	}

	pub fn collapse_random(&mut self) {
		let randomNum: u128 = self.random.gen_range(1..self.super_position).into();
		for v in 0..self.total_variations {
			let num = (1 << v);
			if (self.super_position & num) > 0 {
				if (num) <= randomNum {
					self.collapse(v);
				}
			}
		}
	}

	pub fn print(&self) -> String {
		let mut result = String::new();
		for v in 0..self.total_variations {
			if (self.super_position & (1 << v)) > 0 {
				result.push_str(&v.to_string());
			}
		}
		result.push_str(format!(" [{}]", self.get_entropy()).as_str());
		return result;
	}

	pub fn is_stable(&self) -> bool {
		return self.remaining_variations == 1;
	}

	// Produce a value between 0 - 1 that represents the entropy of the cell
	// Near zero implies stability
	pub fn get_entropy(&self) -> f32 {
		return (self.remaining_variations as f32 - 1.0) / (self.total_variations as f32 - 1.0);
	}

	pub fn get_stable_value(&self) -> u8 {
		return self.super_position.trailing_zeros() as u8;
	}
}

#[test]
fn cell_sanity_checks() {
	let mut cell: Cell = Cell::new(9);
	let constraints: [u128; 4] = [1 << 8, 1 << 4, 1 << 2, 1 << 0];
	cell.constrain_multiple(&constraints);
	assert_eq!(format!("{:b}", cell.super_position), "100010101");
	assert_eq!(cell.get_entropy(), 4.0 / cell.total_variations as f32);
	cell.collapse(8);
	assert_eq!(format!("{:b}", cell.super_position), "100000000");
	assert_eq!(cell.get_entropy(), 1.0 / cell.total_variations as f32);
}
