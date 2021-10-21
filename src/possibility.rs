use crate::constants::GRID_DIRECTIONS;

pub struct Possibility {
	pub constraints: [u128; GRID_DIRECTIONS],
}

impl Possibility {
	pub fn new() -> Self {
		Self { constraints: [0; GRID_DIRECTIONS] }
	}

	pub fn union(&mut self, other: &Possibility, direction: usize) {
		self.constraints[direction] |= other.constraints[direction];
	}

	pub fn intersection(&mut self, other: &Possibility, direction: usize) {
		self.constraints[direction] &= other.constraints[direction];
	}

	pub fn union_value(&mut self, value: u128, direction: usize) {
		println!("unioning {}({:b}) in direction {}", 128 - value.leading_zeros() - 1, value, direction);
		self.constraints[direction] |= value;
	}

	pub fn intersection_value(&mut self, value: u128, direction: usize) {
		self.constraints[direction] &= value;
	}

	pub fn to_string(&self) -> String {
		return format!(
			"   {:b}  \n{:b}  X  {:b}\n   {:b}  ",
			self.constraints[3], self.constraints[2], self.constraints[0], self.constraints[1],
		);
	}

	fn to_value(&self, constraint: u128) -> u8 {
		return (128 - constraint.leading_zeros() - 1) as u8;
	}
}
