use std::os::raw::c_int;
use std::sync::mpsc::Sender;

use crate::size::WorldSize;
use crate::bullet::Ability;
use crate::input::Input;
use crate::world::World;
use crate::point::Point;

const MOVE_SPEED: f64 = 200.0;

extern "C" {

	// id, index, state
	fn update_shield(_: c_int, _: c_int, _: c_int);

	fn check_high_score(_: c_int);

	fn wait_high_score() -> c_int;

	fn new_session();

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
	Win(f64),
	GameOverFastlyTreatment(f64),
	CheckHighScore,
	WaitHighScore,
	ShowHighScore(bool,bool,bool),
	GameOver(f64),
}

pub enum GameEvent {
	ScoreChanged(i32),
	EntityDied(Point, ColourIndex),
	Condition(Condition),
}

// TODO - I don't know if this should live here
#[derive(Clone,Copy)]
pub enum ColourIndex {
	WHITE,
	BLUE,
	RED,
	GREEN,
}

#[derive(Clone,Copy,PartialEq)]
#[repr(u8)]
pub enum Condition {
	None=0,
	Shields=1,
	Bomb=2,
	HeatSeeking=3,
	Wave=4,
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

	/// debug state of the game
	pub current_condition: Condition,
	pub conditions: Vec<Condition>,

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
			current_condition: Condition::None,
			conditions: vec![],
			letter_index: 0,
			cur_letter: 0,
			sender: tx,
        }
    }

	pub fn has_had_condition(&self, condition: Condition) -> bool {
		self.conditions.iter().find(|v| *v == &condition).is_some()
	}

	fn activate_condition(&mut self, condition: Condition) {
		self.current_condition = condition;
		self.conditions.push(condition);
		self.send_game_event(GameEvent::Condition(condition));	
	}

	pub fn update_conditions(&mut self) {
		// conditions that happen in priority order
		let total = self.world.get_active_shields().iter().fold(0., |acc, s| acc + s.get_percentage_full());
		let avg = total / self.world.get_active_shields().len() as f32;
		let lowest = self.world.get_swarm().get_lowest_alive();
		if lowest.is_none() {
			return;
		}
		let lowest = lowest.expect("alive");

		if avg < 0.5 && !self.has_had_condition(Condition::Shields) {
			self.world.enable_fastly_shields();
			self.activate_condition(Condition::Shields);
		} else if lowest.y >= 400. && !self.has_had_condition(Condition::Bomb) {
			self.activate_condition(Condition::Bomb);
			self.world.get_player_bullet_mut().ability = Ability::Bomb;
		} else if self.world.get_swarm().get_percentage_alive() < 0.3 && !self.has_had_condition(Condition::HeatSeeking) {
			self.activate_condition(Condition::HeatSeeking);
			self.world.get_player_bullet_mut().ability = Ability::Heat;
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
		}

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
					self.world.init_shields();
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
				if input.right && self.world.get_player().x() < (self.world.world_size.width-radius) {
					*self.world.get_player_mut().x_mut() += MOVE_SPEED * dt;
				};

				// Add bullets
				if input.fire {
					if !self.world.get_player_bullet().active {
						let bomb = self.conditions.iter().find(|v| *v == &Condition::Bomb).is_some();
						let heat = self.conditions.iter().find(|v| *v == &Condition::HeatSeeking).is_some();
						let (ability,speed) = if heat {
							(Ability::Heat,1000.)
						} else if bomb {
							(Ability::Bomb,400.)
						} else {
							(Ability::None,800.)
						};
						self.world.get_player_bullet_mut().respawn(player_location, speed, ability);
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
					let (bullet,swarm) = self.world.get_for_player_bullet_abilities();
					bullet.update(dt, swarm);
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

				let is_red = self.has_had_condition(Condition::Bomb) || self.has_had_condition(Condition::HeatSeeking);
				let deferred_shield_damage = self.handle_collisions(is_red);
				{
					for d in deferred_shield_damage {
						let bs = self.world.get_active_shields_mut()[d.0].damage(d.1,d.2);
						update_shield(d.0 as i32, (d.1 as f64 + d.2 as f64 * 5.) as i32, bs as i32);
					}
				}
				if let Some(lowest) = self.world.get_swarm().get_lowest_alive() {
					if lowest.y >= self.world.get_active_shields()[0].top_left.y {
						self.game_state = GameState::GameOverFastlyTreatment(3.);
					}
				} else {
					self.game_state = GameState::Win(2.);
				}

				if !self.world.get_player().alive {
					self.lives -= 1;
					if self.lives == 0 {
						self.game_state = GameState::GameOverFastlyTreatment(3.);
					} else {
						self.game_state = GameState::Death(3.);
					}
				}

				self.world.get_ufo_mut().update(dt);

//				self.update_conditions();
			},
			GameState::Death(ref mut timer) => {
				*timer -= dt;
				if *timer < 0. {
					self.post_death_reset();
					self.game_state = GameState::Playing;
				}
			},
			GameState::Win(ref mut timer) => {
				if *timer >= 0. {
					*timer -= dt;
				} else {
					self.reset(ResetType::Next);
					self.world.init_shields();
					self.game_state = GameState::Playing;
					self.activate_condition(Condition::Wave);
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
					self.game_state = GameState::CheckHighScore;
				}
			},
			GameState::CheckHighScore => {
				check_high_score(self.score);
				self.game_state = GameState::WaitHighScore;
			},
			GameState::WaitHighScore => {
				let ret = wait_high_score();
				match ret {
					1 => self.game_state = GameState::GameOver(1.),
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
					if (self.cur_letter < 0) {
						self.cur_letter = 25;
					}
				}
				if !input.right && *right {
					self.cur_letter += 1;
					if (self.cur_letter > 25) {
						self.cur_letter = 0;
					}
				}
				*fire = input.fire;
				*left = input.left;
				*right = input.right;
				if advance {
					self.game_state = GameState::GameOver(1.);
				}
			},
			GameState::GameOver(ref mut timer) => {
				if *timer >= 0. {
					*timer -= dt;
				} else {
					if input.any {
						self.reset(ResetType::New);
						self.game_state = GameState::Intro(2.);
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