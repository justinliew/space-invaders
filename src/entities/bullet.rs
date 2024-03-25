//use rand::Rng;

use crate::{vector::Vector, swarm::Swarm, point::Point};

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
	pub facing: f64,
	pub active: bool,
	pub ability: Ability,
}

impl PlayerBullet {
	pub fn new() -> PlayerBullet {
		PlayerBullet {
			location: Vector::default(),
			speed: 0.,
			facing: 0.,
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

	fn dot(&self, target: Point) -> f64 {
		let bullet_dir = self.location.direction;

		let target_heading = Point::new(target.x - self.location.position.x, target.y - self.location.position.y);
		let target_heading = target_heading / target_heading.mag();
		let dot = bullet_dir.cos() * target_heading.x + bullet_dir.sin() * target_heading.y;
		dot.clamp(-1., 1.)
	}

	/*
	We need to calculate the angle difference between the bullet and the bullet-to-position
	Then we turn it slightly to that direction both in direction and facing
	 */
	pub fn update(&mut self, dt: f64, swarm: &Swarm) {

		if self.ability == Ability::Heat {
			let target = swarm.get_lowest_alive().expect("alive");
			let dot = self.dot(target);
			let delta = self.location.position.x - target.x;
			let signed_angle = if delta.abs() < 10. {
				0.
			} else if delta > 0. {
				-dot.acos()
			} else {
				dot.acos()
			};
			self.location.direction = self.location.direction + signed_angle * dt * 10.;
			self.facing = self.location.direction + std::f64::consts::PI / 2.;
		}
        self.location.position.x += self.location.direction.cos() * dt * self.speed;
        self.location.position.y += self.location.direction.sin() * dt * self.speed;
	}
}