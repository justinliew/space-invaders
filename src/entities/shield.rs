use crate::point::Point;

#[derive(Copy, Clone)]
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
//	pub blocks: Vec<BlockState>,
	pub b: [[BlockState; 5]; 5],
}

impl Shield {
	pub fn new(top_left: Point, state: [[BlockState; 5]; 5]) -> Self {
		Shield {
			top_left: top_left,
			b: state,
		}
	}
}