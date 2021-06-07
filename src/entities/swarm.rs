use crate::point::Point;
use crate::size::Size;
use crate::bullet::Bullet;

enum Movement {
	LEFT,
	DOWN(bool),
	RIGHT,
}

pub struct Swarm {
    pub top_left: Point,
	pub bottom_right: Point,
	pub num_x: usize,
	pub num_y: usize,
	pub spacing_x: usize,
	pub spacing_y: usize,
	pub radius: usize,
	pub alive: Vec<bool>,
	movement: Movement,
	pub world_size: Size,
}

const BASE_MOVE_SPEED: f64 = 50.0;

impl Swarm {

	pub fn new(x: usize, y: usize, world_size: Size) -> Swarm {
		let mut ret = Swarm {
			top_left: Point::new(100.0,100.0),
			bottom_right: Point::default(),
			num_x: x,
			num_y: y,
			spacing_x: 30,
			spacing_y: 30,
			radius: 20,
			alive: vec![true;x * y],
			movement: Movement::LEFT,
			world_size: world_size,
		};
		ret.bottom_right.x = ret.top_left.x + (ret.num_x * ret.radius) as f64 + (ret.num_x-1) as f64 * ret.spacing_x as f64;
		ret.bottom_right.y = ret.top_left.y + (ret.num_y * ret.radius) as f64 + (ret.num_y-1) as f64 * ret.spacing_y as f64;
		ret
	}

	pub fn update(&mut self, dt: f64) {
		let rhs = self.top_left.x as usize + self.num_x * self.spacing_x;
		match self.movement {
			Movement::LEFT => {
				if rhs < self.world_size.width - self.top_left.x as usize {
					self.top_left.x += BASE_MOVE_SPEED * dt;
				} else {
					self.movement = Movement::DOWN(true);
				}
			},
			Movement::DOWN(left) => {
				self.top_left.y += 30.;
				if left {
					self.top_left.x += BASE_MOVE_SPEED * dt;
					self.movement = Movement::RIGHT;
				} else {
					self.top_left.x -= BASE_MOVE_SPEED * dt;
					self.movement = Movement::LEFT;
				}
			}
			Movement::RIGHT => {
				if self.top_left.x > 100. {
					self.top_left.x -= BASE_MOVE_SPEED * dt;
				} else {
					self.movement = Movement::DOWN(false);
				}
			}
		}
	}

	pub fn get_enemy_location(&self, x: usize, y: usize, world_to_screen: (f64,f64)) -> Point {
		Point{
			x: (self.top_left.x + (self.radius as f64) + (x * (self.spacing_x + self.radius)) as f64) * world_to_screen.0,
			y: (self.top_left.y + (self.radius as f64) + (y * (self.spacing_y + self.radius)) as f64) * world_to_screen.1,
		}
	}

	pub fn is_hit(&self, x: f64, y: f64) -> Option<(usize,usize)> {
		let bucket_x = (x - self.top_left.x) / (self.radius + self.spacing_x) as f64;
		let bucket_y = (y - self.top_left.y) / (self.radius + self.spacing_y) as f64;
		let fract_in_x = self.radius as f64 / (self.radius + self.spacing_x) as f64;
		let fract_in_y = self.radius as f64 / (self.radius + self.spacing_y) as f64;

		if bucket_x.fract() > fract_in_x || bucket_y.fract() > fract_in_y {
			return None;
		}

		Some((bucket_x.trunc() as usize, bucket_y.trunc() as usize))
	}

	pub fn check_hit(&mut self, bullet: &Bullet) -> bool {
		if bullet.x() < self.top_left.x || bullet.x() > self.bottom_right.x {
			return false;
		}
		if bullet.y() < self.top_left.y || bullet.y() > self.bottom_right.y {
			return false;
		}

		if let Some(hit) = self.is_hit(bullet.x(), bullet.y()) {
			self.alive[hit.0 + (hit.1 as usize)*self.num_x] = false;
		}
		true
	}
}
