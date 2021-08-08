use crate::point::Point;
use crate::bullet::Bullet;

#[derive(Copy, Clone, PartialEq)]
pub enum BlockState {
	Empty=0,
	Hit=1,
	Full=2,
}

/*
Shields are a 5x5 series of blocks that are either filled in or not
There is a top left, and block dimension
Each block can be hit twice before disappearing
 */
pub struct Shield {
	pub top_left: Point,
	pub b: [BlockState; 25],
	pub def: [BlockState; 25],
}

impl Shield {
	pub const BLOCK_DIM : f64 = 20.;
	pub fn new(top_left: Point, block_state: [BlockState; 25]) -> Self {
		Shield {
			top_left: top_left,
			b: block_state,
			def: block_state,
		}
	}

	pub fn reset(&mut self) {
		self.b = self.def;
	}

	fn get_indices(&self, p: &Point) -> Option<(usize,usize)> {
		if p.x < self.top_left.x || p.x >= self.top_left.x + 5.*Shield::BLOCK_DIM {
			return None;
		}
		if p.y < self.top_left.y || p.y >= self.top_left.y + 5.*Shield::BLOCK_DIM {
			return None;
		}
		Some((((p.x - self.top_left.x) / Shield::BLOCK_DIM).trunc() as usize,
		 ((p.y - self.top_left.y) / Shield::BLOCK_DIM).trunc() as usize))
	}

	pub fn check_hit(&self, bullet: &Bullet) -> Option<(usize,usize)> {
		match self.get_indices(&bullet.location.position) {
			Some((i,j)) => {
				if self.b[i+j*5] != BlockState::Empty {
					return Some((i,j))
				} else {
					return None
				}
			},
			_ => {
				None
			}
		}
	}

	pub fn damage(&mut self, i: usize, j: usize) -> BlockState {
		match self.b[i+j*5] {
			BlockState::Hit => {
				self.b[i+j*5] = BlockState::Empty;
			},
			BlockState::Full => {
				self.b[i+j*5] = BlockState::Hit;
			},
			_ => {
//				unreachable!();
			}
		}
		self.b[i+j*5]
	}
}