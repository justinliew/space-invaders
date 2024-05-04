//use rand::Rng;

use crate::point::Point;
//use crate::{derive_position_direction, vector::{Vector}};
use crate::vector::Vector;
use crate::bullet::Bullet;

//use geometry::{Advance, Collide, Position};

#[derive(Default)]
pub struct Player {
    pub vector: Vector,
	pub alive: bool,
}

lazy_static! {
	static ref START_LOCATION: Vector = Vector::new(
		Point::new(500.0,750.0),
		-std::f64::consts::PI / 2.0,
	);
}


impl Player {

	pub fn new() -> Player {
		Player {
			vector: START_LOCATION.clone(),
			alive: true,
		}
	}

	pub fn reset_location(&mut self) {
		self.vector = START_LOCATION.clone();
	}

	pub fn x(&self) -> f64 { self.vector.position.x }
	pub fn x_mut(&mut self) -> &mut f64 { &mut self.vector.position.x }
	pub fn y(&self) -> f64 { self.vector.position.y }

	pub fn dir(&self) -> f64 { self.vector.direction }

	pub fn check_hit(&mut self, bullet: &Bullet) -> bool {
		// TODO player radius
		let hit = bullet.x() > self.x() - 20. && bullet.x() < self.x() + 20. &&
			bullet.y() > self.y();

		if hit {
			self.alive = false;
		}
		hit
	}
}
