//use rand::Rng;

use crate::point::Point;
//use crate::{derive_position_direction, vector::{Vector}};
use crate::vector::Vector;

//use geometry::{Advance, Collide, Position};

#[derive(Default)]
pub struct Player {
    pub vector: Vector
}

/// The player is represented as the polygon below
pub const POLYGON: &'static [[f64; 2]] = &[
    [0.0, -8.0],
    [20.0, 0.0],
    [0.0, 8.0]
];

lazy_static! {
	static ref START_LOCATION: Vector = Vector::new(
		Point::new(500.0,500.0),
		-std::f64::consts::PI / 2.0,
	);
}


impl Player {

	pub fn new() -> Player {
		Player {
			vector: START_LOCATION.clone(),
		}
	}

	// TODO derive_position_direction
	pub fn x(&self) -> f64 { self.vector.position.x }
	pub fn x_mut(&mut self) -> &mut f64 { &mut self.vector.position.x }
	pub fn y(&self) -> f64 { self.vector.position.y }
	pub fn y_mut(&mut self) -> &mut f64 { &mut self.vector.position.y }

	pub fn dir(&self) -> f64 { self.vector.direction }

    // Returns the front of the rocket
    // pub fn front(&self) -> Point {
    //     Point::new(POLYGON[1][0], POLYGON[1][1])
    //         .rotate(self.direction())
    //         .translate(&self.position())
    // }
}

// impl Collide for Player {
//     fn radius(&self) -> f64 { 6.0 }
// }