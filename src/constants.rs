// Constants
pub const GRID_SIZE: usize = 4;
pub const GRID_DIRECTION_COUNT: usize = 4;

pub const DIRECTION_UP: usize = 0;
pub const DIRECTION_RIGHT: usize = 1;
pub const DIRECTION_DOWN: usize = 2;
pub const DIRECTION_LEFT: usize = 3;

#[macro_export]
macro_rules! BOUND_CHECK {
	($x:expr, $y:expr) => {
		($x >= 0) && ($x < GRID_SIZE as i32) && ($y >= 0) && ($y < GRID_SIZE as i32);
	};
}
