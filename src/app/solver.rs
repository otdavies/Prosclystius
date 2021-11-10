mod cell;
mod constants;
mod possibility;
mod possibility_space;

use cell::Cell;
use constants::BOUND_CHECK;
use constants::DIMENSIONS;
use constants::GRID_SIZE;
use constants::LEGAL_DIRECTION;
use possibility_space::PossibilitySpace;

pub struct Solver {
	pub world: [[Cell; GRID_SIZE]; GRID_SIZE],
	possibilities: PossibilitySpace,
}

impl Solver {
	pub fn new() -> Self {
		// TODO move this into a param, this is just for testing
		let example: [[u8; GRID_SIZE]; GRID_SIZE] = [[1, 2, 2, 1], [1, 3, 3, 1], [1, 3, 0, 1], [1, 2, 2, 1]];
		Self {
			world: [0; GRID_SIZE].map(|_| [0; GRID_SIZE].map(|_| Cell::new(4))),
			possibilities: PossibilitySpace::new(example, 4),
		}
	}

	pub fn propagate(&mut self, px: usize, py: usize, identity: u32) {
		// Make sure we are within the bounds
		if !BOUND_CHECK(px as i32, py as i32) {
			println!("Out of bounds!");
			return;
		}

		// Make sure this isn't already  collapsed
		if self.world[px][py].remaining_variations == 1 {
			println!("Already collapsed!");
			return;
		}

		// Attempt to collapse and propagate changes therein
		if self.world[px][py].collapse(identity) {
			let mut stack = Vec::new();
			stack.push((px, py));
			while stack.len() > 0 {
				// Get the coordinates of the cell we are currently working on
				let (x, y) = stack.pop().unwrap();
				let super_position = self.world[x][y].super_position;

				// Look at all the cells around ourself and apply constraints
				for i in 0..DIMENSIONS {
					let (safe, pos) = LEGAL_DIRECTION(i, x, y);
					if safe {
						let neighbor = &mut self.world[pos.0][pos.1];
						let constraint = self.possibilities.collect(super_position, i);
						neighbor.constrain(constraint);
						// println!("Constraining ({}, {}):{}", pos.0, pos.1, neighbor.print());

						// Did this neighbor change? If so push it onto the stack to propagate next
						if neighbor.dirty {
							stack.push(pos);
							neighbor.dirty = false;
						}
					}
				}
			}
		}
	}
}
