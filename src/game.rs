use std::os::raw::{c_int};
use std::sync::mpsc::{Sender};

use crate::size::WorldSize;
use crate::bullet::{BulletType};
use crate::input::Input;
use crate::world::World;
use crate::point::Point;

const MOVE_SPEED: f64 = 200.0;

extern "C" {

	// id, index, state
	fn update_shield(_: c_int, _: c_int, _: c_int);

	fn new_session();
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
			self.wave = 1;
		}
		if reset_type == ResetType::Next {
			self.wave += 1;
		}

        // Remove all enemies and bullets
		self.world.reset(reset_type);
    }

	pub fn post_death_reset(&mut self) {
		self.world.reset(ResetType::New);
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
						self.world.init_shields();
						if input.any {
							new_session();
							self.game_state = GameState::Playing;
						}
					}
				}
			},
			GameState::Playing => {
				let radius = self.world.get_swarm().radius;
				let player_location = self.world.get_player().vector.clone();

				if input.left && self.world.get_player().x() > radius {
					*self.world.get_player_mut().x_mut() -= MOVE_SPEED * dt;
				}
				if input.right && self.world.get_player().x() < (self.world.world_size.width-radius) {
					*self.world.get_player_mut().x_mut() += MOVE_SPEED * dt;
				};

				// Add bullets
				if input.fire {
					if let BulletType::Player(alive) = self.world.get_player_bullet().bullet_type {
						if !alive {
							self.world.get_player_bullet_mut().inplace_new(player_location, BulletType::Player(true), 600.);
						}
					}
				}

				// update enemies
				if let Some(bullet) = self.world.get_swarm_mut().update(dt) {
					self.world.get_bullets_mut().push(bullet);
				}

				// update bullets
				for bullet in self.world.get_bullets_mut() {
					bullet.update(dt);
				}

				if let BulletType::Player(alive) = self.world.get_player_bullet().bullet_type {
					if alive {
						self.world.get_player_bullet_mut().update(dt);
					}
				}

				// Remove bullets outside the viewport
				{
					let width = self.world.world_size.width;
					let height = self.world.world_size.height;
					let bullets = self.world.get_bullets_mut();
					bullets.retain(|bullet| {
						let within = bullet.x() > 0. && bullet.x() < width &&
								bullet.y() > 0. && bullet.y() < height;
						within
					});

					let player_bullet = self.world.get_player_bullet_mut();
					if player_bullet.x() < 0. || player_bullet.x() > width ||
					player_bullet.y() < 0. || player_bullet.y() > height {
						player_bullet.bullet_type = BulletType::Player(false);
					}
				}

				let deferred_shield_damage = self.handle_collisions();
				{
					for d in deferred_shield_damage {
						let bs = self.world.get_active_shields_mut()[d.0].damage(d.1,d.2);
						update_shield(d.0 as i32, (d.1 as f64 + d.2 as f64 * 5.) as i32, bs as i32);
					}
				}
				if let Some(lowest) = self.world.get_swarm().get_lowest_alive() {
					if lowest >= self.world.get_player().vector.position.y {
						self.game_state = GameState::GameOver(2.);
					}
				} else {
					self.game_state = GameState::Win(2.);
				}

				if !self.world.get_player().alive {
					self.lives -= 1;
					if self.lives == 0 {
						self.game_state = GameState::GameOver(2.);
					} else {
						self.game_state = GameState::Death(1.);
					}
				}

				self.world.get_ufo_mut().update(dt);
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