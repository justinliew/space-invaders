use crate::player::{Player};
use crate::swarm::Swarm;
use crate::size::WorldSize;
use crate::bullet::{Bullet,BulletType};
use crate::point::Point;
use crate::particle::Particle;
use crate::shield::{BlockState,Shield};
use crate::vector::Vector;
use crate::ufo::Ufo;

pub struct World {
    pub player: Player,
	pub swarm: Swarm,
	pub world_size: WorldSize,
	pub player_bullet: Bullet,
	pub bullets: Vec<Bullet>,
    pub particles: Vec<Particle>,
	pub shields: Vec<Shield>,
	pub ufo: Ufo,
}

impl World {
    /// Returns a new world of the given size
    pub fn new(world_size: WorldSize) -> World {
        World {
            player: Player::new(),
			swarm: Swarm::new(10,5, world_size),
			world_size: world_size,
			player_bullet: Bullet::new(Vector::default(), BulletType::Player(false), 0.),
			bullets: vec![],
            particles: Vec::with_capacity(1000),
			shields: vec![
				Shield::new(Point::new(150.,550.,),
				[BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Full,
				BlockState::Full,BlockState::Empty,BlockState::Empty,BlockState::Empty,BlockState::Empty,
				BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Full,
				BlockState::Full,BlockState::Empty,BlockState::Empty,BlockState::Empty,BlockState::Empty,
				BlockState::Full,BlockState::Empty,BlockState::Empty,BlockState::Empty,BlockState::Empty]),
				Shield::new(Point::new(350.,550.,),
				[BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Full,
				BlockState::Full,BlockState::Empty,BlockState::Empty,BlockState::Empty,BlockState::Empty,
				BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Full,
				BlockState::Empty,BlockState::Empty,BlockState::Empty,BlockState::Empty,BlockState::Full,
				BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Full]),
				Shield::new(Point::new(550.,550.,),
				[BlockState::Full,BlockState::Empty,BlockState::Empty,BlockState::Empty,BlockState::Empty,
				BlockState::Full,BlockState::Empty,BlockState::Empty,BlockState::Empty,BlockState::Empty,
				BlockState::Full,BlockState::Empty,BlockState::Empty,BlockState::Empty,BlockState::Empty,
				BlockState::Full,BlockState::Empty,BlockState::Empty,BlockState::Empty,BlockState::Empty,
				BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Full]),
				Shield::new(Point::new(750.,550.,),
				[BlockState::Full,BlockState::Empty,BlockState::Empty,BlockState::Empty,BlockState::Full,
				BlockState::Full,BlockState::Empty,BlockState::Empty,BlockState::Empty,BlockState::Full,
				BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Full,
				BlockState::Empty,BlockState::Empty,BlockState::Full,BlockState::Empty,BlockState::Empty,
				BlockState::Empty,BlockState::Empty,BlockState::Full,BlockState::Empty,BlockState::Empty]),
			],
			ufo: Ufo::new(world_size),
        }
    }
}
