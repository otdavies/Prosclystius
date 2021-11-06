pub struct Cell {
	// u128 represents all the possible unique states (up to 128) the Cell can be in
	pub super_position: u128,
	pub variations: u32,
}

impl Cell {
	pub fn new(variations: u32) -> Self {
		Self {
			// All positions
			super_position: u128::MAX,
			variations: if variations < 128 { variations } else { 127 },
		}
	}

	pub fn constrain(&mut self, neighbor_constraints: &[u128]) {
		let mut unioned_constraints: u128 = 0;
		for constraint in neighbor_constraints {
			unioned_constraints |= constraint;
		}
		self.super_position &= unioned_constraints;

		// prune neighbors possibility space. Lower the "degree" of the super position.
		// union all constraints in a given direction
		// intersect unioned constraints with each neighbor
	}

	// Collapse the possibility space to a single outcome
	pub fn collapse(&mut self, mut identity: u32) {
		if identity > 127 {
			identity = 127;
		}

		self.super_position &= 1 << identity;
		// let out = 128 - self.super_position.leading_zeros() - 1; // may not work
	}

	pub fn print(&self) -> String {
		let mut result = String::new();
		for v in 0..self.variations {
			if self.super_position & (1 << v) > 0 {
				result.push_str(&v.to_string());
			}
		}
		return result;
	}

	// Produce a value between 0 - 1 that represents the entropy of the cell
	// Near zero implies stability
	pub fn get_entropy(&self) -> f32 {
		return self.super_position.count_ones() as f32 / self.variations as f32;
	}
}

#[test]
fn cell_sanity_checks() {
	let mut cell: Cell = Cell::new(9);
	let constraints: [u128; 4] = [1 << 8, 1 << 4, 1 << 2, 1 << 0];
	cell.constrain(&constraints);
	assert_eq!(format!("{:b}", cell.super_position), "100010101");
	assert_eq!(cell.get_entropy(), 4.0 / cell.variations as f32);
	cell.collapse(8);
	assert_eq!(format!("{:b}", cell.super_position), "100000000");
	assert_eq!(cell.get_entropy(), 1.0 / cell.variations as f32);
}