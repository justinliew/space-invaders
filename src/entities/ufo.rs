use crate::point::Point;
use crate::bullet::PlayerBullet;
use crate::size::WorldSize;
use crate::game::ResetType;

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
	level: i32,
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
			level: 0,
		}
	}

	pub fn x(&self) -> f64 { self.position.x }
	pub fn y(&self) -> f64 { self.position.y }

	pub fn reset(&mut self, reset_type: ResetType) {
		if reset_type == ResetType::Next {
			self.level += 1;
		}
		self.active = false;
		self.spawn_cooldown = Ufo::SPAWN_COOLDOWN;
	}

	pub fn update(&mut self, dt: f64) {
		if self.active {
			match self.start {
				StartLocation::LEFT => {
					self.position.x += Ufo::UFO_SPEED;
					if self.position.x > self.world_size.width {
						self.reset(ResetType::Respawn);
					}
				},
				StartLocation::RIGHT => {
					self.position.x -= Ufo::UFO_SPEED;
					if self.position.x < 0. {
						self.reset(ResetType::Respawn);
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
						self.position.y = 20.;
					},
					StartLocation::RIGHT => {
						self.start = StartLocation::LEFT;
						self.position.x = 0.;
						self.position.y = 20.;
					}
				}
			}
		}
	}

	pub fn check_hit(&mut self, bullet: &PlayerBullet) -> Option<(i32,Point)> {
		if !bullet.active {
			return None;
		}

		if !self.active {
			return None;
		}

		let hit = bullet.x() > self.x() && bullet.x() < self.x() + Ufo::UFO_WIDTH*1.5 &&
			bullet.y() < self.y() + Ufo::UFO_HEIGHT;
		if hit {
			self.reset(ResetType::Respawn);
			let multiplier = f32::max(1., 1.5 * self.level as f32);
			Some(((300.*multiplier) as i32, bullet.location.position))
		} else {
			None
		}
	}
}
