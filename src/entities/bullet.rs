//use rand::Rng;

use crate::point::Point;
use crate::vector::Vector;

#[derive(Default)]
pub struct Bullet {
    pub location: Vector
}

const BULLET_SPEED: f64 = 200.0;

impl Bullet {

	pub fn new(spawn_location: Vector) -> Bullet {
		Bullet {
			location: spawn_location,
		}
	}

	// TODO derive_position_direction
	pub fn x(&self) -> f64 { self.location.position.x }
	pub fn x_mut(&mut self) -> &mut f64 { &mut self.location.position.x }
	pub fn y(&self) -> f64 { self.location.position.y }
	pub fn y_mut(&mut self) -> &mut f64 { &mut self.location.position.y }

	pub fn dir(&self) -> f64 { self.location.direction }

    // Returns the front of the rocket
    // pub fn front(&self) -> Point {
    //     Point::new(POLYGON[1][0], POLYGON[1][1])
    //         .rotate(self.direction())
    //         .translate(&self.position())
    // }

	pub fn update(&mut self, dt: f64) {
        self.location.position.x += self.location.direction.cos() * dt * BULLET_SPEED;
        self.location.position.y += self.location.direction.sin() * dt * BULLET_SPEED;
	}
}

// impl Collide for Player {
//     fn radius(&self) -> f64 { 6.0 }
// }