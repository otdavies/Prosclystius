pub trait PossibilitySpace {
	fn constraints(&self, super_position: u128, direction: usize) -> u128;
	fn print(&self) -> String;
}
