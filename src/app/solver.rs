mod cell;
pub(crate) mod constants;
mod possibility;
mod possibility_space;

use cell::Cell;
use constants::BOUND_CHECK;
use constants::DIMENSIONS;
use constants::GRID_SIZE;
use constants::LEGAL_DIRECTION;
use possibility_space::PossibilitySpace;

pub struct Solver {
	pub world: Option<[[Cell; GRID_SIZE]; GRID_SIZE]>,
	possibilities: Option<PossibilitySpace>,
	lowest_entropy: (f32, usize, usize),
	trained: bool,
}

impl Solver {
	pub fn new() -> Self {
		Self {
			world: None,
			possibilities: None,
			lowest_entropy: (1.0, 0, 0),
			trained: false,
		}
	}

	pub fn train(&mut self, example: &[[u8; GRID_SIZE]; GRID_SIZE], variations: u32) {
		self.world = Some([0; GRID_SIZE].map(|_| [0; GRID_SIZE].map(|_| Cell::new(variations))));
		self.possibilities = Some(PossibilitySpace::new(example, variations));
		self.trained = true;
	}

	pub fn is_trained(&self) -> bool {
		return self.trained;
	}

	pub fn world_width(&self) -> usize {
		return self.world.as_ref().unwrap().len();
	}

	pub fn get_cell(&self, x: usize, y: usize) -> &Cell {
		let world = self.world.as_ref().unwrap();
		return &world[x][y];
	}

	pub fn propagate(&mut self, px: usize, py: usize, identity: u32) {
		let world = self.world.as_mut().unwrap();
		let possibilities = &self.possibilities.as_mut().unwrap();

		// Make sure we are within the bounds
		if !BOUND_CHECK(px as i32, py as i32) {
			println!("Out of bounds!");
			return;
		}

		// Make sure this isn't already  collapsed
		if world[px][py].remaining_variations == 1 {
			println!("Already collapsed!");
			return;
		}

		// Attempt to collapse and propagate changes therein
		if world[px][py].collapse(identity) {
			let mut stack = Vec::new();
			stack.push((px, py));
			while stack.len() > 0 {
				// Get the coordinates of the cell we are currently working on
				let (x, y) = stack.pop().unwrap();
				let super_position = world[x][y].super_position;

				// Look at all the cells around ourself and apply constraints
				for i in 0..DIMENSIONS {
					let (safe, pos) = LEGAL_DIRECTION(i, x, y);
					if safe {
						let neighbor = &mut world[pos.0][pos.1];
						// let entropy = neighbor.get_entropy();
						// if !neighbor.is_stable() && entropy < self.lowest_entropy.0 {
						// 	self.lowest_entropy = (entropy, pos.0, pos.1);
						// }
						let constraint = possibilities.collect(super_position, i);
						neighbor.constrain(constraint);
						// println!("Constraining ({}, {}):{}", pos.0, pos.1, neighbor.print());

						// Did this neighbor change? If so push it onto the stack to propagate next
						if neighbor.dirty {
							stack.push(pos);
							neighbor.dirty = false;
						}
					}
				}
				// if stack.len() == 0 {
				// 	world[self.lowest_entropy.1][self.lowest_entropy.2].collapse_random();
				// 	stack.push((self.lowest_entropy.1, self.lowest_entropy.2));
				// }
			}
		}
	}
}
