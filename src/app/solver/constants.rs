// Constants
pub const GRID_SIZE: usize = 6;
pub const VARIATIONS: usize = 10;
pub const DIMENSIONS: usize = 4;

pub const DIRECTION_TO_VEC: [(i32, i32); 4] = [(0, -1), (-1, 0), (0, 1), (1, 0)];

pub const fn BOUND_CHECK(x: i32, y: i32) -> bool {
	(x >= 0) && (x < GRID_SIZE as i32) && (y >= 0) && (y < GRID_SIZE as i32)
}

pub const fn LEGAL_DIRECTION(index: usize, x: usize, y: usize) -> (bool, (usize, usize)) {
	let vec = DIRECTION_TO_VEC[index];
	let x = vec.0 + x as i32;
	let y = vec.1 + y as i32;
	return (BOUND_CHECK(x, y), (x as usize, y as usize));
}
