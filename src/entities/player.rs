//use rand::Rng;

use crate::point::Point;
//use crate::{derive_position_direction, vector::{Vector}};
use crate::vector::Vector;
use crate::bullet::Bullet;

//use geometry::{Advance, Collide, Position};

pub struct Player {
    pub vector: Vector,
	pub collision: [i32; 121],
	pub alive: bool,
}

lazy_static! {
	static ref START_LOCATION: Vector = Vector::new(
		Point::new(500.0,750.0),
		-std::f64::consts::PI / 2.0,
	);
}

impl Player {

	pub fn new(collision: [i32; 121]) -> Player {
		Player {
			vector: START_LOCATION.clone(),
			collision: collision,
			alive: true,
		}
	}

	pub fn reset_location(&mut self) {
		self.vector = START_LOCATION.clone();
	}

	pub fn x(&self) -> f64 { self.vector.position.x }
	pub fn x_mut(&mut self) -> &mut f64 { &mut self.vector.position.x }
	pub fn y(&self) -> f64 { self.vector.position.y }
	pub fn y_mut(&mut self) -> &mut f64 { &mut self.vector.position.y }

	pub fn dir(&self) -> f64 { self.vector.direction }

	pub fn check_hit(&mut self, bullet: &Bullet) -> bool {
		let within = bullet.x() > self.x() - 16. && bullet.x() < self.x() + 17. &&
			bullet.y() > self.y() && bullet.y() < self.y() + 33.;
		
		if !within {
			return false;
		}

		// truncate, since the collision map scale is /3
		let offsetx = ((bullet.x()- self.x() + 16.) / 3.) as usize;
		let offsety = ((bullet.y() - self.y()) / 3.) as usize;

		let hit = self.collision[offsetx + 11 * offsety] != 0;

		if hit {
			self.alive = false;
		}
		hit
	}
}
