use crate::{game::{Condition, Game,GameEvent, ColourIndex}, bullet::Ability};

type DeferredShieldDamage = (usize,usize,usize);

impl Game {

	pub unsafe fn handle_collisions(&mut self, is_red: bool) -> Vec<DeferredShieldDamage> {
		let world = &mut self.world;
		let (player,swarm,player_bullet, bullets,shields,ufo) = world.get_for_collisions();

		// TODO move this to a game event?
		let mut deferred_shield_damage = vec![];
		let mut queued_events = vec![];

		bullets.retain(|bullet| {

			// shields first
			for (index,shield) in shields.iter().enumerate() {
				match shield.check_hit(&bullet.location) {
					Some((i,j)) => {
						deferred_shield_damage.push((index,i,j));
						return false;
					},
					None => {},
				}
			}

			let playerhit = player.check_hit(bullet);
			if playerhit {
				// TODO this should be green if we have certain conditions
				if is_red {
					queued_events.push(GameEvent::EntityDied(bullet.location.position, ColourIndex::RED));
				} else {
					queued_events.push(GameEvent::EntityDied(bullet.location.position, ColourIndex::GREEN));
				}
			}
			!playerhit
		});

		if player_bullet.active {
			// shields first
			if player_bullet.ability == Ability::None {
				for (index,shield) in shields.iter().enumerate() {
					match shield.check_hit(&player_bullet.location) {
						Some((i,j)) => {
							deferred_shield_damage.push((index,i,j));
							player_bullet.active = false;
							break;
						},
						None => {},
					}
				}
			}

			let swarmhit = swarm.check_hit(player_bullet);
			if let Some(hits) = swarmhit {
				for hit in hits {
					let points = hit.0;
					let loc = hit.1;
					queued_events.push(GameEvent::EntityDied(loc, ColourIndex::WHITE));
					self.score += points as i32;
					queued_events.push(GameEvent::ScoreChanged(self.score));
					if player_bullet.ability != Ability::Bomb {
						player_bullet.active = false;
					}
				}
			}

			let ufohit = ufo.check_hit(player_bullet);
			if let Some(hit) = ufohit {
				let points = hit.0;
				let loc = hit.1;
				queued_events.push(GameEvent::EntityDied(loc, ColourIndex::BLUE));
				self.score += points as i32;
				queued_events.push(GameEvent::ScoreChanged(self.score));
				player_bullet.active = false;
			}
		}

		for e in queued_events {
			self.send_game_event(e);
		}

		deferred_shield_damage
	}
}