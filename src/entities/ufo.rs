use crate::point::Point;
use crate::bullet::{Bullet,BulletType};
use crate::size::WorldSize;

enum StartLocation {
	LEFT,
	RIGHT,
}

pub struct Ufo {
    pub position: Point,
	pub spawn_cooldown: f64,
	pub active: bool,
	start: StartLocation,
	world_size: WorldSize,
}

impl Ufo {
	pub const UFO_WIDTH: f64 = 32.;
	pub const UFO_HEIGHT: f64 = 24.;
	pub const SPAWN_COOLDOWN: f64 = 5.;
	pub const UFO_SPEED: f64 = 3.;

	pub fn new(world_size: WorldSize) -> Ufo {
		Ufo {
			position: Point::default(),
			spawn_cooldown: Ufo::SPAWN_COOLDOWN,
			active: false,
			start: StartLocation::LEFT,
			world_size: world_size,
		}
	}

	pub fn x(&self) -> f64 { self.position.x }
	pub fn y(&self) -> f64 { self.position.y }

	fn reset(&mut self) {
		self.active = false;
		self.spawn_cooldown = Ufo::SPAWN_COOLDOWN;
	}

	pub fn update(&mut self, dt: f64) {
		if self.active {
			match self.start {
				StartLocation::LEFT => {
					self.position.x += Ufo::UFO_SPEED;
					if self.position.x > self.world_size.width {
						self.reset();
					}
				},
				StartLocation::RIGHT => {
					self.position.x -= Ufo::UFO_SPEED;
					if self.position.x < 0. {
						self.reset();
					}
				}
			}
		} else {
			self.spawn_cooldown -= dt;
			if self.spawn_cooldown < 0. {
				self.active = true;
				match self.start {
					StartLocation::LEFT => {
						self.start = StartLocation::RIGHT;
						self.position.x = self.world_size.width;
						self.position.y = 10.;
					},
					StartLocation::RIGHT => {
						self.start = StartLocation::LEFT;
						self.position.x = 0.;
						self.position.y = 10.;
					}
				}
			}
		}
	}

	pub fn check_hit(&mut self, bullet: &Bullet) -> Option<(i32,Point)> {
		if bullet.bullet_type != BulletType::Player(true) {
			return None;
		}

		let hit = bullet.x() > self.x() - Ufo::UFO_WIDTH/2. && bullet.x() < self.x() + Ufo::UFO_WIDTH/2. &&
			bullet.y() > self.y() - Ufo::UFO_HEIGHT/2. && bullet.y() < self.y() + Ufo::UFO_HEIGHT/2.;
		if hit {
			self.reset();
			Some((300, bullet.location.position))
		} else {
			None
		}
	}
}
