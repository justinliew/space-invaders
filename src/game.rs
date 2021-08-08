use std::os::raw::{c_int};
use std::sync::mpsc;
use std::sync::mpsc::{Sender,Receiver};

use crate::size::WorldSize;
use crate::bullet::{BulletType};
use crate::input::Input;
use crate::world::World;
use crate::point::Point;

const MOVE_SPEED: f64 = 200.0;

extern "C" {
	fn init_shield(_: c_int);
	fn add_shield_state(_: c_int, _: c_int, _: c_int);

	// id, index, state
	fn update_shield(_: c_int, _: c_int, _: c_int);
}

#[derive(PartialEq)]
pub enum ResetType {
	New,
	Next,
}

#[derive(Clone,Copy)]
pub enum GameState {
	Intro(f64),
	Playing,
	Death(f64),
	Win(f64),
	GameOver(f64),
}

pub enum GameEvent {
	ScoreChanged(i32),
	EntityDied(Point, ColourIndex)
}

// TODO - I don't know if this should live here
#[derive(Clone,Copy)]
pub enum ColourIndex {
	WHITE,
	BLUE,
	RED,
}

/// The data structure that contains the state of the game
pub struct Game {
    /// The world contains everything that needs to be drawn
    pub world: World,
    /// The current score of the player
    pub score: i32,
	/// Number of lives
	pub lives: i32,
	/// Number of waves
	pub wave: i32,
	/// state of the game
	pub game_state: GameState,

	/// Events to other parts of the system
	sender: Sender<GameEvent>,
}

impl Game {
    /// Returns a new `Game` containing a `World` of the given `Size`
    pub fn new(world_size: WorldSize, tx: Sender<GameEvent>) -> Game {
        Game {
            world: World::new(world_size),
            score: 0,
			lives: 3,
			wave: 1,
			game_state: GameState::Intro(0.5),
			sender: tx,
        }
    }

    /// Reset our game-state
    pub fn reset(&mut self, reset_type: ResetType) {

		if reset_type == ResetType::New {
			self.score = 0;
			self.lives = 3;
			self.wave = 0;
		}
		if reset_type == ResetType::Next {
			self.wave += 1;
		}

        // Remove all enemies and bullets
        self.world.bullets.clear();
		self.world.swarm.reset(reset_type);
		self.world.player.alive = true;
    }

	pub fn post_death_reset(&mut self) {
        self.world.bullets.clear();
		self.world.player.alive = true;
		self.world.player.reset_location();
	}

	pub fn send_game_event(&mut self, event: GameEvent) {
		self.sender.send(event).expect("Wasn't able to send event");
	}

	pub unsafe fn update(&mut self, input: &Input, dt: f64) {
		match self.game_state {
			GameState::Intro(_) => {
				if let GameState::Intro(ref mut timer) = self.game_state {
					if *timer >= 0. {
						*timer -= dt;
					} else {
						for (index,shield) in self.world.shields.iter_mut().enumerate() {
							shield.reset();
							init_shield(index as i32);
							for i in 0..25 {
								let state = &shield.b[i];
								add_shield_state(index as i32, i as i32, *state as i32);
							}
						}
						if input.any {
							self.game_state = GameState::Playing;
						}
					}
				}
			},
			GameState::Playing => {
				let radius = self.world.swarm.radius;

				if input.left && self.world.player.x() > radius {
					*self.world.player.x_mut() -= MOVE_SPEED * dt;
				}
				if input.right && self.world.player.x() < (self.world.world_size.width-radius) {
					*self.world.player.x_mut() += MOVE_SPEED * dt;
				};

				// Add bullets
				if input.fire {
					if let BulletType::Player(alive) = self.world.player_bullet.bullet_type {
						if !alive {
							self.world.player_bullet.inplace_new(self.world.player.vector.clone(), BulletType::Player(true), 600.);
						}
					}
				}

				// update enemies
				if let Some(bullet) = self.world.swarm.update(dt) {
					self.world.bullets.push(bullet);
				}

				// update bullets
				for bullet in &mut self.world.bullets {
					bullet.update(dt);
				}

				if let BulletType::Player(alive) = self.world.player_bullet.bullet_type {
					if alive {
						self.world.player_bullet.update(dt);
					}
				}

				// Remove bullets outside the viewport
				{
					let width = self.world.world_size.width;
					let height = self.world.world_size.height;
					let bullets = &mut self.world.bullets;
					bullets.retain(|bullet| {
						let within = bullet.x() > 0. && bullet.x() < width &&
								bullet.y() > 0. && bullet.y() < height;
						within
					});

					if self.world.player_bullet.x() < 0. || self.world.player_bullet.x() > width ||
					self.world.player_bullet.y() < 0. || self.world.player_bullet.y() > height {
						self.world.player_bullet.bullet_type = BulletType::Player(false);
					}
				}

				let deferred_shield_damage = self.handle_collisions();
				{
					let mut_shields = &mut self.world.shields;
					for d in deferred_shield_damage {
						let bs = mut_shields[d.0].damage(d.1,d.2);
						update_shield(d.0 as i32, (d.1 as f64 + d.2 as f64 * 5.) as i32, bs as i32);
					}
				}
				if let Some(lowest) = self.world.swarm.get_lowest_alive() {
					if lowest >= self.world.player.vector.position.y {
						self.game_state = GameState::GameOver(2.);
					}
				} else {
					self.game_state = GameState::Win(2.);
				}

				if !self.world.player.alive {
					self.lives -= 1;
					if self.lives == 0 {
						self.game_state = GameState::GameOver(2.);
					} else {
						self.game_state = GameState::Death(1.);
					}
				}

				self.world.ufo.update(dt);
			},
			GameState::Death(_) => {
				if let GameState::Death(ref mut timer) = self.game_state {
					*timer -= dt;
					if *timer < 0. {
						self.post_death_reset();
						self.game_state = GameState::Playing;
					}
				}
			},
			GameState::Win(_) => {
				if let GameState::Win(ref mut timer) = self.game_state {
					if *timer >= 0. {
						*timer -= dt;
					} else {
						self.reset(ResetType::Next);
						self.game_state = GameState::Playing;
					}
				}
			},
			GameState::GameOver(_) => {
				if let GameState::GameOver(ref mut timer) = self.game_state {
					if *timer >= 0. {
						*timer -= dt;
					} else {
						if input.any {
							self.reset(ResetType::New);
							self.game_state = GameState::Intro(2.);
						}
					}
				}
			}
		}
	}
}

pub struct GameData {
	pub game: Game,
	pub input: Input,
}

impl GameData {
	pub fn new(world_size: WorldSize, tx: Sender<GameEvent>) -> GameData {
		let game = Game::new(world_size, tx);
		GameData {
			game: game,
			input: Input::default(),
		}
	}
}