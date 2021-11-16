use crate::app::{
	solver::constants::{DIMENSIONS, GRID_SIZE, LEGAL_DIRECTION},
	solver::possibility::Possibility,
	solver::possibility_space::PossibilitySpace,
};
use std::collections::HashMap;

struct ImageSocketPossibilities {
	variations: u32,
	possibilities: HashMap<u8, Possibility>,
}

impl ImageSocketPossibilities {
	pub fn new(image: &image::Image, slices: u32) -> Self {
		Self {
			variations,
			possibilities: calculate_possibilities(image, slices),
		}
	}
}

fn calculate_possibilities(image: &image::Image, slices: u32) -> HashMap<u8, Possibility> {
	todo!();
}

impl PossibilitySpace for ImageSocketPossibilities {
	fn constraints(&self, super_position: u128, direction: usize) -> u128 {
		todo!()
	}

	fn print(&self) -> String {
		todo!()
	}
}
