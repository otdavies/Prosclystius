use crate::{app::solver::cell::Cell, app::solver::constants::DIMENSIONS};

pub struct Possibility {
	pub constraints: [u128; DIMENSIONS],
}

impl Possibility {
	pub fn new() -> Self {
		Self { constraints: [0; DIMENSIONS] }
	}

	pub fn union(&mut self, value: u128, direction: usize) {
		println!("unioning {}({:b}) in direction {}", 128 - value.leading_zeros() - 1, value, direction);
		self.constraints[direction] |= value;
	}

	pub fn intersect(&mut self, value: u128, direction: usize) {
		self.constraints[direction] &= value;
	}

	pub fn get_constraint(&self, direction: usize) -> u128 {
		return self.constraints[direction];
	}

	pub fn to_string(&self) -> String {
		return format!(
			"   {:b}  \n{:b}  X  {:b}\n   {:b}  ",
			self.constraints[0], self.constraints[1], self.constraints[2], self.constraints[3],
		);
	}

	fn to_value(&self, constraint: u128) -> u8 {
		return (128 - constraint.leading_zeros() - 1) as u8;
	}
}
