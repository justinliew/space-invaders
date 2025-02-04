use crate::{game::{Game,GameEvent, ColourIndex}};

impl Game {

	pub unsafe fn handle_collisions(&mut self) {
		let world = &mut self.world;
		let (player,swarm,player_bullet, bullets) = world.get_for_collisions();

		// TODO move this to a game event?
		let mut queued_events = vec![];

		bullets.retain(|bullet| {

			let _playerhit = player.check_hit(bullet);
			true
		});

		if player_bullet.active {
			let swarmhit = swarm.check_hit(player_bullet);
			if let Some(hits) = swarmhit {
				for hit in hits {
					let points = hit.0;
					let loc = hit.1;
					queued_events.push(GameEvent::EntityDied(loc, ColourIndex::WHITE));
					self.score += points as i32;
					queued_events.push(GameEvent::ScoreChanged(self.score));
					player_bullet.active = false;
				}
			}
		}

		for e in queued_events {
			self.send_game_event(e);
		}
	}
}