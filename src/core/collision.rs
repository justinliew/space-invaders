// TODO these aren't core game functions
use crate::leaderboard::{prep_leaderboard_entries};

use crate::bullet::BulletType;
use crate::game::{Game,GameEvent, ColourIndex};

type DeferredShieldDamage = (usize,usize,usize);

impl Game {

	pub unsafe fn handle_collisions(&mut self) -> Vec<DeferredShieldDamage> {
		let world = &mut self.world;
		let player = &mut world.player;
		let swarm = &mut world.swarm;
		let bullets = &mut world.bullets;
	//	let particles = &mut world.particles;
		let shields = &world.shields;
		let ufo = &mut world.ufo;

		// TODO move this to a game event?
		let mut deferred_shield_damage = vec![];
		let mut queued_events = vec![];

		bullets.retain(|bullet| {

			// shields first
			for (index,shield) in shields.iter().enumerate() {
				match shield.check_hit(bullet) {
					Some((i,j)) => {
						deferred_shield_damage.push((index,i,j));
						return false;
					},
					None => {},
				}
			}

			let playerhit = player.check_hit(bullet);
			if playerhit {
				queued_events.push(GameEvent::EntityDied(bullet.location.position, ColourIndex::RED));
			}
			!playerhit
		});

		if let BulletType::Player(alive) = world.player_bullet.bullet_type {
			if alive {
				// shields first
				for (index,shield) in shields.iter().enumerate() {
					match shield.check_hit(&world.player_bullet) {
						Some((i,j)) => {
							deferred_shield_damage.push((index,i,j));
							world.player_bullet.bullet_type = BulletType::Player(false);
							break;
						},
						None => {},
					}
				}

				let swarmhit = swarm.check_hit(&world.player_bullet);
				if let Some(hit) = swarmhit {
					let points = hit.0;
					let loc = hit.1;
					queued_events.push(GameEvent::EntityDied(loc, ColourIndex::WHITE));
					self.score += points as i32;
					queued_events.push(GameEvent::ScoreChanged(self.score));
					world.player_bullet.bullet_type = BulletType::Player(false);
				}

				let ufohit = ufo.check_hit(&world.player_bullet);
				if let Some(hit) = ufohit {
					let points = hit.0;
					let loc = hit.1;
					queued_events.push(GameEvent::EntityDied(loc, ColourIndex::BLUE));
					self.score += points as i32;
					queued_events.push(GameEvent::ScoreChanged(self.score));
					world.player_bullet.bullet_type = BulletType::Player(false);
				}
			}
		}

		for e in queued_events {
			self.send_game_event(e);
		}

		deferred_shield_damage
	}
}