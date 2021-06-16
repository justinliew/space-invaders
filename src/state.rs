// use pcg_rand::Pcg32Basic;
// use rand::SeedableRng;

// use geometry::{Position, Size};
// use models::World;

// use rand::Rng;

use crate::player::{Player};
use crate::swarm::Swarm;
use crate::size::Size;
use crate::bullet::Bullet;
use crate::point::Point;
use crate::input::Input;
use crate::particle::Particle;
use crate::shield::{BlockState,Shield};



/// A model that contains the other models and renders them
pub struct World {
    pub player: Player,
	pub swarm: Swarm,
	pub world_size: Size,
	pub bullets: Vec<Bullet>,
    pub particles: Vec<Particle>,
	pub shields: Vec<Shield>,
    // pub size: Size
}

impl World {
    /// Returns a new world of the given size
    pub fn new(world_size: Size) -> World {
        World {
            player: Player::new(),
			swarm: Swarm::new(10,5, world_size),
			world_size: world_size,
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
            // size: size
        }
    }
}

pub enum GameState {
	Intro,
	Playing,
	Death,
	GameOver,
}

/// The data structure that contains the state of the game
pub struct State {
    /// The world contains everything that needs to be drawn
    pub world: World,
    /// The current score of the player
    pub score: i32,
	/// Number of lives
	pub lives: i32,
	/// state of the game
	pub game_state: GameState,
}

impl State {
    /// Returns a new `State` containing a `World` of the given `Size`
    pub fn new(world_size: Size) -> State {
        State {
            world: World::new(world_size),
            score: 0,
			lives: 3,
			game_state: GameState::Intro,
        }
    }

    /// Reset our game-state
    pub fn reset(&mut self) {

        self.score = 0;
		self.lives = 3;

        // Remove all enemies and bullets
        self.world.bullets.clear();
		self.world.swarm.reset();
		self.world.player.alive = true;
    }

	pub fn post_death_reset(&mut self) {
        self.world.bullets.clear();
		self.world.player.alive = true;
		self.world.player.reset_location();

	}

	pub fn update(&mut self) {
		if let Some(lowest) = self.world.swarm.get_lowest_alive() {
			if lowest >= self.world.player.vector.position.y {
				self.game_state = GameState::Death;
			}
		} else {
			// if there are no enemies then we win
		}

		if !self.world.player.alive {
			self.lives -= 1;
			if self.lives == 0 {
				self.game_state = GameState::GameOver;
			} else {
				self.game_state = GameState::Death;
			}
		}
	}
}

pub struct GameData {
	pub state: State,
	pub input: Input,
	pub current_time: f64,
//	screen_size: Size,
	pub screen_top_left_offset: Point,
	pub game_to_screen: f64,
	pub width: usize,
	pub height: usize,
    // state: GameState,
    // actions: Actions,
    // time_controller: TimeController<Pcg32Basic>
}

impl GameData {
	pub fn new(world_size: Size) -> GameData {
		GameData {
			state: State::new(world_size),
			input: Input::default(),
			current_time: 0.0,
			screen_top_left_offset: Point::new(0.0,0.0),
			game_to_screen: 1.,
			width: 1024,
			height: 768,
			// actions: Actions::default(),
			// time_controller: TimeController::new(Pcg32Basic::from_seed([42, 42]))
		}
	}

	pub fn world_to_screen(&self, in_point: &Point) -> Point {
		Point{
			x: (in_point.x + self.screen_top_left_offset.x) * self.game_to_screen,
			y: (in_point.y + self.screen_top_left_offset.y) * self.game_to_screen,
		}
	}

}