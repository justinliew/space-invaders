//use rand::Rng;

use crate::point::Point;
use crate::vector::Vector;

#[derive(PartialEq)]
pub enum BulletType {
	Player,
	Swarm,
}

pub struct Bullet {
    pub location: Vector,
	pub bullet_type: BulletType,
	speed: f64,
}

impl Bullet {

	pub fn new(spawn_location: Vector, bullet_type: BulletType, speed: f64) -> Bullet {
		Bullet {
			location: spawn_location,
			bullet_type: bullet_type,
			speed: speed,
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
        self.location.position.x += self.location.direction.cos() * dt * self.speed;
        self.location.position.y += self.location.direction.sin() * dt * self.speed;
	}
}

// impl Collide for Player {
//     fn radius(&self) -> f64 { 6.0 }
// }