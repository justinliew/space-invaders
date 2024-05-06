use crate::game::ResetType;
use crate::player::Player;
use crate::swarm::Swarm;
use crate::size::WorldSize;
use crate::bullet::{Bullet,PlayerBullet};
use crate::point::Point;
use crate::shield::{BlockState,Shield};
use crate::ufo::Ufo;
use std::os::raw::c_int;

extern "C" {
	fn init_shield(_: c_int);
	fn add_shield_state(_: c_int, _: c_int, _: c_int);
}

pub struct World {
	pub world_size: WorldSize,
    player: Player,
	swarm: Swarm,
	player_bullet: PlayerBullet,
	bullets: Vec<Bullet>,
	using_fastly_shields: bool,
	shields: Vec<Shield>,
	fastly_shields: Vec<Shield>,
	ufo: Ufo,
}

impl World {
    /// Returns a new world of the given size
    pub fn new(world_size: WorldSize) -> World {
        World {
			world_size: world_size,
            player: Player::new([0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0,0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0,0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0,0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0,0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0,0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0,0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0,0, 0, 1, 0, 1, 1, 1, 0, 1, 0, 0,0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0,1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1,1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,]),
			swarm: Swarm::new(10,5, world_size),
			player_bullet: PlayerBullet::new(),
			bullets: vec![],
			using_fastly_shields: false,
			shields: vec![
				Shield::new(Point::new(150.,500.,),
				[BlockState::Empty,BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Empty,
				BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Full,
				BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Full,
				BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Full,
				BlockState::Empty,BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Empty]),
				Shield::new(Point::new(400.,500.,),
				[BlockState::Empty,BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Empty,
				BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Full,
				BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Full,
				BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Full,
				BlockState::Empty,BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Empty]),
				Shield::new(Point::new(650.,500.,),
				[BlockState::Empty,BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Empty,
				BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Full,
				BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Full,
				BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Full,
				BlockState::Empty,BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Empty]),
			],			
			fastly_shields: vec![
				Shield::new(Point::new(150.,550.,),
				[BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Empty,
				BlockState::Full,BlockState::Empty,BlockState::Empty,BlockState::Empty,BlockState::Empty,
				BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Empty,
				BlockState::Full,BlockState::Empty,BlockState::Empty,BlockState::Empty,BlockState::Empty,
				BlockState::Full,BlockState::Empty,BlockState::Empty,BlockState::Empty,BlockState::Empty]),
				Shield::new(Point::new(300.,550.,),
				[BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Empty,
				BlockState::Full,BlockState::Empty,BlockState::Empty,BlockState::Full,BlockState::Empty,
				BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Empty,
				BlockState::Full,BlockState::Empty,BlockState::Empty,BlockState::Full,BlockState::Empty,
				BlockState::Full,BlockState::Empty,BlockState::Empty,BlockState::Full,BlockState::Empty]),
				Shield::new(Point::new(450.,550.,),
				[BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Empty,
				BlockState::Full,BlockState::Empty,BlockState::Empty,BlockState::Empty,BlockState::Empty,
				BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Empty,
				BlockState::Empty,BlockState::Empty,BlockState::Empty,BlockState::Full,BlockState::Empty,
				BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Empty]),
				Shield::new(Point::new(600.,550.,),
				[BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Full,
				BlockState::Empty,BlockState::Empty,BlockState::Full,BlockState::Empty,BlockState::Empty,
				BlockState::Empty,BlockState::Empty,BlockState::Full,BlockState::Empty,BlockState::Empty,
				BlockState::Empty,BlockState::Empty,BlockState::Full,BlockState::Empty,BlockState::Empty,
				BlockState::Empty,BlockState::Empty,BlockState::Full,BlockState::Empty,BlockState::Empty]),
				Shield::new(Point::new(750.,550.,),
				[BlockState::Full,BlockState::Empty,BlockState::Empty,BlockState::Empty,BlockState::Empty,
				BlockState::Full,BlockState::Empty,BlockState::Empty,BlockState::Empty,BlockState::Empty,
				BlockState::Full,BlockState::Empty,BlockState::Empty,BlockState::Empty,BlockState::Empty,
				BlockState::Full,BlockState::Empty,BlockState::Empty,BlockState::Empty,BlockState::Empty,
				BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Empty]),
				Shield::new(Point::new(900.,550.,),
				[BlockState::Full,BlockState::Empty,BlockState::Empty,BlockState::Empty,BlockState::Full,
				BlockState::Full,BlockState::Empty,BlockState::Empty,BlockState::Empty,BlockState::Full,
				BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Full,BlockState::Full,
				BlockState::Empty,BlockState::Empty,BlockState::Full,BlockState::Empty,BlockState::Empty,
				BlockState::Empty,BlockState::Empty,BlockState::Full,BlockState::Empty,BlockState::Empty]),
			],
			ufo: Ufo::new(world_size),
        }
    }

	pub fn reset(&mut self, reset_type: ResetType) {
        self.bullets.clear();
		self.swarm.reset(reset_type);
		self.player.alive = true;
		self.player.reset_location();
		if reset_type == ResetType::New {
			self.using_fastly_shields = false;
		}
	}

	pub fn init_shields(&mut self) {
		for (index,shield) in self.get_active_shields_mut().iter_mut().enumerate() {
			shield.reset();
			unsafe {init_shield(index as i32)};
			for i in 0..25 {
				let state = &shield.b[i];
				unsafe {add_shield_state(index as i32, i as i32, *state as i32)};
			}
		}
	}

	pub fn enable_fastly_shields(&mut self) {
		self.using_fastly_shields = true;
		self.init_shields();
	}

	pub fn get_active_shields(&self) -> &Vec<Shield> {
		match self.using_fastly_shields {
			true => &self.fastly_shields,
			false => &self.shields,
		}
	}

	pub fn get_active_shields_mut(&mut self) -> &mut Vec<Shield> {
		match self.using_fastly_shields {
			true => &mut self.fastly_shields,
			false => &mut self.shields,
		}
	}

	pub fn get_bullets(&self) -> &Vec<Bullet> {
		&self.bullets
	}

	pub fn get_bullets_mut(&mut self) -> &mut Vec<Bullet> {
		&mut self.bullets
	}

	pub fn get_player_bullet(&self) -> &PlayerBullet {
		&self.player_bullet
	}

	pub fn get_player_bullet_mut(&mut self) -> &mut PlayerBullet {
		&mut self.player_bullet
	}

	pub fn get_for_player_bullet_abilities(&mut self) -> (&mut PlayerBullet, &Swarm) {
		(&mut self.player_bullet,&self.swarm)
	}

	pub fn get_player(&self) -> &Player {
		&self.player
	}

	pub fn get_player_mut(&mut self) -> &mut Player {
		&mut self.player
	}

	pub fn get_swarm(&self) -> &Swarm {
		&self.swarm
	}

	pub fn get_swarm_mut(&mut self) -> &mut Swarm {
		&mut self.swarm
	}

	pub fn get_ufo(&self) -> &Ufo {
		&self.ufo
	}

	pub fn get_ufo_mut(&mut self) -> &mut Ufo {
		&mut self.ufo
	}
	pub fn get_for_collisions(&mut self) -> (&mut Player, &mut Swarm, &mut PlayerBullet, &mut Vec<Bullet>, &mut Vec<Shield>, &mut Ufo) {
		let shields = match self.using_fastly_shields {
			true => &mut self.fastly_shields,
			false => &mut self.shields
		};
		(&mut self.player, &mut self.swarm, &mut self.player_bullet, &mut self.bullets, shields, &mut self.ufo)
	}


}
