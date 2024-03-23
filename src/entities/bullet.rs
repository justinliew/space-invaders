//use rand::Rng;

use crate::{vector::Vector, swarm::Swarm};

#[derive(PartialEq)]
pub enum Ability {
	None,
	Bomb,
	Heat,
}

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
	pub active: bool,
	pub ability: Ability,
}

impl PlayerBullet {
	pub fn new() -> PlayerBullet {
		PlayerBullet {
			location: Vector::default(),
			speed: 0.,
			active: false,
			ability: Ability::None,
		}
	}

	pub fn respawn(&mut self, spawn_location: Vector, speed: f64, ability: Ability) {
		self.active = true;
		self.location = spawn_location;
		self.speed = speed;	
		self.ability = ability;
	}

	pub fn despawn(&mut self) {
		self.active = false;
	}

	pub fn x(&self) -> f64 { self.location.position.x }
	pub fn y(&self) -> f64 { self.location.position.y }


	// pub fn get_closest_swarm_location(swarm: &Swarm) -> Vector {
	// }

	pub fn update(&mut self, dt: f64, swarm: &Swarm) {
        self.location.position.x += self.location.direction.cos() * dt * self.speed;
        self.location.position.y += self.location.direction.sin() * dt * self.speed;
	}
}