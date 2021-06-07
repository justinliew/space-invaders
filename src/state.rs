// use pcg_rand::Pcg32Basic;
// use rand::SeedableRng;

// use geometry::{Position, Size};
// use models::World;

// use rand::Rng;

use crate::player::{Player};
use crate::swarm::Swarm;
use crate::size::Size;
use crate::bullet::Bullet;

/// A model that contains the other models and renders them
pub struct World {
    pub player: Player,
	pub swarm: Swarm,
	pub world_size: Size,
	pub bullets: Vec<Bullet>,
    // pub particles: Vec<Particle>,
    // pub bullets: Vec<Bullet>,
    // pub enemies: Vec<Enemy>,
    // pub size: Size
}

impl World {
    /// Returns a new world of the given size
    pub fn new(world_size: Size) -> World {
        World {
            player: Player::new(),
			swarm: Swarm::new(10,6, world_size),
			world_size: world_size,
			bullets: vec![],
            // particles: Vec::with_capacity(1000),
            // bullets: vec![],
            // enemies: vec![],
            // size: size
        }
    }
}

/// The data structure that contains the state of the game
pub struct State {
    /// The world contains everything that needs to be drawn
    pub world: World,
    /// The current score of the player
    pub score: u32,
}

impl State {
    /// Returns a new `State` containing a `World` of the given `Size`
    pub fn new(world_size: Size) -> State {
        State {
            world: World::new(world_size),
            score: 0,
        }
    }

    /// Reset our game-state
    pub fn reset(&mut self) {
//        let mut rng = Pcg32Basic::from_seed([42, 42]);

        // Reset player position
        // *self.world.player.x_mut() = self.world.size.random_x(&mut rng);
        // *self.world.player.y_mut() = self.world.size.random_y(&mut rng);

        // Reset score
        self.score = 0;

        // Remove all enemies and bullets
        // self.world.bullets.clear();
        // self.world.enemies.clear();
    }
}