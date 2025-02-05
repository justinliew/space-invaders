use std::os::raw::c_int;
use std::sync::mpsc::Sender;

use crate::size::WorldSize;
use crate::input::Input;
use crate::world::World;
use crate::point::Point;

const MOVE_SPEED: f64 = 200.0;

extern "C" {

	fn check_high_score();

	fn wait_high_score() -> c_int;

	fn wait_outro_complete() -> c_int;

	fn new_session();

	fn handle_game_over();

	fn update_local_score(_: c_int);

//	fn console_log_int(_: c_int);
}

#[derive(PartialEq,Clone,Copy)]
pub enum ResetType {
	New,
	Next,
	Respawn,
}

#[derive(Clone,Copy,PartialEq)]
pub enum GameState {
	Intro(f64),
	Playing,
	Death(f64),
	_Win(f64),
	GameOverFastlyTreatment(f64),
	CheckHighScore,
	WaitHighScore,
	ShowHighScore(bool,bool,bool),
	GameOver(f64),
}

pub enum GameEvent {
	ScoreChanged(i32),
	EntityDied(Point, ColourIndex),
}

// TODO - I don't know if this should live here
#[derive(Clone,Copy)]
pub enum ColourIndex {
	WHITE,
	BLUE,
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

	/// name picker
	pub letter_index: i32,
	pub cur_letter: i32,

	/// Events to other parts of the system
	sender: Sender<GameEvent>,
}

impl Game {
    /// Returns a new `Game` containing a `World` of the given `Size`
    pub fn new(world_size: WorldSize, tx: Sender<GameEvent>) -> Game {
        Game {
            world: World::new(world_size),
            score: 0,
			lives: 1,
			wave: 1,
			game_state: GameState::Intro(0.5),
			letter_index: 0,
			cur_letter: 0,
			sender: tx,
        }
    }

    /// Reset our game-state
    pub fn reset(&mut self, reset_type: ResetType) {

		if reset_type == ResetType::New {
			self.score = 0;
			self.lives = 1;
			self.wave = 1;
		}
		if reset_type == ResetType::Next {
			self.wave += 1;
			self.score += 700;
			unsafe {update_local_score(self.score)};
		}

		self.letter_index = 0;
		self.cur_letter = 0;

        // Remove all enemies and bullets
		self.world.reset(reset_type);
    }

	pub fn post_death_reset(&mut self) {
		self.world.reset(ResetType::Respawn);
	}

	pub fn send_game_event(&mut self, event: GameEvent) {
		self.sender.send(event).expect("Wasn't able to send event");
	}

	pub unsafe fn update(&mut self, input: &Input, dt: f64) {
		match self.game_state {
			GameState::Intro(ref mut timer) => {
				if *timer >= 0. {
					*timer -= dt;
				} else {
					if input.any {
						new_session();
						self.game_state = GameState::Playing;
					}
				}
			},
			GameState::Playing => {
				let radius = self.world.get_swarm().radius;
				let player_location = self.world.get_player().vector.clone();

				if input.left && self.world.get_player().x() > radius {
					*self.world.get_player_mut().x_mut() -= MOVE_SPEED * dt;
				}
				if input.right && self.world.get_player().x() < (self.world.world_size.width-radius-75.) {
					*self.world.get_player_mut().x_mut() += MOVE_SPEED * dt;
				};

				if input.up && self.world.get_player().y() > 0. {
					*self.world.get_player_mut().y_mut() -= MOVE_SPEED * dt;
				}
				if input.down && self.world.get_player().y() < (self.world.world_size.height-radius-75.) {
					*self.world.get_player_mut().y_mut() += MOVE_SPEED * dt;
				};

				// Add bullets
				if input.fire {
					if !self.world.get_player_bullet().active {
						self.world.get_player_bullet_mut().respawn(player_location, 800.);
					}
				}

				// if input.alt {
				// 	self.activate_condition(Condition::HeatSeeking);
				// }

				// update enemies
				if let Some(bullets) = self.world.get_swarm_mut().update(dt) {
					for bullet in bullets {
						self.world.get_bullets_mut().push(bullet);
					}
				}

				// update bullets
				for bullet in self.world.get_bullets_mut() {
					bullet.update(dt);
				}

				if self.world.get_player_bullet().active {
					let bullet = self.world.get_for_player_bullet_abilities();
					bullet.update(dt);
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
						player_bullet.despawn();
					}
				}

				self.handle_collisions();
				// TODO Winning game state

				if !self.world.get_player().alive {
					self.lives -= 1;
					if self.lives == 0 {
						self.game_state = GameState::GameOverFastlyTreatment(3.);
					} else {
						self.game_state = GameState::Death(3.);
					}
				}
			},
			GameState::Death(ref mut timer) => {
				*timer -= dt;
				if *timer < 0. {
					self.post_death_reset();
					self.game_state = GameState::Playing;
				}
			},
			GameState::_Win(ref mut timer) => {
				if *timer >= 0. {
					*timer -= dt;
				} else {
					self.reset(ResetType::Next);
					self.game_state = GameState::Playing;
				}
			},
			// freeze the game and kill all bots
			GameState::GameOverFastlyTreatment(ref mut timer) => {
				if *timer >= 0. {
					*timer -= dt;
					self.world.get_swarm_mut().force_kill(1.);
					// for event in queued_events {
					// 	self.send_game_event(event);
					// }
				} else {
					self.game_state = GameState::GameOver(1.);
				}
			},
			GameState::CheckHighScore => {
				check_high_score();
				self.game_state = GameState::WaitHighScore;
			},
			GameState::WaitHighScore => {
				let ret = wait_high_score();
				match ret {
					1 => {
						self.reset(ResetType::New);
						self.game_state = GameState::Intro(0.5);
					},
					2 => self.game_state = GameState::ShowHighScore(false,false,false),
					_ => {},
				}
			},
			GameState::ShowHighScore(ref mut fire,ref mut left,ref mut right) => {
				let mut advance = false;
				if !input.fire && *fire {
					if self.letter_index == 2 {
						advance = true;
					} else {
						self.letter_index += 1;
						self.cur_letter = 0;
					}
				}
				if !input.left && *left {
					self.cur_letter -= 1;
					if self.cur_letter < 0 {
						self.cur_letter = 25;
					}
				}
				if !input.right && *right {
					self.cur_letter += 1;
					if self.cur_letter > 25 {
						self.cur_letter = 0;
					}
				}
				*fire = input.fire;
				*left = input.left;
				*right = input.right;
				if advance {
					handle_game_over();
					self.reset(ResetType::New);
					self.game_state = GameState::Intro(0.5);
			}
			},
			GameState::GameOver(ref mut timer) => {
				if *timer >= 0. {
					*timer -= dt;
				} else {
					let ret = wait_outro_complete();
					if ret == 1 {
						self.game_state = GameState::CheckHighScore;
					}
				}
			},			
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