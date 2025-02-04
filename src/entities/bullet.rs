//use rand::Rng;

use crate::vector::Vector;

pub struct Bullet {
    pub location: Vector,
	speed: f64,
}

impl Bullet {
	pub fn new(spawn_location: Vector, speed: f64) -> Bullet {
		Bullet {
			location: spawn_location,
			speed: speed,
		}
	}

	pub fn x(&self) -> f64 { self.location.position.x }
	pub fn y(&self) -> f64 { self.location.position.y }

	pub fn update(&mut self, dt: f64) {
        self.location.position.x += self.location.direction.cos() * dt * self.speed;
        self.location.position.y += self.location.direction.sin() * dt * self.speed;
	}
}

pub struct PlayerBullet {
    pub location: Vector,
	pub speed: f64,
	pub facing: f64,
	pub active: bool,
}

impl PlayerBullet {
	pub fn new() -> PlayerBullet {
		PlayerBullet {
			location: Vector::default(),
			speed: 0.,
			facing: 0.,
			active: false,
		}
	}

	pub fn respawn(&mut self, spawn_location: Vector, speed: f64) {
		self.active = true;
		self.location = spawn_location;
		self.speed = speed;	
	}

	pub fn despawn(&mut self) {
		self.active = false;
	}

	pub fn x(&self) -> f64 { self.location.position.x }
	pub fn y(&self) -> f64 { self.location.position.y }

	/*
	We need to calculate the angle difference between the bullet and the bullet-to-position
	Then we turn it slightly to that direction both in direction and facing
	 */
	pub fn update(&mut self, dt: f64) {
        self.location.position.x += self.location.direction.cos() * dt * self.speed;
        self.location.position.y += self.location.direction.sin() * dt * self.speed;
	}
}