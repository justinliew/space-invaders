use crate::point::Point;
use crate::size::Size;
use crate::bullet::Bullet;
use crate::state::GameData;

enum Movement {
	LEFT,
	DOWN(bool),
	RIGHT,
}

pub struct Swarm {
    pub top_left: Point,
	pub num_x: usize,
	pub num_y: usize,
	pub spacing_x: usize,
	pub spacing_y: usize,
	pub radius: usize,
	pub alive: Vec<bool>,
	num_alive: usize,
	movement: Movement,
	pub world_size: Size,
	time_to_move: f64,
}

/*
I think it starts in the middle
moves sideways a total of 10 from L to R
Speeds up as there are fewer and fewer enemies
 */
const MOVE_AMT: f64 = 20.0;
const BASE_MOVE_DELAY: f64 = 1.0;
const START_LOCATION: Point = Point{x: 200.0, y: 50.0};

impl Swarm {

	pub fn new(x: usize, y: usize, world_size: Size) -> Swarm {
		let mut ret = Swarm {
			top_left: START_LOCATION,
			num_x: x,
			num_y: y,
			spacing_x: 40,
			spacing_y: 40,
			radius: 20,
			alive: vec![true;x * y],
			num_alive: x*y,
			movement: Movement::LEFT,
			world_size: world_size,
			time_to_move: BASE_MOVE_DELAY,
		};
		ret
	}

	pub fn reset(&mut self) {
		self.top_left = START_LOCATION;
		self.alive = vec![true;self.num_x*self.num_y];
		self.num_alive = self.num_x * self.num_y;
		self.movement = Movement::LEFT;
		self.time_to_move = BASE_MOVE_DELAY;
	}

	pub fn update(&mut self, dt: f64) {
		let rhs = self.top_left.x as usize + self.num_x * self.spacing_x;
		self.time_to_move -= dt;
		if self.time_to_move > 0.0 {
			return;
		}
		self.time_to_move = BASE_MOVE_DELAY * (self.num_alive as f64 / (self.num_x * self.num_y) as f64);
		match self.movement {
			Movement::LEFT => {
				if rhs < self.world_size.width - self.top_left.x as usize {
					self.top_left.x += MOVE_AMT;
				} else {
					self.movement = Movement::DOWN(true);
				}
			},
			Movement::DOWN(left) => {
				self.top_left.y += MOVE_AMT;
				if left {
					self.movement = Movement::RIGHT;
				} else {
					self.movement = Movement::LEFT;
				}
			}
			Movement::RIGHT => {
				if self.top_left.x > 100. {
					self.top_left.x -= MOVE_AMT;
				} else {
					self.movement = Movement::DOWN(false);
				}
			}
		}
	}

	pub fn get_enemy_location_screen(&self, x: usize, y: usize, data: &GameData) -> Point {
		data.world_to_screen(&Point{
			x: (self.top_left.x + (self.radius as f64) + (x * (self.spacing_x + self.radius)) as f64),
			y: (self.top_left.y + (self.radius as f64) + (y * (self.spacing_y + self.radius)) as f64),
		})
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

	fn get_bottom_right(&self) -> Point {
		Point{
			x: self.top_left.x + (self.num_x * self.radius) as f64 + (self.num_x-1) as f64 * self.spacing_x as f64,
			y: self.top_left.y + (self.num_y * self.radius) as f64 + (self.num_y-1) as f64 * self.spacing_y as f64,
		}
	}


	pub fn check_hit(&mut self, bullet: &Bullet) -> bool {
		if bullet.x() < self.top_left.x || bullet.x() > self.get_bottom_right().x {
			return false;
		}
		if bullet.y() < self.top_left.y || bullet.y() > self.get_bottom_right().y {
			return false;
		}

		if let Some(hit) = self.is_hit(bullet.x(), bullet.y()) {
			if self.alive[hit.0 + (hit.1 as usize)*self.num_x] {
				self.alive[hit.0 + (hit.1 as usize)*self.num_x] = false;
				self.num_alive -= 1;
				return true;
			}
		}
		false
	}

	pub fn get_lowest_alive(&self) -> Option<f64> {
		for (index, alive) in self.alive.iter().enumerate().rev() {
			if *alive {
				let row = index / self.num_x;
				let y = self.top_left.y + (self.radius * (row+1)) as f64 + (self.spacing_y * row) as f64;
				return Some(y);
			}
		}
		None
	}
}
