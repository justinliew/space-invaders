//use rand::Rng;

use crate::vector::Vector;

type IsBomb = bool;
type IsHeatSeeking = bool;
type BulletActive = bool;

#[derive(PartialEq)]
pub enum BulletType {
	Player(BulletActive, IsBomb, IsHeatSeeking),
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

	pub fn inplace_new(&mut self, spawn_location: Vector, bullet_type: BulletType, speed: f64) {
		self.location = spawn_location;
		self.bullet_type = bullet_type;
		self.speed = speed;	
	}

	pub fn x(&self) -> f64 { self.location.position.x }
	pub fn y(&self) -> f64 { self.location.position.y }

	pub fn update(&mut self, dt: f64) {
        self.location.position.x += self.location.direction.cos() * dt * self.speed;
        self.location.position.y += self.location.direction.sin() * dt * self.speed;
	}
}
