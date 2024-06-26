
use crate::point::Point;
use crate::size::WorldSize;
use crate::bullet::{Bullet,PlayerBullet, Ability};
use crate::vector::Vector;
use crate::game::{ResetType,GameEvent,ColourIndex};

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
	pub num_drops: usize,
	pub level: usize,
	pub lhs: f64,
	pub rhs: f64,
}

/*
I think it starts in the middle
moves sideways a total of 10 from L to R
Speeds up as there are fewer and fewer enemies
 */
const MOVE_AMT: f64 = 20.0;
const BASE_MOVE_DELAY: f64 = 1.;
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
			num_drops: 0,
			level: 0,
			lhs: 0.,
			rhs: 0.,
		};
		ret
	}

	pub fn reset(&mut self, reset_type: ResetType) {
		if reset_type == ResetType::Next {
			self.level += 1;
			self.top_left = START_LOCATION + Point::new(0.,10.);
		} else if reset_type == ResetType::Respawn {
			return;			
		} else {
			self.top_left = START_LOCATION;
		}
		self.alive = vec![true;self.num_x*self.num_y];
		self.num_alive = self.num_x * self.num_y;
		self.movement = Movement::RIGHT;
		self.time_to_move = self.get_level_modifier();
		self.fire_column = 0;
		self.num_drops = 0;
	}

	pub fn get_percentage_alive(&self) -> f64 {
		self.num_alive as f64 / (self.num_x * self.num_y) as f64
	}

	pub fn get_num_drops(&self) -> usize {
		self.num_drops
	}

	pub fn get_level_modifier(&self) -> f64 {
		f64::max(BASE_MOVE_DELAY - 0.12 * self.level as f64,0.2)
	}

	pub fn update(&mut self, dt: f64) -> Option<Vec<Bullet>> {

		let mut lhs = self.top_left.x;
		for l in 0..self.num_x {
			if self.get_lowest_in_column(l).is_some() {
				lhs = self.top_left.x + (l as f64) * (self.radius + self.spacing_x);
				break;
			}
		}
		let mut rhs = self.top_left.x + self.num_x as f64 * (self.radius + self.spacing_x) + self.radius;
		for r in (0..self.num_x).rev() {
			if self.get_lowest_in_column(r).is_some() {
				rhs = self.top_left.x + (r as f64) * (self.radius + self.spacing_x) + self.radius;
				break;
			}
		}

		self.lhs = lhs;
		self.rhs = rhs;

		self.time_to_move -= dt;
		if self.time_to_move > 0.0 {
			return None;
		}

		self.time_to_move = self.get_level_modifier() * self.get_percentage_alive();
		match self.movement {
			Movement::RIGHT => {
				if self.world_size.width - rhs > 75. {
					self.top_left.x += MOVE_AMT;
				} else {
					self.movement = Movement::DOWN(true);
				}
			},
			Movement::DOWN(right) => {
				self.top_left.y += MOVE_AMT;
				self.num_drops += 1;
				if right {
					self.movement = Movement::LEFT;
				} else {
					self.movement = Movement::RIGHT;
				}
			}
			Movement::LEFT => {
				if lhs > 30. {
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

		let mut ret = vec![];

		for i in 0..4 {
			if let Some(loc) = self.get_bullet_spawn_location() {
				ret.push(Bullet::new(Vector::new(loc, std::f64::consts::PI / 2.0), 200.));
			}
		}
		if ret.is_empty() {
			return None;
		} else {
			return Some(ret);
		}
	}

	// percentage will go from 0.0 to ~1.0. We should make sure that many are dead
	pub fn force_kill(&mut self, percentage: f64) -> Vec<GameEvent> {
		let mut queued_events = vec![];
		while self.get_percentage_alive() > 1.0-percentage && self.get_percentage_alive() > 0. {
			let mut kill_index = None;
			for (index,alive) in self.alive.iter().enumerate().rev() {
				if *alive {
					kill_index = Some(index);
					break;
				}
			}
			if let Some(index) = kill_index {
				self.alive[index] = false;
				self.num_alive -= 1;
				let x = index % self.num_x;
				let y = index / self.num_x;
				let loc = self.get_enemy_location(x,y) + Point::new(self.radius / 2.,self.radius / 2.);
				queued_events.push(GameEvent::EntityDied(loc, ColourIndex::WHITE));
			}
		}
		queued_events
	}

	pub fn get_enemy_location(&self, x: usize, y: usize) -> Point {
		Point{
			x: self.top_left.x + x as f64 * (self.radius + self.spacing_x),
			y: self.top_left.y + y as f64 * (self.radius + self.spacing_y),
		}
	}

	pub fn get_lowest_in_column(&self, col: usize) -> Option<usize> {
		// find an alive enemy
		let mut row = self.num_y;

		while row > 0 {
			if self.alive[(row-1) * self.num_x + col] {
				return Some(row-1);
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

	fn get_closest(&self, x: f64, y: f64) -> Option<(usize,usize)> {
		let bucket_x = (x - self.top_left.x) / (self.radius + self.spacing_x);
		let bucket_y = (y - self.top_left.y) / (self.radius + self.spacing_y);
		Some((bucket_x.trunc() as usize, bucket_y.trunc() as usize))
	}

	pub fn get_bottom_right(&self) -> Point {
		Point{
			x: self.top_left.x + (self.num_x as f64) * self.radius + ((self.num_x-1) as f64) * self.spacing_x,
			y: self.top_left.y + (self.num_y as f64) * self.radius + ((self.num_y-1) as f64) * self.spacing_y,
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


	pub fn check_hit(&mut self, bullet: &PlayerBullet) -> Option<Vec<(i32,Point)>> {
		if !bullet.active {
			return None;
		}
		if bullet.x() < self.top_left.x || bullet.x() > self.get_bottom_right().x {
			return None;
		}
		if bullet.y() < self.top_left.y || bullet.y() > self.get_bottom_right().y {
			return None;
		}

		if bullet.ability == Ability::Bomb {
			// I think this should just destroy things in a path as it goes up the screen
			let hit = self.get_closest(bullet.x(),bullet.y());
			if let Some(hit) = hit {
				let x_index = hit.0;
				if self.alive[x_index + hit.1 * self.num_x] {
					self.alive[x_index + hit.1 * self.num_x] = false;
					self.num_alive -= 1;
					let loc = self.get_enemy_location(x_index,hit.1) + Point::new(self.radius / 2.,self.radius / 2.);
					return Some(vec![(30,loc)]);
				}
				return None;
			} else {
				return None;
			}
		} else {
			if let Some(hit) = self.is_hit(bullet.x(), bullet.y()) {
				if self.alive[hit.0 + hit.1 * self.num_x] {
					self.alive[hit.0 + hit.1 * self.num_x] = false;
					self.num_alive -= 1;
					let loc = self.get_enemy_location(hit.0,hit.1) + Point::new(self.radius / 2.,self.radius / 2.);
					let multiplier = f32::max(1., 1.5 * self.level as f32);
					return match hit.1 {
						0 => {
							Some(vec![((30.*multiplier) as i32,loc)])
						}
						1|2 => {
							Some(vec![((20.*multiplier) as i32,loc)])
						},
						3|4 => {
							Some(vec![((10.*multiplier) as i32,loc)])
						},
						_ => {
							unreachable!()
						}
					}
				}
			}
			None
		}
	}

	pub fn get_lowest_alive(&self) -> Option<Point> {
		for (index, alive) in self.alive.iter().enumerate().rev() {
			if *alive {
				let col = index % self.num_x;
				let x = self.top_left.x + (self.radius + self.spacing_x) * ((col+1) as f64) - self.spacing_x - self.radius/2.;

				let row = index / self.num_x;
				let y = self.top_left.y + self.radius * ((row+1) as f64) + self.spacing_y * (row as f64);
				return Some(Point::new(x,y));
			}
		}
		None
	}


}
