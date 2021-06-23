
use crate::point::Point;
use crate::size::WorldSize;
use crate::bullet::{Bullet,BulletType};
use crate::vector::Vector;
use crate::game::{ResetType};

enum Movement {
	LEFT,
	DOWN(bool),
	RIGHT,
}

pub struct Swarm {
    pub top_left: Point,
	pub num_x: usize,
	pub num_y: usize,
	pub spacing_x: f64,
	pub spacing_y: f64,
	pub radius: f64,
	pub alive: Vec<bool>,
	num_alive: usize,
	movement: Movement,
	pub world_size: WorldSize,
	time_to_move: f64,
	pub fire_column: usize,
	pub frame: u32,
}

/*
I think it starts in the middle
moves sideways a total of 10 from L to R
Speeds up as there are fewer and fewer enemies
 */
const MOVE_AMT: f64 = 20.0;
const BASE_MOVE_DELAY: f64 = 1.0;
const START_LOCATION: Point = Point{x: 200.0, y: 60.0};
// I am having issues with rand packages on the wasm-unknown-unknown target
// so I am just using a hard coded list of columns that repeats
const FIRING_COLUMNS : &[usize;10] = &[4,5,3,2,6,8,1,0,7,9];

impl Swarm {

	pub fn new(x: usize, y: usize, world_size: WorldSize) -> Swarm {
		let ret = Swarm {
			top_left: START_LOCATION,
			num_x: x,
			num_y: y,
			spacing_x: 20.,
			spacing_y: 20.,
			radius: 36.,
			alive: vec![true;x * y],
			num_alive: x*y,
			movement: Movement::RIGHT,
			world_size: world_size,
			time_to_move: BASE_MOVE_DELAY,
			fire_column: 0,
			frame: 0,
		};
		ret
	}

	pub fn reset(&mut self, reset_type: ResetType) {
		if reset_type == ResetType::Next {
			self.top_left = START_LOCATION + Point::new(0.,10.);
		} else {
			self.top_left = START_LOCATION;
		}
		self.alive = vec![true;self.num_x*self.num_y];
		self.num_alive = self.num_x * self.num_y;
		self.movement = Movement::RIGHT;
		self.time_to_move = BASE_MOVE_DELAY;
		self.fire_column = 0;
	}

	pub fn update(&mut self, dt: f64) -> Option<Bullet> {


		self.time_to_move -= dt;
		if self.time_to_move > 0.0 {
			return None;
		}

		let mut lhs = self.top_left.x;
		let mut rhs = self.top_left.x + self.num_x as f64 * self.spacing_x;
		for l in 0..self.num_x {
			if !self.get_lowest_in_column(l).is_none() {
				lhs = self.top_left.x + (l as f64) * (self.radius + self.spacing_x);
				break;
			}
		}
		for r in (0..self.num_x).rev() {
			if !self.get_lowest_in_column(r).is_none() {
				rhs = self.top_left.x + (r as f64) * (self.radius + self.spacing_x);
				break;
			}
		}

		self.time_to_move = BASE_MOVE_DELAY * (self.num_alive as f64 / (self.num_x * self.num_y) as f64);
		match self.movement {
			Movement::RIGHT => {
				if self.world_size.width - rhs > 50. {
					self.top_left.x += MOVE_AMT;
				} else {
					self.movement = Movement::DOWN(true);
				}
			},
			Movement::DOWN(right) => {
				self.top_left.y += MOVE_AMT;
				if right {
					self.movement = Movement::LEFT;
				} else {
					self.movement = Movement::RIGHT;
				}
			}
			Movement::LEFT => {
				if lhs > 50. {
					self.top_left.x -= MOVE_AMT;
				} else {
					self.movement = Movement::DOWN(false);
				}
			}
		}

		match self.frame {
			0 => {
				self.frame = 1;
			},
			1 => {
				self.frame = 0;
			},
			_ => {},
		}

		if let Some(loc) = self.get_bullet_spawn_location() {
			return Some(Bullet::new(Vector::new(loc, std::f64::consts::PI / 2.0), BulletType::Swarm, 200.));
		} else {
			return None;
		}
	}

	pub fn get_enemy_location(&self, x: usize, y: usize) -> Point {
		Point{
			x: self.top_left.x + x as f64 * (self.radius + self.spacing_x),
			y: self.top_left.y + y as f64 * (self.radius + self.spacing_y),
		}
	}

	pub fn get_lowest_in_column(&self, col: usize) -> Option<usize> {
		// find an alive enemy
		let mut row = self.num_y - 1;

		while row > 0 {
			if self.alive[row * self.num_x + col] {
				return Some(row);
			}
			row -= 1;
		}
		None
	}

	pub fn is_hit(&self, x: f64, y: f64) -> Option<(usize,usize)> {
		let bucket_x = (x - self.top_left.x) / (self.radius + self.spacing_x);
		let bucket_y = (y - self.top_left.y) / (self.radius + self.spacing_y);
		let fract_in_x = self.radius as f64 / (self.radius + self.spacing_x);
		let fract_in_y = self.radius as f64 / (self.radius + self.spacing_y);

		if bucket_x.fract() > fract_in_x || bucket_y.fract() > fract_in_y {
			return None;
		}

		Some((bucket_x.trunc() as usize, bucket_y.trunc() as usize))
	}

	pub fn get_bottom_right(&self) -> Point {
		Point{
			x: self.top_left.x + self.num_x as f64 * self.radius + ((self.num_x-1) as f64) * self.spacing_x,
			y: self.top_left.y + self.num_y as f64 * self.radius + ((self.num_y-1) as f64) * self.spacing_y,
		}
	}

	fn get_bullet_spawn_location(&mut self) -> Option<Point> {
		// get the next column
		if self.fire_column >= self.num_x-1 {
			self.fire_column = 0;
		} else {
			self.fire_column += 1;
		}
		let col = FIRING_COLUMNS[self.fire_column];

		if let Some(row) = self.get_lowest_in_column(col) {
			return Some(self.get_enemy_location(col, row) + Point::new(self.radius as f64/2., self.radius as f64));
		}
		None
	}


	pub fn check_hit(&mut self, bullet: &Bullet) -> Option<(u32,Point)> {
		if bullet.bullet_type != BulletType::Player(true) {
			return None;
		}
		if bullet.x() < self.top_left.x || bullet.x() > self.get_bottom_right().x {
			return None;
		}
		if bullet.y() < self.top_left.y || bullet.y() > self.get_bottom_right().y {
			return None;
		}

		if let Some(hit) = self.is_hit(bullet.x(), bullet.y()) {
			if self.alive[hit.0 + hit.1 * self.num_x] {
				self.alive[hit.0 + hit.1 * self.num_x] = false;
				self.num_alive -= 1;
				let loc = self.get_enemy_location(hit.0,hit.1) + Point::new(self.radius / 2.,self.radius / 2.);
				return match hit.1 {
					0 => {
						Some((30,loc))
					}
					1|2 => {
						Some((20,loc))
					},
					3|4 => {
						Some((10,loc))
					},
					_ => {
						unreachable!()
					}
				}
			}
		}
		None
	}

	pub fn get_lowest_alive(&self) -> Option<f64> {
		for (index, alive) in self.alive.iter().enumerate().rev() {
			if *alive {
				let row = index / self.num_x;
				let y = self.top_left.y + self.radius * ((row+1) as f64) + self.spacing_y * (row as f64);
				return Some(y);
			}
		}
		None
	}
}
