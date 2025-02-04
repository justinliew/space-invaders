use crate::game::ResetType;
use crate::player::Player;
use crate::swarm::Swarm;
use crate::size::WorldSize;
use crate::bullet::{Bullet,PlayerBullet};

pub struct World {
	pub world_size: WorldSize,
    player: Player,
	swarm: Swarm,
	player_bullet: PlayerBullet,
	bullets: Vec<Bullet>,
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
        }
    }

	pub fn reset(&mut self, reset_type: ResetType) {
        self.bullets.clear();
		self.swarm.reset(reset_type);
		self.player.alive = true;
		self.player.reset_location();
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

	pub fn get_for_player_bullet_abilities(&mut self) -> &mut PlayerBullet {
		&mut self.player_bullet
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

	pub fn get_for_collisions(&mut self) -> (&mut Player, &mut Swarm, &mut PlayerBullet, &mut Vec<Bullet>) {
		(&mut self.player, &mut self.swarm, &mut self.player_bullet, &mut self.bullets)
	}


}
